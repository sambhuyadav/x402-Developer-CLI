# x402-cli Integration Guide

This guide helps you integrate x402-cli into your own projects and development workflows.

## Quick Integration

### For End Users

If you just want to **use** the x402-cli tool:

1. **Build from source** (see main README.md for detailed instructions)
2. **Add to PATH** for easy access:

```bash
# Add to PATH temporarily (current session)
export PATH="$PATH:/path/to/x402-cli/target/release:$PATH"

# Add to PATH permanently (bash)
echo 'export PATH="$PATH:/path/to/x402-cli/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Add to PATH permanently (zsh)
echo 'export PATH="$PATH:/path/to/x402-cli/target/release:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### For Developers Integrating x402-cli

If you want to **integrate x402-cli** into your own application or service:

#### Option 1: Use as a Library

You can import x402-cli modules as a library in your Rust project:

```toml
# Cargo.toml
[dependencies]
x402-cli = { path = "/path/to/x402-cli" }
```

Then use the modules in your code:

```rust
use x402_cli::{Wallet, Facilitator, Project};

// Create a project
let project = Project::new("my-app".to_string(), "aptos".to_string(), "next".to_string());
project.create_directories()?;

// Create a wallet
let wallet = Wallet::create("testnet").await?;
wallet.save_to_file()?;

// Start facilitator
let facilitator = Facilitator::start(3001)?;
```

#### Option 2: Call as a Subprocess

From any programming language, you can execute x402-cli commands:

**JavaScript/Node.js:**
```javascript
const { execSync } = require('child_process');

// Initialize a project
execSync('./x402-cli init --name my-app --chain aptos --framework next', { stdio: 'inherit' });

// Create wallet
execSync('./x402-cli wallet create --network testnet', { stdio: 'inherit' });

// Start facilitator
const facilitator = execSync('./x402-cli facilitator start --port 3001', { stdio: 'pipe' });
console.log(facilitator.toString());
```

**Python:**
```python
import subprocess
import json

# Initialize project
result = subprocess.run(['./x402-cli', 'init', '--name', 'my-app', '--chain', 'aptos', '--framework', 'next'],
    capture_output=True, text=True)
print(result.stdout)

# Create wallet
result = subprocess.run(['./x402-cli', 'wallet', 'create', '--network', 'testnet'],
    capture_output=True, text=True)
print(result.stdout)

# Get wallet info
with open('~/.x402/wallets/0x*.json', 'r') as f:
    wallet = json.load(f)
    print(f"Wallet address: {wallet['address']}")
```

**Go:**
```go
package main

import (
    "fmt"
    "os/exec"
)

func main() {
    // Initialize project
    cmd := exec.Command("./x402-cli", "init", "--name", "my-app", "--chain", "aptos", "--framework", "next")
    output, _ := cmd.CombinedOutput()
    fmt.Println(string(output))

    // Create wallet
    cmd = exec.Command("./x402-cli", "wallet", "create", "--network", "testnet")
    output, _ = cmd.CombinedOutput()
    fmt.Println(string(output))
}
```

**Bash/Shell:**
```bash
#!/bin/bash
# Initialize project
./x402-cli init --name my-app --chain aptos --framework next

# Create wallet
WALLET_ADDRESS=$(./x402-cli wallet create --network testnet 2>&1 | grep "Wallet Address:" | cut -d: -f2)
echo "Created wallet: $WALLET_ADDRESS"

# Start facilitator
./x402-cli facilitator start --port 3001 &
FACILITATOR_PID=$!
echo "Facilitator started with PID: $FACILITATOR_PID"

# Do work...

# Stop facilitator when done
./x402-cli facilitator stop
```

## CI/CD Integration

### GitHub Actions

Create `.github/workflows/x402-test.yml`:

```yaml
name: Test x402 Payment Flow

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  workflow_dispatch:

jobs:
  test-payment:
    name: Test Payment Flow
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Set up Rust
        run: |
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          echo "$HOME/.cargo/bin" >> $GITHUB_PATH

      - name: Install dependencies
        run: sudo apt update && sudo apt install -y curl

      - name: Build x402-cli
        run: |
          cd x402-cli
          cargo build --release

      - name: Create test wallet
        run: |
          ./x402-cli/target/release/x402-cli wallet create --network testnet

      - name: Start test API
        run: |
          # Start your test API here
          npm run dev &
          sleep 10

      - name: Test payment flow
        run: |
          ./x402-cli/target/release/x402-cli test payment \
            --api http://localhost:3000/test \
            --amount 1000

      - name: Upload results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test-results.txt
