pub mod deploy;
pub mod facilitator;
pub mod project;
pub mod test;
pub mod wallet;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

pub use facilitator::Facilitator;
pub use project::Project;
pub use wallet::Wallet;

#[derive(Parser)]
pub enum WalletCommands {
    #[command(name = "create")]
    Create {
        #[arg(short, long, default_value = "testnet")]
        network: String,
    },
    #[command(name = "import")]
    Import {
        #[arg(short, long)]
        private_key: String,
        #[arg(short, long, default_value = "testnet")]
        network: String,
    },
}

#[derive(Parser)]
pub enum FacilitatorCommands {
    #[command(name = "start")]
    Start {
        #[arg(short, long, default_value = "3001")]
        port: u16,
        #[arg(long)]
        wallet: Option<String>,
        #[arg(long)]
        private_key: Option<String>,
        #[arg(short, long, default_value = "testnet")]
        network: String,
    },
    #[command(name = "stop")]
    Stop,
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

pub async fn init(name: String, chain: String, framework: String) -> Result<()> {
    println!(
        "{}",
        format!("Initializing x402 project: {}", name.cyan()).bold()
    );

    let project = Project::new(name.clone(), chain, framework);

    println!("{}", "  Creating project structure...".dimmed());
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
        WalletCommands::Import { private_key, network } => {
            println!("{}", "Importing wallet...".cyan());

            let wallet = Wallet::import(&private_key, &network)?;

            wallet.save_to_file()?;

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
        FacilitatorCommands::Start { port, wallet, private_key, network } => {
            let wallet = if let Some(private_key) = private_key {
                Wallet::import(&private_key, &network)?
            } else if let Some(wallet_address) = wallet {
                Wallet::load_from_address(&wallet_address)?
            } else {
                Wallet::find_default()?
            };

            let _facilitator = Facilitator::start(port, wallet)?;

            println!("{}", "  Start facilitator in background...".dimmed());
            println!(
                "{}",
                "  Run `x402 facilitator stop` to stop".yellow().dimmed()
            );

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

            test::test_payment_flow(&api, amount).await?;

            Ok(())
        }
    }
}

pub async fn deploy(provider: String) -> Result<()> {
    println!("{}", format!("Deploying to {}", provider.cyan()).bold());

    deploy::deploy(&provider).await?;

    Ok(())
}
