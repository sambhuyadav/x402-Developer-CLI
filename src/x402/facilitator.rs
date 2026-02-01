use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Facilitator {
    pub port: u16,
    pub wallet: crate::x402::wallet::Wallet,
    pub url: String,
    pub running: Arc<AtomicBool>,
}

impl Facilitator {
    pub fn start(port: u16) -> Result<Self> {
        println!("{}", "Starting facilitator...".cyan());

        let url = format!("http://localhost:{}", port);
        let running = Arc::new(AtomicBool::new(true));

        let facilitator = Facilitator {
            port,
            wallet: crate::x402::wallet::Wallet::default(),
            url: url.clone(),
            running,
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

        let output = std::process::Command::new("pkill")
            .args(["-f", "x402-cli"])
            .output()
            .context("Failed to execute pkill command")?;

        if output.status.success() {
            println!("{}", "✓ Facilitator stopped".green().bold());
            Ok(true)
        } else {
            println!("{}", "  ⚠ No facilitator processes found".yellow().dimmed());
            Ok(false)
        }
    }

    fn start_server(&self) -> Result<()> {
        let port = self.port;
        let url = self.url.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to bind to port {}: {}", port, e);
                    return;
                }
            };

            println!("{}", "  Facilitator ready to receive requests".dimmed());

            for stream in listener.incoming() {
                if !running.load(Ordering::Relaxed) {
                    break;
                }

                match stream {
                    Ok(stream) => {
                        if let Err(e) = Self::handle_connection(stream, &url) {
                            eprintln!("Error handling connection: {}", e);
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

    fn handle_connection(mut stream: TcpStream, url: &str) -> Result<()> {
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .context("Failed to set read timeout")?;

        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let request_line = request_line.trim();
        println!("{}", format!("  Request: {}", request_line).dimmed());

        let response = if request_line.contains("GET /health") {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                r#"{"status":"healthy","timestamp":"{timestamp}"}"#.replace(
                    "{timestamp}",
                    &chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
                )
            )
        } else if request_line.contains("POST") {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                format!(
                    r#"{{"message":"Payment facilitated","status":"success","url":"{}"}}"#,
                    url
                )
            )
        } else {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                format!(r#"{{"message":"Facilitator running","url":"{}"}}"#, url)
            )
        };

        stream.write_all(response.as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