```

### GitLab CI

Create `.gitlab-ci.yml`:

```yaml
stages:
  - test

test-payment:
  stage: test
  image: rust:latest
  before_script:
    - cd x402-cli
    - cargo build --release
  script:
    - ./target/release/x402-cli wallet create --network testnet
    - ./target/release/x402-cli test payment --api $API_URL --amount 1000
  artifacts:
    paths:
      - test-results.txt
```

### Travis CI

Create `.travis.yml`:

```yaml
language: rust
cache: cargo

before_install:
  - cd x402-cli

script:
  - cargo build --release
  - ./target/release/x402-cli wallet create --network testnet
  - ./target/release/x402-cli test payment --api $API_URL --amount 1000
```

## Docker Integration

### Using x402-cli in Docker

Create a `Dockerfile`:

```dockerfile
FROM rust:1.74-slim as builder

WORKDIR /app

# Copy x402-cli source
COPY x402-cli /app/x402-cli
WORKDIR /app/x402-cli

# Build the CLI
RUN cargo build --release

# Final stage
FROM rust:1.74-slim

WORKDIR /app
COPY --from=builder /app/x402-cli/target/release/x402-cli /usr/local/bin/

# Create necessary directories
RUN mkdir -p /root/.x402/wallets

# Set PATH
ENV PATH="/usr/local/bin:${PATH}"

# Default command
CMD ["x402-cli", "--help"]
```

**Build and run:**

```bash
# Build the image
docker build -t x402-cli:latest .

# Run x402-cli commands in Docker
docker run --rm -v ~/.x402:/root/.x402 x402-cli:latest \
  init --name docker-app --chain aptos --framework next

# For wallet operations (persisted volume)
docker run --rm -v ~/.x402:/root/.x402 x402-cli:latest \
  wallet create --network testnet
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  x402-cli:
    build: .
    image: x402-cli:latest
    volumes:
      - ~/.x402:/root/.x402
    working_dir: /app

  test-api:
    image: node:18
    command: npm run dev
    ports:
      - "3000:3000"

  facilitator:
    depends_on:
      - x402-cli
    image: x402-cli:latest
    command: facilitator start --port 3001
    ports:
      - "3001:3001"
```

## NPM Integration

### Using x402-cli as an NPM Package

You can publish x402-cli to npm for easy installation:

**1. Create `package.json`:**

```json
{
  "name": "x402-cli",
  "version": "0.1.0",
  "description": "A command-line tool for automating x402 project lifecycle management",
  "bin": {
    "x402-cli": "./x402-cli"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/your-username/x402-cli.git"
  },
  "keywords": [
    "x402",
    "cli",
    "aptos",
    "blockchain",
    "payment"
  ],
  "license": "MIT",
  "os": [
    "darwin",
    "linux",
    "win32"
  ],
  "engines": {
    "node": ">=14.0.0"
  }
}
```

**2. Create installation script `install.sh`:**

```bash
#!/bin/bash

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

# Download appropriate binary
case "$OS" in
  Darwin)
    if [ "$ARCH" = "arm64" ]; then
      BINARY="x402-cli-aarch64-apple-darwin"
    else
      BINARY="x402-cli-x86_64-apple-darwin"
    fi
    ;;
  Linux)
    BINARY="x402-cli-x86_64-unknown-linux-gnu"
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# Create installation directory
INSTALL_DIR="$HOME/.x402/bin"
mkdir -p "$INSTALL_DIR"

# Download binary
echo "Downloading x402-cli..."
curl -L "https://github.com/your-username/x402-cli/releases/download/v0.1.0/$BINARY" -o "$INSTALL_DIR/x402-cli"

# Make executable
chmod +x "$INSTALL_DIR/x402-cli"

# Add to PATH
echo "Adding x402-cli to PATH..."
echo 'export PATH="$HOME/.x402/bin:$PATH"' >> ~/.bashrc
echo 'export PATH="$HOME/.x402/bin:$PATH"' >> ~/.zshrc

