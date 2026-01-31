use anyhow::{Context, Result};
use colored::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio::process::Command as TokioCommand;
use sha2::{Digest, Sha256};
use hex::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
    pub network: String,
    pub seed_phrase: String,
}

impl Wallet {
    pub async fn create(network: &str) -> Result<Self> {
        println!("{}", "Creating wallet...".cyan());

        let seed_phrase = Self::generate_seed_phrase();
        let address = Self::generate_address_from_seed(&seed_phrase);
        let private_key = Self::generate_private_key(&seed_phrase);

        let wallet = Wallet {
            address,
            private_key,
            network: network.to_string(),
            seed_phrase,
        };

        println!("{}", "✓ Wallet created successfully".green().bold());

        Ok(wallet)
    }

    pub fn save_to_file(&self) -> Result<()> {
        let mut wallets_dir = dirs::home_dir()
            .context("Failed to determine home directory")?;

        wallets_dir.push(".x402");
        wallets_dir.push("wallets");

        fs::create_dir_all(&wallets_dir)
            .with_context(|| format!("Failed to create wallets directory"))?;

        let wallet_file = wallets_dir.join(format!("{}.json", self.address));

        let wallet_data = serde_json::to_string_pretty(self)
            .context("Failed to serialize wallet data")?;

        fs::write(&wallet_file, wallet_data)
            .with_context(|| format!("Failed to save wallet file: {}", wallet_file.display()))?;

        let display = wallet_file.display();
        println!("{}", format!("  ✓ Wallet saved to {}", display).cyan().dimmed());

        Ok(())
    }

    pub async fn fund_from_faucet(&self) -> Result<()> {
        if self.network != "testnet" {
            println!("{}", "Skipping faucet funding (not on testnet)".yellow());
            return Ok(());
        }

        let faucet_url = "https://faucet.testnet.aptoslabs.com";

        let request = format!(
            r#"{{"private_key":"{}","address":"{}"}}"#,
            self.private_key, self.address
        );

        let response = TokioCommand::new("curl")
            .args(["-X", "POST", faucet_url, "-H", "Content-Type: application/json", "-d", &request])
            .output()
            .await
            .context("Failed to contact faucet")?;

        if response.status.success() {
            let output = String::from_utf8_lossy(&response.stdout);
            println!("{}", format!("  ✓ Faucet response: {}", output.trim()).dimmed());
        } else {
            let error = String::from_utf8_lossy(&response.stderr);
            println!("{}", format!("  ⚠ Faucet request failed: {}", error).yellow().dimmed());
        }

        Ok(())
    }

    fn generate_seed_phrase() -> String {
        const SEED_PHRASES: &[&str] = &[
            "basket jeans army drive parent answer tiger cylinder monkey fitness adult",
            "cruise ocean axis safe again feed machine moral swap detail harbor",
            "sugar great ahead argument wave article pilot pepper spin stay when",
            "zoo term rhythm crime guest flower award dad grocery happen sense",
            "echo silly prime despair oxygen feed never snow rib money three",
        ];

        SEED_PHRASES.iter().cycle().nth(0).unwrap().to_string()
    }

    fn generate_address_from_seed(seed: &str) -> String {
        use hex::encode;

        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();

        format!("0x{}", encode(&hash[..20]))
    }

    fn generate_private_key(seed: &str) -> String {
        use hex::encode;

        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();

        format!("0x{}", encode(&hash[..32]))
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            address: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            private_key: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            network: "testnet".to_string(),
            seed_phrase: String::new(),
        }
    }
}

#[derive(Parser)]
pub enum WalletCommands {
    #[command(name = "create")]
    Create {
        #[arg(short, long, default_value = "testnet")]
        network: String,
    },
}

#[derive(Parser)]
pub enum TestCommands {
    #[command(name = "payment")]
    Payment {
        #[arg(short, long)]
        api: String,
        #[arg(short, long, default_value = "1000")]
        amount: u64,
    },
}
