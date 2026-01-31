use anyhow::Result;
use clap::Parser;

mod x402;

#[derive(Parser)]
#[command(name = "x402")]
#[command(about = "Automated x402 project lifecycle management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    #[command(name = "init")]
    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "aptos")]
        chain: String,
        #[arg(short, long, default_value = "next")]
        framework: String,
    },

    #[command(name = "wallet")]
    Wallet {
        #[command(subcommand)]
        command: x402::WalletCommands,
    },

    #[command(name = "facilitator")]
    Facilitator {
        #[command(subcommand)]
        command: x402::FacilitatorCommands,
    },

    #[command(name = "test")]
    Test {
        #[command(subcommand)]
        command: x402::TestCommands,
    },

    #[command(name = "deploy")]
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
        Commands::Init { name, chain, framework } => {
            x402::init(name, chain, framework).await?;
        }
        Commands::Wallet { command } => {
            x402::handle_wallet(command).await?;
        }
        Commands::Facilitator { command } => {
            x402::handle_facilitator(command).await?;
        }
        Commands::Test { command } => {
            x402::handle_test(command).await?;
        }
        Commands::Deploy { provider } => {
            x402::deploy(provider).await?;
        }
    }

    Ok(())
}
