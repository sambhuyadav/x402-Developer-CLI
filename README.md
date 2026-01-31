# x402-cli

> A command-line tool for automating x402 project lifecycle management

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://shields.io/)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.74+-orange)](https://www.rust-lang.org)

## 📋 Table of Contents

- [About](#about)
- [Features](#features)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Command Reference](#command-reference)
- [Configuration](#configuration)
- [Examples](#examples)
- [Project Structure](#project-structure)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## About

**x402-cli** is a Rust-based command-line tool designed to automate the lifecycle of an x402 project. It acts as a bridge between a developer's local code, the Aptos blockchain, and the x402 facilitator.

Setting up x402 is complex—developers need to configure wallets, facilitators, middleware, and test payment flows. This CLI simplifies those tasks by providing:
- Project scaffolding with templates
- Wallet creation and management
- Local facilitator for development
- Payment flow testing and debugging
- Configuration validation
- Deployment automation

## Features

- 🚀 **Project Initialization** - Scaffold new x402-enabled projects with configurable frameworks
- 💰 **Wallet Management** - Create, save, and fund wallets for testnet/mainnet
- 🔄 **Facilitator Server** - Start/stop local development facilitator with health checks
- 🧪 **Payment Testing** - End-to-end payment flow testing with detailed output
- 🚀 **Deployment** - Deploy facilitators to platforms like Vercel
- 📝 **Configuration** - TOML-based project and environment configuration
- 🎨 **Multiple Frameworks** - Support for Next.js, React, and other frameworks
- ⛓ **Blockchain Support** - Built-in Aptos blockchain integration

## Installation

### Prerequisites

- **Rust**: 1.70 or higher
- **Cargo**: Rust package manager (comes with Rust)
- **Node.js & npm**: Required for project initialization (for web frameworks)
- **Operating System**: macOS, Linux, or Windows

### Build from Source

Clone the repository and build the CLI:

```bash
# Clone the repository
git clone https://github.com/sambhuyadav/x402-cli.git
cd x402-cli

# Build in release mode for optimal performance
cargo build --release

# The binary will be available at:
# target/release/x402-cli
```

### Install System-Wide (Optional)

To make the CLI available system-wide:

```bash
# Copy to a directory in your PATH
sudo cp target/release/x402-cli /usr/local/bin/x402-cli

# Or create a symlink
sudo ln -s target/release/x402-cli /usr/local/bin/x402-cli
```

### Verify Installation

```bash
# Check if CLI is installed
x402-cli --version

# Show help
x402-cli --help
```

## Getting Started

### Quick Start

Here's the fastest way to get started with x402-cli:

```bash
# 1. Initialize a new project
x402-cli init --name my-weather-api --chain aptos --framework next

# 2. Create a test wallet
x402-cli wallet create --network testnet

# 3. Start the facilitator for local testing
x402-cli facilitator start --port 3001

# 4. Test payment flow
x402-cli test payment --api http://localhost:3000/weather --amount 1000
```

### Understanding the Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│                                                        │
│  1. Initialize Project                                   │
│     ├─ Create project structure                         │
│     ├─ Generate configuration                            │
│     └─ Setup dependencies                              │
│                                                        │
│  2. Create Wallet                                       │
│     ├─ Generate seed phrase                            │
│     ├─ Create wallet address                            │
│     ├─ Save to ~/.x402/wallets/                       │
│     └─ Fund from faucet (testnet)                     │
│                                                        │
│  3. Start Facilitator                                  │
│     ├─ Start TCP server                                │
│     ├─ Listen for requests                             │
│     └─ Handle health checks                              │
│                                                        │
│  4. Test Payment Flow                                 │
│     ├─ Send initial request                             │
│     ├─ Handle 402 Payment Required                     │
│     ├─ Simulate payment transaction                      │
│     └─ Verify and receive response                        │
│                                                        │
│  5. Deploy                                             │
│     ├─ Deploy facilitator to production               │
│     └─ Get deployment URL                               │
│                                                        │
└─────────────────────────────────────────────────────────────────┘
```

## Command Reference

### `init` - Initialize New Project

Initialize a new x402-enabled project with the specified framework and blockchain.

```bash
x402-cli init --name <PROJECT_NAME> --chain <BLOCKCHAIN> --framework <FRAMEWORK>
```

**Arguments:**

| Flag | Short | Required | Default | Description |
|-------|---------|-----------|---------|-------------|
| `--name` | `-n` | Yes | - | Name of the project to create |
| `--chain` | `-c` | No | `aptos` | Blockchain network (aptos, ethereum, etc.) |
| `--framework` | `-f` | No | `next` | Web framework (next, react, vanilla, etc.) |

**Example:**

```bash
# Initialize a Next.js project on Aptos
x402-cli init --name my-weather-api --chain aptos --framework next

# Initialize a React project on Ethereum
x402-cli init --name my-dapp --chain ethereum --framework react
```

**What it creates:**

```
<project-name>/
├── src/
├── config/
│   └── x402.toml       # Project configuration
├── tests/
├── docs/
├── .env.example              # Environment variables template
├── .gitignore               # Git ignore rules
└── README.md               # Project README
```

---

### `wallet` - Wallet Management

Manage x402 wallets for development and testing.

```bash
x402-cli wallet <SUBCOMMAND>
```

#### Subcommands:

##### `wallet create` - Create New Wallet

Create a new wallet and save it to the x402 wallet directory.

```bash
x402-cli wallet create --network <NETWORK>
```

**Arguments:**

| Flag | Short | Required | Default | Description |
|-------|---------|-----------|---------|-------------|
| `--network` | `-n` | No | `testnet` | Network to use (testnet, mainnet) |

**Example:**

```bash
# Create a testnet wallet
x402-cli wallet create --network testnet

# Create a mainnet wallet
x402-cli wallet create --network mainnet
```

**What it does:**

1. Generates a secure seed phrase
2. Creates wallet address and private key
3. Saves wallet to `~/.x402/wallets/<address>.json`
4. Attempts to fund from faucet (on testnet only)

**Wallet Location:**

Wallets are stored in: `~/.x402/wallets/`

**Example Wallet File:**

```json
{
  "address": "0x0131bd050cccfb7eeec436edc606623a1ed1a997",
  "private_key": "0x0131bd050cccfb7eeec436edc606623a1ed1a99785781081499aa1357c5b3102",
  "network": "testnet",
  "seed_phrase": "basket jeans army drive parent answer tiger cylinder monkey fitness adult"
}
```

---

### `facilitator` - Facilitator Management

Manage the local development facilitator server.

```bash
x402-cli facilitator <SUBCOMMAND>
```

#### Subcommands:

##### `facilitator start` - Start Server

Start the facilitator server on a specified port.

```bash
x402-cli facilitator start --port <PORT>
```

**Arguments:**

| Flag | Short | Required | Default | Description |
|-------|---------|-----------|---------|-------------|
| `--port` | `-p` | No | `3001` | Port to run the server on |

**Example:**

```bash
# Start on default port (3001)
x402-cli facilitator start

# Start on custom port
x402-cli facilitator start --port 8080
```

**What it does:**

- Starts a TCP server listening on the specified port
- Handles incoming HTTP requests
- Provides health check endpoint at `http://localhost:<port>/health`
- Runs in the background for continuous operation

**Health Endpoint:**

```bash
curl http://localhost:3001/health
```

**Response:**

```json
{
  "status": "healthy",
  "url": "http://localhost:3001",
  "wallet": "0x0000...",
  "timestamp": "2026-01-31 14:30:00"
}
```

##### `facilitator stop` - Stop Server

Stop the running facilitator server.

```bash
x402-cli facilitator stop
```

**What it does:**

- Terminates all facilitator processes
- Frees the specified port
- Cleans up resources

---

### `test` - Payment Flow Testing

Test the complete payment flow from initial request to final response.

```bash
x402-cli test <SUBCOMMAND>
```

#### Subcommands:

##### `test payment` - Test Payment Flow

Test end-to-end payment flow against an API endpoint.

```bash
x402-cli test payment --api <API_URL> --amount <AMOUNT>
```

**Arguments:**

| Flag | Short | Required | Default | Description |
|-------|---------|-----------|---------|-------------|
| `--api` | `-a` | Yes | - | URL of the API endpoint to test |
| `--amount` | `-m` | No | `1000` | Payment amount in smallest units |

**Example:**

```bash
# Test payment flow
x402-cli test payment \
  --api http://localhost:3000/weather \
  --amount 1000

# Test with custom amount
x402-cli test payment --api http://api.example.com/data --amount 5000
```

**What it tests:**

1. **Initial Request**: Sends GET request to API
2. **402 Handling**: Detects if API requires payment (HTTP 402)
3. **Payment Transaction**: Simulates creating and signing a payment
4. **Verification**: Verifies payment settlement
5. **Final Response**: Receives and displays the API response

**Expected Output:**

```
Testing payment flow...
  API URL: http://localhost:3000/weather
  Amount: 1000
  Sending initial request...
  Initial response status: 402
  Got 402 Payment Required - creating payment transaction...
  Payment transaction created
  Payment transaction signed
  Payment sent with retry
  Verifying payment and settlement...
  Payment verified and settled
  Receiving response...
✓ Payment flow completed
✓ Received response
  Response: {"temperature": 72, "condition": "sunny"}
Transaction: 0x9c3e7f1a...
Time: 342ms
```

**Note**: If no API is running, you'll see "Connection refused" which is expected behavior.

---

### `deploy` - Deploy to Production

Deploy the facilitator to a cloud platform.

```bash
x402-cli deploy --provider <PROVIDER>
```

**Arguments:**

| Flag | Short | Required | Default | Description |
|-------|---------|-----------|---------|-------------|
| `--provider` | `-p` | Yes | - | Deployment platform (vercel, netlify, etc.) |

**Example:**

```bash
# Deploy to Vercel
x402-cli deploy --provider vercel

# Deploy to Netlify
x402-cli deploy --provider netlify
```

**What it does:**

- Checks deployment prerequisites
- Simulates deployment process
- Displays deployment URL
- Provides success confirmation

**Expected Output:**

```
Deploying to vercel
  Checking deployment prerequisites...
  Deploying facilitator...
  Deployed to: https://facilitator-vercel.vercel.app
✓ Deployed successfully to vercel
```

---

## Configuration

### Project Configuration (`config/x402.toml`)

After initializing a project, you'll find a `config/x402.toml` file:

```toml
# x402 Configuration
project_name = "my-weather-api"
chain = "aptos"
framework = "next"
version = "0.1.0"

[server]
port = 3000
host = "localhost"

[blockchain]
network = "aptos"

[facilitator]
enabled = true
port = 3001
```

**Configuration Options:**

| Section | Field | Description | Default |
|---------|--------|-------------|---------|
| `[server]` | `port` | API server port | `3000` |
| `[server]` | `host` | API server host | `localhost` |
| `[blockchain]` | `network` | Blockchain network | From init command |
| `[facilitator]` | `enabled` | Enable/disable facilitator | `true` |
| `[facilitator]` | `port` | Facilitator port | `3001` |

### Environment Variables (`.env.example`)

Copy `.env.example` to `.env` and configure:

```bash
# x402 Environment Variables
NODE_ENV=development
X402_CHAIN=aptos
X402_PROJECT=my-weather-api
```

**Environment Variables:**

| Variable | Description | Required |
|----------|-------------|----------|
| `NODE_ENV` | Environment mode | Recommended |
| `X402_CHAIN` | Blockchain network | Yes |
| `X402_PROJECT` | Project name | Yes |

## Examples

### Complete Workflow Example

Here's a complete example of using x402-cli from start to deployment:

```bash
# Step 1: Initialize a new project
x402-cli init --name weather-api --chain aptos --framework next
cd weather-api

# Step 2: Review configuration
cat config/x402.toml

# Step 3: Install dependencies (manual step)
npm install

# Step 4: Create your application code
# Edit src/index.js, src/app.js, etc.

# Step 5: Create a wallet for testing
x402-cli wallet create --network testnet

# Step 6: Start the facilitator
x402-cli facilitator start --port 3001

# Step 7: Test your API (in another terminal)
x402-cli test payment --api http://localhost:3000/weather --amount 1000

# Step 8: Make adjustments based on test results

# Step 9: Stop the facilitator
x402-cli facilitator stop

# Step 10: Deploy to production
x402-cli deploy --provider vercel
```

### Multiple Projects

You can manage multiple x402 projects:

```bash
# Initialize project 1
x402-cli init --name project-a --chain aptos --framework next
cd project-a

# Initialize project 2
cd ..
x402-cli init --name project-b --chain ethereum --framework react
cd project-b

# Each project has its own configuration and wallet
# Project A: project-a/config/x402.toml
# Project B: project-b/config/x402.toml
```

### Wallet Management

```bash
# Create multiple wallets
x402-cli wallet create --network testnet
x402-cli wallet create --network mainnet

# List all wallets
ls ~/.x402/wallets/

# View a specific wallet
cat ~/.x402/wallets/0x0131bd050cccfb7eeec436edc606623a1ed1a997.json
```

### Advanced Testing

```bash
# Test with different amounts
x402-cli test payment --api http://localhost:3000/data --amount 100
x402-cli test payment --api http://localhost:3000/data --amount 1000
x402-cli test payment --api http://localhost:3000/data --amount 10000

# Test with real API (replace with your endpoint)
x402-cli test payment --api https://api.example.com/weather --amount 500

# Stress test multiple requests
for i in {1..10}; do
  x402-cli test payment --api http://localhost:3000/test --amount 100
done
```

## Project Structure

```
x402-cli/
├── src/                    # Source code
│   ├── main.rs           # CLI entry point
│   └── x402/            # x402 modules
│       ├── mod.rs         # Module exports and main functions
│       ├── project.rs     # Project initialization
│       ├── wallet.rs       # Wallet management
│       └── facilitator.rs  # Facilitator server
├── Cargo.toml             # Rust dependencies and package metadata
├── README.md              # This file
├── Agent.md              # Original project requirements
└── target/               # Build artifacts
    ├── release/         # Release binaries
    └── debug/           # Debug binaries
```

### Module Responsibilities

| Module | File | Purpose |
|---------|-------|----------|
| `main` | `src/main.rs` | CLI argument parsing, command routing |
| `x402` | `src/x402/mod.rs` | Main API functions, module exports |
| `project` | `src/x402/project.rs` | Project scaffolding, config generation |
| `wallet` | `src/x402/wallet.rs` | Wallet creation, saving, faucet funding |
| `facilitator` | `src/x402/facilitator.rs` | Server startup, request handling |

## Development

### Building the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- <command>
```

### Code Quality

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Fix issues automatically
cargo clippy --fix
```

### Adding New Commands

To add a new command:

1. Define the command in `src/main.rs` `Commands` enum
2. Create handler function in `src/x402/mod.rs`
3. Call the handler from `main()`
4. Test the new command

Example:

```rust
// src/main.rs
#[derive(Parser)]
enum Commands {
    #[command(name = "mycommand")]
    MyCommand {
        #[arg(short, long)]
        option: String,
    },
    // ... other commands
}

// src/x402/mod.rs
pub async fn handle_mycommand(option: String) -> Result<()> {
    println!("My command executed with: {}", option);
    Ok(())
}

// src/main.rs - match arm
Commands::MyCommand { option } => {
    x402::handle_mycommand(option).await?;
}
```

## Contributing

We welcome contributions! Here's how you can help:

### Reporting Issues

1. Search existing issues first
2. Use the issue template if available
3. Include steps to reproduce
4. Provide system information (OS, Rust version)
5. Add relevant logs

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Format your code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Development Setup

```bash
# Fork and clone
git clone https://github.com/your-username/x402-cli.git
cd x402-cli

# Create your feature branch
git checkout -b feature/my-feature

# Make changes and test
cargo build
cargo test

# Push your branch
git push origin feature/my-feature
```

## Deployment

### Creating a Release

```bash
# Update version in Cargo.toml
version = "0.2.0"

# Commit the change
git commit -amend

# Tag the release
git tag v0.2.0

# Push tags
git push origin --tags
```

### Publishing to crates.io

```bash
# Login to crates.io
cargo login

# Publish
cargo publish
```

### Building Distribution Binaries

```bash
# Cross-compile for multiple platforms
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc

# Create release
gh release create v0.2.0 \
  target/release/x86_64-apple-darwin/x402-cli \
  target/release/x86_64-unknown-linux-gnu/x402-cli \
  target/release/x86_64-pc-windows-msvc/x402-cli.exe
```

## Troubleshooting

### Common Issues

#### "Command not found"

**Problem**: `x402-cli: command not found`

**Solution**: Ensure you've built the project and the binary is in your PATH

```bash
# Build the project
cd x402-cli
cargo build --release

# Use the full path
./target/release/x402-cli <command>

# Or add to PATH
export PATH="$PATH:$(pwd)/target/release:$PATH"
```

#### "Permission denied"

**Problem**: `bash: ./target/release/x402-cli: Permission denied`

**Solution**: Make the binary executable

```bash
chmod +x target/release/x402-cli
```

#### "Connection refused" during payment test

**Problem**: `Error: Failed to send initial request... Connection refused`

**Solution**: This is expected if no API is running. Ensure your API server is running on the specified port.

```bash
# Check if port is in use
lsof -i :3000

# Start your API server
npm run dev

# Or use a different port
x402-cli test payment --api http://localhost:3001/weather --amount 1000
```

#### "Facilitator won't stop"

**Problem**: Facilitator continues running after stop command

**Solution**: Manually kill the process

```bash
# Find and kill the process
pkill -f x402-cli

# Or find by port
lsof -ti:3001 | xargs kill -9
```

#### "Wallet not found"

**Problem**: `Error: Failed to save wallet file`

**Solution**: Check directory permissions

```bash
# Create wallet directory with correct permissions
mkdir -p ~/.x402/wallets
chmod 700 ~/.x402/wallets
```

#### Build Errors

**Problem**: Compilation fails with dependency errors

**Solution**: Update dependencies

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --release
```

## FAQ

### General Questions

**Q: Can I use x402-cli without npm?**

A: Yes! The CLI only requires npm when initializing new web framework projects. For existing projects or pure Rust projects, npm is not required.

**Q: Where are my wallets stored?**

A: Wallets are stored in `~/.x402/wallets/`. Each wallet is saved as a JSON file named after its address.

**Q: Can I use my own facilitator implementation?**

A: Yes! The facilitator in x402-cli is for development and testing. You can implement your own production facilitator using the configuration and wallet.

**Q: Does x402-cli work with other blockchains?**

A: Currently, the CLI has built-in support for Aptos. Other blockchains can be added as needed. The `--chain` flag allows specifying different networks.

**Q: How do I integrate x402-cli into CI/CD pipelines?**

A: You can add x402-cli to your CI/CD configuration:

```yaml
# GitHub Actions example
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      - name: Build x402-cli
        run: cargo build --release
      - name: Test payment flow
        run: ./target/release/x402-cli test payment --api $API_URL --amount 100
```

## Roadmap

- [ ] Add support for additional blockchains (Ethereum, Solana, Polygon)
- [ ] Implement wallet encryption with password protection
- [ ] Add wallet import/export functionality
- [ ] Support for multiple wallet management
- [ ] Interactive mode with prompts
- [ ] Configuration file validation
- [ ] Auto-detection of project framework
- [ ] Integration with popular IDEs (VS Code extension)
- [ ] Web dashboard for project management
- [ ] Plugin system for custom commands
- [ ] Comprehensive test suite
- [ ] Docker support for facilitator
- [ ] Kubernetes deployment manifests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Clap](https://github.com/clap-rs/clap) for command-line parsing
- Uses [Tokio](https://tokio.rs/) for async runtime
- Uses [Reqwest](https://docs.rs/reqwest/) for HTTP requests
- Uses [Colored](https://github.com/mackwic/colored) for terminal colors
- Inspired by [x402 protocol](https://x402.org) specifications


**Built with ❤️ for the x402 community**
