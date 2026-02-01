use anyhow::Result;
use clap::{Parser, Subcommand};
use x402_cli::{handle_facilitator, handle_test, handle_wallet, init};

#[derive(Parser)]
#[command(
    name = "x402",
    version = "1.0.0",
    about = "Developer CLI for x402 projects"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        chain: String,
        #[arg(short, long)]
        framework: String,
    },
    Wallet {
        #[command(subcommand)]
        command: x402_cli::WalletCommands,
    },
    Facilitator {
        #[command(subcommand)]
        command: x402_cli::FacilitatorCommands,
    },
    Test {
        #[command(subcommand)]
        command: x402_cli::TestCommands,
    },
    Deploy {
        #[arg(short, long)]
        provider: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            chain,
            framework,
        } => {
            init(name, chain, framework).await?;
        }
        Commands::Wallet { command } => {
            handle_wallet(command).await?;
        }
        Commands::Facilitator { command } => {
            handle_facilitator(command).await?;
        }
        Commands::Test { command } => {
            handle_test(command).await?;
        }
        Commands::Deploy { provider } => {
            x402_cli::deploy(provider).await?;
        }
    }

    Ok(())
}
