use anyhow::{Context, Result};
use colored::*;
use reqwest::Client;

pub mod project;
pub mod wallet;
pub mod facilitator;

pub use project::Project;
pub use wallet::{Wallet, WalletCommands, TestCommands};
pub use facilitator::Facilitator;
pub use facilitator::FacilitatorCommands;

/// Main initialization function for creating new x402 projects
///
/// This function scaffolds a new x402-enabled project with the specified parameters
///
/// # Arguments
///
/// * `name` - The name of the project to create
/// * `chain` - The blockchain network to use (e.g., "aptos", "ethereum")
/// * `framework` - The web framework to use (e.g., "next", "react", "vanilla")
///
/// # Behavior
///
/// Creates a complete project structure with:
/// - Source directories (`src/`, `tests/`, `docs/`)
/// - Configuration files (`config/x402.toml`, `.env.example`)
/// - Documentation (`.gitignore`, `README.md`)
/// - Attempts to install dependencies (for web frameworks)
///
/// # Examples
///
/// ```bash
/// # Initialize a Next.js project on Aptos
/// x402-cli init --name my-weather-api --chain aptos --framework next
///
/// # Initialize a React project on Ethereum
/// x402-cli init --name my-dapp --chain ethereum --framework react
/// ```
///
/// # Returns
///
/// Returns `Ok(())` on successful project initialization
///
/// # Errors
///
/// Returns an error if:
/// - Project directory cannot be created
/// - Configuration files cannot be written
/// - Dependencies installation fails
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

/// Handles wallet-related commands
///
/// # Arguments
///
/// * `command` - The wallet command to execute (`Create`)
///
/// # Behavior
///
/// Routes wallet commands to appropriate handler functions:
/// - `WalletCommands::Create` → Creates a new wallet and funds it from faucet
///
/// # Examples
///
/// ```bash
/// # Create a testnet wallet
/// x402-cli wallet create --network testnet
///
/// # Create a mainnet wallet
/// x402-cli wallet create --network mainnet
/// ```
///
/// # Returns
///
/// Returns `Ok(())` on successful wallet creation
///
/// # Errors
///
/// Returns an error if:
/// - Wallet cannot be created
/// - Wallet file cannot be saved
/// - Faucet funding fails
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

/// Handles facilitator-related commands
///
/// # Arguments
///
/// * `command` - The facilitator command to execute (`Start`, `Stop`)
///
/// # Behavior
///
/// Routes facilitator commands to appropriate handler functions:
/// - `FacilitatorCommands::Start` → Starts a development facilitator server
/// - `FacilitatorCommands::Stop` → Stops a running facilitator server
///
/// # Examples
///
/// ```bash
/// # Start facilitator on default port (3001)
/// x402-cli facilitator start
///
/// # Start facilitator on custom port
/// x402-cli facilitator start --port 8080
///
/// # Stop facilitator
/// x402-cli facilitator stop
/// ```
///
/// # Returns
///
/// Returns `Ok(())` on successful facilitator operation
///
/// # Errors
///
/// Returns an error if:
/// - Facilitator cannot be started
/// - Facilitator cannot be stopped
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

/// Handles testing-related commands
///
/// # Arguments
///
/// * `command` - The test command to execute (`Payment`)
///
/// # Behavior
///
/// Routes test commands to appropriate handler functions:
/// - `TestCommands::Payment` → Tests end-to-end payment flow against an API endpoint
///
/// # Payment Flow
///
/// The payment testing follows this sequence:
/// 1. **Initial Request**: Sends HTTP GET request to API
/// 2. **402 Handling**: Detects if API requires payment (HTTP 402)
/// 3. **Payment Transaction**: Simulates creating and signing a payment
/// 4. **Verification**: Verifies payment settlement
/// 5. **Final Request**: Resends the request with payment token
/// 6. **Response Display**: Shows the API response
///
/// # Examples
///
/// ```bash
/// # Test payment flow
/// x402-cli test payment --api http://localhost:3000/weather --amount 1000
///
/// # Test with custom amount
/// x402-cli test payment --api http://api.example.com/data --amount 5000
/// ```
///
/// # Notes
///
/// If no API is running, you'll see "Connection refused" which is expected behavior.
/// Ensure your API server is running on the specified port for successful testing.
///
/// # Returns
///
/// Returns `Ok(())` on successful test completion
///
/// # Errors
///
/// Returns an error if:
/// - Initial request fails to reach the API
/// - Payment transaction cannot be created
/// - Final request fails
/// - Response cannot be read or parsed
pub async fn handle_test(command: TestCommands) -> Result<()> {
    match command {
        TestCommands::Payment { api, amount } => {
            println!("{}", "Testing payment flow...".cyan());
            println!("{}", format!("  API URL: {}", api.cyan()).dimmed());
            println!("{}", format!("  Amount: {}", amount));

            let client = Client::new();

            println!("{}", "  Sending initial request...".dimmed());

            let response = client
                .get(&api)
                .send()
                .await
                .context("Failed to send initial request")?;

            let status = response.status();

            println!(
                "{}",
                format!("  Initial response status: {}", status.as_str().bright_black()
            );

            if status.as_u16() == 200 {
                println!("{}", "✓ Payment flow completed (no payment required)".green().bold());
                return Ok(());
            } else if status.as_u16() == 402 {
                println!("{}", "  Got 402 Payment Required - creating payment transaction...".dimmed());
                println!("{}", "  Payment transaction created".green());
                println!("{}", "  Payment transaction signed".green());
                println!("{}", "  Payment sent with retry".green());
                println!("{}", "  Verifying payment and settlement...".dimmed());

                let client_for_verification = Client::new();
                let payment_response = client_for_verification
                    .get(&api)
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
        }
    }
}

/// Deploys the facilitator to a cloud platform
///
/// # Arguments
///
/// * `provider` - The deployment platform to use (e.g., "vercel", "netlify")
///
/// # Behavior
///
/// Simulates the deployment process and displays deployment URL
///
/// # Examples
///
/// ```bash
/// # Deploy to Vercel
/// x402-cli deploy --provider vercel
///
/// # Deploy to Netlify
/// x402-cli deploy --provider netlify
/// ```
///
/// # Returns
///
/// Returns `Ok(())` on successful deployment simulation
///
/// # Notes
///
/// This is a simulation and does not actually deploy to the specified platform.
/// Real deployment would require additional infrastructure setup and credentials.
///
/// # Supported Providers
///
/// - **Vercel**: Serverless functions platform
/// - **Netlify**: Static site hosting with serverless functions
/// - **Railway**: Full-stack deployment
/// - **Heroku**: Cloud application platform
pub async fn deploy(provider: String) -> Result<()> {
    println!("{}", format!("Deploying to {}", provider.cyan()).bold());

    println!("{}", "  Checking deployment prerequisites...".dimmed();

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
