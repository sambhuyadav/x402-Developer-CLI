# x402-cli

> A command-line tool for automating x402 project lifecycle management

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://shields.io/)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange)](https://www.rust-lang.org)

## About

**x402-cli** is a Rust-based command-line tool designed to automate the lifecycle of an x402 project. It acts as a bridge between a developer's local code, the Aptos blockchain, and the x402 facilitator.

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
git clone https://github.com/sambhuyadav/x402-Developer-CLI.git
cd x402-Developer-CLI

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
# 1. Initialize a new x402 project
x402-cli init my-weather-api --chain aptos --framework next

# 2. Create a test wallet
x402-cli wallet create --network testnet

# 3. Start the local facilitator
x402-cli facilitator start --port 3001

# 4. Test the payment flow
x402-cli test payment --api http://localhost:3000/weather --amount 1000

# 5. Deploy to production
x402-cli deploy --provider vercel
```

## Command Reference

### `init` - Initialize a new project

Create a new x402-enabled project with the specified framework.

```bash
x402-cli init --name <NAME> --chain <CHAIN> --framework <FRAMEWORK>
```

**Options:**
- `-n, --name <NAME>`: Project name (required)
- `-c, --chain <CHAIN>`: Blockchain network (e.g., aptos)
- `-f, --framework <FRAMEWORK>`: Framework to use (e.g., next, react, vanilla)

**Example:**
```bash
x402-cli init --name my-api --chain aptos --framework next
```

**Output:**
- Creates project directory structure (src/, config/, tests/, docs/)
- Generates configuration files (x402.toml, .env.example, .gitignore)
- Installs framework dependencies
- Generates README with x402-specific commands

### `wallet` - Manage wallets

Create and manage wallets for x402 transactions.

```bash
x402-cli wallet create [OPTIONS]
```

**Subcommands:**
- `create`: Create a new wallet

**Options for `create`:**
- `-n, --network <NETWORK>`: Network to use (default: testnet)

**Example:**
```bash
# Create a wallet on testnet
x402-cli wallet create --network testnet

# Create a wallet on mainnet
x402-cli wallet create --network mainnet
```

**Output:**
- Generates a new Ed25519 key pair
- Creates a 12-word BIP39 seed phrase
- Saves wallet to `~/.x402/wallets/<address>.json`
- Funds wallet from faucet (testnet only)

### `facilitator` - Manage facilitator server

Start and stop the local development facilitator server.

```bash
x402-cli facilitator <COMMAND>
```

**Subcommands:**
- `start`: Start the facilitator server
- `stop`: Stop all facilitator processes

**Options for `start`:**
- `-p, --port <PORT>`: Port to listen on (default: 3001)

**Examples:**
```bash
# Start facilitator on default port (3001) with auto-detected wallet
x402-cli facilitator start

# Start facilitator on custom port
x402-cli facilitator start --port 8080

# Start facilitator on mainnet
x402-cli facilitator start --network mainnet

# Start facilitator with specific wallet address
x402-cli facilitator start --wallet 0xe910ad0506573009839aa55ad8211ad01ba3f7394d93d849378d342449df09

# Start facilitator with private key and custom network
x402-cli facilitator start --private-key 0x<private_key> --network testnet

# Stop facilitator
x402-cli facilitator stop
```

**Options for `start`:**
- `-p, --port <PORT>`: Port to listen on (default: 3001)
- `--wallet <ADDRESS>`: Use wallet with this address (optional)
- `--private-key <KEY>`: Use wallet from this private key (optional)
- `-n, --network <NETWORK>`: Network to use (default: testnet)

**Output:**
- Starts a TCP server on the specified port
- Health check endpoint at `http://localhost:<port>/health`
- Handles payment facilitation requests
- Uses specified wallet for payment transactions (defaults to first found wallet)

### `test` - Test payment flows

Test end-to-end payment flows to verify functionality.

```bash
x402-cli test payment [OPTIONS]
```

**Subcommands:**
- `payment`: Test a payment flow

**Options for `payment`:**
- `-a, --api <API>`: API endpoint to test (required)
- `-a, --amount <AMOUNT>`: Amount to pay in micro-APT (default: 1000)

**Example:**
```bash
x402-cli test payment --api http://localhost:3000/weather --amount 1000
```

**Output:**
- Step-by-step payment flow progress
- Transaction hash and timing information
- Detailed error messages if failures occur

### `deploy` - Deploy to production

Deploy your facilitator to production platforms.

```bash
x402-cli deploy --provider <PROVIDER>
```

**Options:**
- `-p, --provider <PROVIDER>`: Deployment platform (e.g., vercel)

**Example:**
```bash
x402-cli deploy --provider vercel
```

**Output:**
- Builds the project
- Checks for provider CLI installation
- Initiates deployment process
- Provides deployment URL

## Configuration

### Project Configuration (`config/x402.toml`)

```toml
project_name = "my-project"
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

### Environment Variables (`.env`)

```bash
NODE_ENV=development
X402_CHAIN=aptos
X402_PROJECT=my-project
```

## Wallet Storage

Wallets are stored securely in:
- **Location**: `~/.x402/wallets/<address>.json`
- **Format**: JSON with address, private_key, network, and seed_phrase
- **Security**: Files are not encrypted - ensure system security

## Supported Frameworks

- **Next.js**: Full-featured React framework
- **React**: React library support
- **Vanilla**: Basic HTML/JavaScript setup

## Architecture

```
User Input (CLI) 
    → Command Parsing (Clap)
    → Command Routing (main.rs)
    → Module Handler (x402/mod.rs)
    → Domain Logic (project.rs/wallet.rs/facilitator.rs/test.rs/deploy.rs)
    → External Services (blockchain, file system, HTTP)
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name
```

### Adding New Features

1. Create new module in `src/x402/`
2. Add command enum in `src/main.rs`
3. Export from `src/lib.rs`
4. Add handler in `src/x402/mod.rs`
5. Test functionality

## Troubleshooting

### Wallet Funding Fails

If faucet funding fails with 405 Method Not Allowed:
- The Aptos faucet API may have changed
- Fund the wallet manually using the Aptos explorer
- Check the faucet service status

### Facilitator Won't Start

If the facilitator fails to start:
- Check if the port is already in use: `lsof -i :3001`
- Use a different port: `x402-cli facilitator start --port 3002`
- Check firewall settings

### Payment Flow Fails

If payment testing fails:
- Ensure the API endpoint is accessible
- Verify the facilitator is running
- Check network connectivity
- Review error messages for specific issues

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Resources

- [Aptos TypeScript SDK](https://github.com/aptos-labs/aptos-ts-sdk)
- [x402 Protocol Documentation](https://x402.org)
- [Aptos Developer Portal](https://developers.aptoslabs.com)

## Changelog

### v1.0.0 (2025-02-01)
- Initial release
- Project scaffolding with framework support
- Wallet creation and management
- Facilitator server management
- Payment flow testing
- Vercel deployment support
- Aptos blockchain integration