# Reload shell
source ~/.bashrc 2>/dev/null || source ~/.zshrc 2>/dev/null

echo "✓ x402-cli installed successfully!"
echo "Run 'x402-cli --help' to get started"
```

**3. Publish to npm:**

```bash
# Create a minimal package structure
mkdir -p npm-package
cp README.md npm-package/
cp install.sh npm-package/
cp package.json npm-package/

# Publish
cd npm-package
npm publish
```

**Users can then install:**

```bash
npm install -g x402-cli

# Use globally
x402-cli init --name my-app --chain aptos --framework next
```

## Homebrew Integration

### Creating a Homebrew Formula

Create `Formula/x402-cli.rb`:

```ruby
class X402Cli < Formula
  desc "A command-line tool for automating x402 project lifecycle management"
  homepage "https://github.com/your-username/x402-cli"
  url "https://github.com/your-username/x402-cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "SHA256_HASH_OF_TARBALL"
  license "MIT"

  depends_on "rust"

  def install
    system "cargo", "build", "--release"

    bin.install "x402-cli", target: "release/x402-cli"

    # Persist wallet directory
    (etc/"x402").mkpath
  end

  test do
    system "x402-cli", "--version"
  end
end
```

**Users can install:**

```bash
# Tap the repository
brew tap your-username/x402-cli

# Install
brew install x402-cli

# Use
x402-cli --help
```

## Pre-commit Integration

Add git hooks to ensure x402 configuration is valid:

### `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: check-x402-config
        name: Check x402.toml validity
        entry: bash -c '[[ -f config/x402.toml ]] && echo "✓ x402.toml exists"'
        language: system
        files: ^(x402\.toml|\.env\.)$
        pass_filenames:
          - x402.toml

      - id: check-wallet-dir
        name: Check wallet directory permissions
        entry: bash -c '[[ -d ~/.x402/wallets ]] && chmod 700 ~/.x402/wallets && echo "✓ Wallet directory secure"'
        language: system
        files: ^(.*)$

      - id: format-code
        name: Format Rust code
        entry: cargo fmt --
        language: rust
        files: \.rs$
```

## IDE Integration

### VS Code Extension

Create a `.vscode/launch.json` for debugging:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug x402-cli",
      "program": "${workspaceFolder}/target/debug/x402-cli",
      "args": [
        "init",
        "--name",
        "test-app",
        "--chain",
        "aptos",
        "--framework",
        "next"
      ],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "rust: cargo build"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Test Payment Flow",
      "program": "${workspaceFolder}/target/debug/x402-cli",
      "args": [
        "test",
        "payment",
        "--api",
        "http://localhost:3000/test",
        "--amount",
        "1000"
      ],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### VS Code Tasks

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "x402: Init Project",
      "type": "shell",
      "command": "cargo run -- init --name test-app --chain aptos --framework next",
      "problemMatcher": []
    },
    {
      "label": "x402: Create Wallet",
      "type": "shell",
      "command": "cargo run -- wallet create --network testnet",
      "problemMatcher": []
    },
    {
      "label": "x402: Start Facilitator",
      "type": "shell",
      "command": "cargo run -- facilitator start --port 3001",
      "problemMatcher": []
    },
    {
      "label": "x402: Test Payment",
      "type": "shell",
      "command": "cargo run -- test payment --api http://localhost:3000/test --amount 1000",
      "problemMatcher": []
    },
    {
      "label": "x402: Stop Facilitator",
      "type": "shell",
      "command": "cargo run -- facilitator stop",
      "problemMatcher": []
    },
    {
      "label": "cargo: Build",
      "type": "shell",
      "command": "cargo build --release",
      "problemMatcher": [
        "$rustc"
      ]
    }
  ]
}
```

## Monitoring & Logging

### Structured Logging

Set up structured logging for x402-cli:

```bash
# Enable debug logging
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Enable specific module logging
export RUST_LOG=x402_cli=debug,tokio=warn

# Log to file
RUST_LOG=x402-cli=info cargo run -- init --name test --chain aptos > x402.log 2>&1
```

### Log Monitoring

```bash
# Tail logs in real-time
tail -f x402.log

# Search logs for errors
grep -i "error" x402.log

# Search logs for wallet addresses
grep -i "0x" x402.log
```

