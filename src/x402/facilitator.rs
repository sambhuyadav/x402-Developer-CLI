use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct Facilitator {
    pub port: u16,
    pub wallet: crate::x402::wallet::Wallet,
    pub url: String,
    pub running: bool,
}

impl Facilitator {
    pub fn start(port: u16) -> Result<Self> {
        println!("{}", "Starting facilitator...".cyan());

        let url = format!("http://localhost:{}", port);

        let facilitator = Facilitator {
            port,
            wallet: crate::x402::wallet::Wallet::default(),
            url: url.clone(),
            running: true,
        };

        facilitator.start_server()?;

        println!(
            "{}",
            format!("✓ Facilitator server started on {}", url.cyan()).bold()
        );
        println!("{}", "  Waiting for wallet connections...".dimmed());

        Ok(facilitator)
    }

    pub fn stop() -> Result<bool> {
        println!("{}", "Stopping facilitator...".yellow());

        let mut cmd = Command::new("pkill")
            .args(["-f", "facilitator"])
            .spawn()
            .ok();

        if let Some(mut process) = cmd {
            if process.wait().is_ok() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn start_server(&self) -> Result<()> {
        let port = self.port;
        let url = self.url.clone();

        thread::spawn(move || {
            let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to bind to port {}: {}", port, e);
                    return;
                }
            };

            println!("{}", "  Facilitator ready to receive requests".dimmed());

            loop {
                match listener.accept() {
                    Ok((mut stream, addr)) => {
                        println!("{}", format!("  Connection from: {}", addr));

                        let mut buffer = [0u8; 4096];
                        match stream.read(&mut buffer) {
                            Ok(0) => {
                                println!("{}", "  Connection closed".dimmed());
                                break;
                            }
                            Ok(n) => {
                                let request = String::from_utf8_lossy(&buffer[..n]);
                                println!("{}", format!("  Request: {}", request.trim()));

                                let response = self.handle_request(&request);
                                stream.write_all(response.as_bytes()).ok();
                                stream.flush().ok();
                            }
                            Err(e) => {
                                eprintln!("Failed to read from connection: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                        break;
                    }
                }
            }
        });

        thread::sleep(Duration::from_millis(100));

        Ok(())
    }

    fn handle_request(&self, request: &str) -> String {
        if request.contains("health") {
            return format!(
                r#"{{"status":"healthy","url":"{}","wallet":"{}","timestamp":"{}"}}"#,
                self.url,
                self.wallet.address,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
            );
        }

        format!(
            r#"{{"message":"Facilitator running","url":"{}"}}"#,
            self.url
        )
    }
}

#[allow(dead_code)]
#[derive(Parser)]
pub enum FacilitatorCommands {
    #[command(name = "start")]
    Start {
        #[arg(short, long, default_value = "3001")]
        port: u16,
    },
    #[command(name = "stop")]
    Stop,
}
