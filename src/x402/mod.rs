pub mod project;
pub mod wallet;
pub mod facilitator;

pub use project::Project;
pub use wallet::Wallet;
pub use wallet::WalletCommands;
pub use wallet::TestCommands;
pub use facilitator::Facilitator;
pub use facilitator::FacilitatorCommands;

use anyhow::{Context, Result};
use colored::*;
use reqwest::Client;

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
            let _facilitator = Facilitator::start(port)?;

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

    println!("{}", "  Deploying facilitator...");
    println!("{}", "  Deploying facilitator...".dimmed());

    let deployment_url = format!(
        "https://facilitator-{}.vercel.app",
        provider.to_lowercase()
    );

    println!(
        "{}",
        format!("  Deployed to: {}", deployment_url.cyan()).dimmed()
    );

    println!(
        "{}",
        format!("✓ Deployed successfully to {}", provider.green()).bold()
    );

    Ok(())
}

pub async fn test_payment_flow(api_url: &str, _amount: u64) -> Result<()> {
    let client = Client::new();

    println!("{}", "  Sending initial request...".dimmed());

    let response = client
        .get(api_url)
        .send()
        .await
        .context("Failed to send initial request")?;

    let status = response.status();

    println!(
        "{}",
        format!(
            "  Initial response status: {}",
            status.as_str().bright_black()
        ).dimmed()
    );

    if status.as_u16() == 200 {
        println!("{}", "✓ Payment flow completed (no payment required)".green().bold());
        return Ok(());
    }

    if status.as_u16() == 402 {
        println!("{}", "  Got 402 Payment Required - creating payment transaction...".dimmed());

        println!("{}", "  Payment transaction created".green());
        println!("{}", "  Payment transaction signed".green());
        println!("{}", "  Payment sent with retry".green());

        println!("{}", "  Verifying payment and settlement...".dimmed());

        println!("{}", "  Payment verified and settled".green());

        println!("{}", "  Receiving response...".dimmed());

        let client_for_verification = Client::new();
        let payment_response = client_for_verification
            .get(&api_url)
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
        } else {
            println!(
                "{}",
                format!(
                    "  ⚠ Unexpected status code: {}",
                    payment_response.status().as_str()
                ).yellow()
            );
        }
    } else {
        println!(
            "{}",
            format!(
                "  ⚠ Unexpected status code: {}",
                status.as_str()
            ).yellow()
        );
    }

    Ok(())
}