## Testing Strategies

### Unit Testing

Add tests to the x402-cli modules:

```rust
// x402-cli/src/x402/project.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("test".to_string(), "aptos".to_string(), "next".to_string());
        assert_eq!(project.name, "test");
        assert_eq!(project.chain, "aptos");
        assert_eq!(project.framework, "next");
    }

    #[test]
    fn test_config_generation() {
        // Test configuration file generation
        // ...
    }
}
```

### Integration Testing

```bash
#!/bin/bash
# test-integration.sh

set -e

echo "Running integration tests..."

# Test 1: Project initialization
echo "Test 1: Initialize project..."
./target/release/x402-cli init --name integration-test --chain aptos --framework next
[[ -f integration-test/config/x402.toml ]] || { echo "✗ Failed: Config not created"; exit 1; }
echo "✓ Test 1 passed"

# Test 2: Wallet creation
echo "Test 2: Create wallet..."
./target/release/x402-cli wallet create --network testnet > /dev/null 2>&1
WALLET_FILE=$(ls -t ~/.x402/wallets/*.json | head -1)
[[ -f "$WALLET_FILE" ]] || { echo "✗ Failed: Wallet not created"; exit 1; }
echo "✓ Test 2 passed"

# Test 3: Facilitator operations
echo "Test 3: Start facilitator..."
./target/release/x402-cli facilitator start --port 3999 &
FACILITATOR_PID=$!
sleep 2
curl -s http://localhost:3999/health > /dev/null || { echo "✗ Failed: Facilitator not responding"; kill $FACILITATOR_PID; exit 1; }
./target/release/x402-cli facilitator stop
echo "✓ Test 3 passed"

# Cleanup
rm -rf integration-test

echo "All integration tests passed!"
```

### End-to-End Testing

```bash
#!/bin/bash
# test-e2e.sh

# Initialize project
./target/release/x402-cli init --name e2e-test --chain aptos --framework next
cd e2e-test

# Create test API
cat > src/index.js << 'EOF'
const express = require('express');
const app = express();

app.get('/api', (req, res) => {
  res.json({ message: 'Hello, World!' });
});

app.listen(3000, () => console.log('API running on port 3000'));
EOF

npm init -y
npm install express

# Start API in background
node src/index.js &
API_PID=$!
sleep 3

# Create wallet and test
./target/release/x402-cli wallet create --network testnet
./target/release/x402-cli test payment --api http://localhost:3000/api --amount 100

# Cleanup
kill $API_PID
cd ..
rm -rf e2e-test
```

## Performance Optimization

### Build Optimization

```bash
# Use LTO (Link Time Optimization)
RUSTFLAGS="-C link-arg=-s -C link-arg=-lto" cargo build --release

# Strip debug symbols
RUSTFLAGS="-C link-arg=-s" cargo build --release

# Optimize for size
cargo build --release --profile release-min-size
```

Add to `Cargo.toml`:

```toml
[profile.release-min-size]
inherits = "release"
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## Deployment Checklist

Before deploying x402-cli to production, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Version is bumped in Cargo.toml
- [ ] CHANGELOG.md is updated
- [ ] Release notes are prepared
- [ ] Binary is tested on target platforms
- [ ] Installation instructions are verified

## Quick Reference

### Common Commands

```bash
# Initialize project
x402-cli init --name <name> --chain <chain> --framework <framework>

# Wallet operations
x402-cli wallet create --network <network>
x402-cli wallet list    # Future feature
x402-cli wallet import <path>  # Future feature

# Facilitator operations
x402-cli facilitator start --port <port>
x402-cli facilitator stop
x402-cli facilitator status   # Future feature

# Testing
x402-cli test payment --api <url> --amount <amount>
x402-cli test health   # Future feature

# Deployment
x402-cli deploy --provider <provider>
x402-cli deploy --production   # Future enhancement
```

### File Locations

| Component | Location |
|-----------|----------|
| CLI Binary | `./target/release/x402-cli` |
| Wallets | `~/.x402/wallets/` |
| Project Config | `<project>/config/x402.toml` |
| Environment | `<project>/.env` |
| Logs | `~/.local/state/x402-cli/log` (if configured) |

---

**For more information**, see the main [README.md](./README.md) documentation.
