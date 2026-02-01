use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub chain: String,
    pub framework: String,
    pub version: String,
}

impl Project {
    pub fn new(name: String, chain: String, framework: String) -> Self {
        let version = "0.1.0".to_string();
        Project {
            name,
            chain,
            framework,
            version,
        }
    }

    pub fn create_directories(&self) -> Result<()> {
        let base_dir = PathBuf::from(&self.name);

        let dirs = vec![
            base_dir.join("src"),
            base_dir.join("config"),
            base_dir.join("tests"),
            base_dir.join("docs"),
        ];

        for dir in dirs {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
        }

        println!(
            "{}",
            format!("  ✓ Created directories for {}", self.name.green()).dimmed()
        );
        Ok(())
    }

    pub fn create_config_files(&self) -> Result<()> {
        let base_dir = PathBuf::from(&self.name);
        let config_dir = base_dir.join("config");

        let config_content = format!(
            r#"# x402 Configuration
project_name = "{}"
chain = "{}"
framework = "{}"
version = "{}"

[server]
port = 3000
host = "localhost"

[blockchain]
network = "{}"

[facilitator]
enabled = true
port = 3001
"#,
            self.name, self.chain, self.framework, self.version, self.chain
        );

        fs::write(config_dir.join("x402.toml"), config_content)
            .with_context(|| format!("Failed to create config file"))?;

        let env_content = format!(
            r#"# x402 Environment Variables
NODE_ENV=development
X402_CHAIN={}
X402_PROJECT={}
"#,
            self.chain, self.name
        );

        fs::write(base_dir.join(".env.example"), env_content)
            .with_context(|| format!("Failed to create .env.example"))?;

        let gitignore_content = r#"# Dependencies
node_modules/
target/

# Environment
.env

# Logs
*.log
npm-debug.log*

# IDE
.vscode/
.idea/

# Build
dist/
build/"#;

        fs::write(base_dir.join(".gitignore"), gitignore_content)
            .with_context(|| format!("Failed to create .gitignore"))?;

        println!("{}", "  ✓ Created configuration files".dimmed());
        Ok(())
    }

    pub fn install_dependencies(&self) -> Result<()> {
        match self.framework.to_lowercase().as_str() {
            "next" | "nextjs" => {
                let output = Command::new("npm")
                    .args(["init", "-y"])
                    .current_dir(&self.name)
                    .output()
                    .context("Failed to run npm init")?;

                if output.status.success() {
                    println!("{}", "  ✓ Installed Node.js dependencies".dimmed());
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    println!(
                        "{}",
                        format!("  ⚠ npm init warning: {}", error.trim())
                            .yellow()
                            .dimmed()
                    );
                }
            }
            "react" => {
                println!(
                    "{}",
                    "  ⚠ React: Run `npm install` after project creation"
                        .yellow()
                        .dimmed()
                );
            }
            _ => {
                println!(
                    "{}",
                    "  ℹ Custom framework: Install dependencies manually".dimmed()
                );
            }
        }

        Ok(())
    }

    pub fn generate_readme(&self) -> Result<()> {
        let readme_content = format!(
            r#"# {} - {} Framework

An x402-enabled API built on {} blockchain.

## Features

- Payment-enabled API endpoints
- Automated wallet management
- Development facilitator integration

## Getting Started

```bash
# Install dependencies (if applicable)
npm install

# Copy environment variables
cp .env.example .env

# Run the development server
npm run dev

# Start the facilitator
x402 facilitator start
```

## Configuration

See `config/x402.toml` for project configuration.

## x402 CLI Commands

```bash
# Initialize new x402-enabled API
x402 init my-weather-api --chain aptos --framework next

# Create a test wallet
x402 wallet create --network testnet

# Start local facilitator for testing
x402 facilitator start --port 3001

# Test a payment flow end-to-end
x402 test payment --api http://localhost:3000/weather --amount 1000

# Deploy to production
x402 deploy --provider vercel
```

## Documentation

See the `docs/` directory for additional documentation.
"#,
            self.name, self.framework, self.chain
        );

        fs::write(format!("{}/README.md", self.name), readme_content)
            .with_context(|| format!("Failed to create README.md"))?;

        println!("{}", "  ✓ Generated README.md".dimmed());
        Ok(())
    }
}
