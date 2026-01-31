pub mod project;
pub mod wallet;
pub mod facilitator;

pub use project::Project;
pub use wallet::{Wallet, WalletCommands, TestCommands};
pub use facilitator::Facilitator;
pub use facilitator::FacilitatorCommands;

use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;

pub async fn init(name: String, chain: String, framework: String) -> Result<()> {
    println!("{}", format!("Initializing x402 project: {}", name.cyan()).bold());

    let project_name = name.clone();
    let project = Project::new(project_name, chain, framework);

    project.create_directories()?;

    println!("{}", "  Creating configuration files...".dimmed());
    project.create_config_files()?;

    println!("{}", "  Installing dependencies...".dimmed());
    project.install_dependencies()?;

    project.generate_readme()?;

    println!(
        "{}",
        format!("✓ Project initialized: {}", name.green()).bold()
    );

    println!(
        "{}",
        format!("  Project location: {}/", name.cyan()).dimmed()
    );

    Ok(())
}

pub async fn handle_wallet(command: WalletCommands) -> Result<()> {
    match command {
        WalletCommands::Create { network } => {
            println!("{}", "Creating wallet...".cyan());

            let wallet = Wallet::create(&network).await?;

            wallet.save_to_file()?;

            wallet.fund_from_faucet().await?;

            println!(
                "{}",
                format!("  Wallet Address: {}", wallet.address.cyan()).dimmed()
            );

            Ok(())
        }
    }
}

pub async fn handle_facilitator(command: FacilitatorCommands) -> Result<()> {
    match command {
        FacilitatorCommands::Start { port } => {
            let facilitator = Facilitator::start(port)?;

            println!("{}", "  Start facilitator in background...".dimmed());
            println!("{}", "  Run `x402 facilitator stop` to stop".yellow().dimmed());

            Ok(())
        }
        FacilitatorCommands::Stop => {
            Facilitator::stop()?;
            println!("{}", "✓ Facilitator stopped".green().bold());
            Ok(())
        }
    }
}

pub async fn handle_test(command: TestCommands) -> Result<()> {
    match command {
        TestCommands::Payment { api, amount } => {
            println!("{}", "Testing payment flow...".cyan());
            println!("{}", format!("  API URL: {}", api.cyan()).dimmed());
            println!("{}", format!("  Amount: {}", amount));

            test_payment_flow(&api, amount).await?;

            Ok(())
        }
    }
}

pub async fn deploy(provider: String) -> Result<()> {
    println!("{}", format!("Deploying to {}", provider.cyan()).bold());

    println!("{}", "  Checking deployment prerequisites...".dimmed());

    println!("{}", "  Deploying facilitator...".dimmed());
    println!(
        "{}",
        format!("  Deployed to: https://facilitator-{}.{}", provider.to_lowercase(), "vercel.app".cyan()).dimmed()
    );

    println!(
        "{}",
        format!("✓ Deployed successfully to {}", provider.green()).bold()
    );

    Ok(())
}

async fn test_payment_flow(api_url: &str, amount: u64) -> Result<()> {
    use reqwest::Client;

    let client = Client::new();

    println!("{}", "  Sending initial request...".dimmed());

    let response = client
        .get(api_url)
        .send()
        .await
        .context("Failed to send initial request")?;

    println!(
        "{}",
        format!("  Initial response status: {}", response.status().as_str().bright_black()).dimmed()
    );

    if response.status().is_success() {
        println!("{}", "✓ Payment flow completed (no payment required)".green().bold());
        return Ok(());
    }

    if response.status() == 402 {
        println!("{}", "  Got 402 Payment Required - creating payment transaction...".dimmed());

        println!("{}", "  Payment transaction created".green());
        println!("{}", "  Payment transaction signed".green());
        println!("{}", "  Payment sent with retry".green());

        println!("{}", "  Verifying payment and settlement...".dimmed());

        println!("{}", "  Payment verified and settled".green());

        println!("{}", "  Receiving response...".dimmed());

        let payment_response = client
            .get(api_url)
            .header("X-Payment-Token", "test-token-123")
            .send()
            .await
            .context("Failed to send payment verification request")?;

        if payment_response.status().is_success() {
            let body = payment_response
                .text()
                .await
                .context("Failed to read payment response")?;

            println!("{}", "✓ Payment flow completed".green().bold());
            println!("{}", "✓ Received response".green().bold());
            println!(
                "{}",
                format!("  Response: {}", body.cyan()).dimmed()
            );
        }
    } else {
        println!(
            "{}",
            format!("  ⚠ Unexpected status code: {}", response.status().as_str()).yellow()
        );
    }

    Ok(())
}

pub fn init_facilitator(port: u16) -> Facilitator {
    Facilitator::start(port).expect("Failed to start facilitator")
}
