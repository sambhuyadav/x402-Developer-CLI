use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

pub async fn deploy(provider: &str) -> Result<()> {
    match provider.to_lowercase().as_str() {
        "vercel" | "vercel.app" => deploy_to_vercel().await,
        _ => {
            println!(
                "{}",
                format!("  ⚠ Provider '{}' not yet supported", provider)
                    .yellow()
                    .dimmed()
            );
            println!("{}", "  Supported providers: vercel".dimmed());
            Ok(())
        }
    }
}

async fn deploy_to_vercel() -> Result<()> {
    println!("{}", "  Step 1: Building facilitator...".dimmed());

    let build_result = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .context("Failed to build project")?;

    if !build_result.status.success() {
        let error = String::from_utf8_lossy(&build_result.stderr);
        anyhow::bail!("Build failed: {}", error);
    }

    println!("{}", "  ✓ Build successful".green().dimmed());

    println!(
        "{}",
        "  Step 2: Checking for vercel installation...".dimmed()
    );

    let check_result = Command::new("vercel").arg("--version").output();

    match check_result {
        Ok(output) if output.status.success() => {
            println!("{}", "  ✓ Vercel CLI installed".green().dimmed());
        }
        _ => {
            println!(
                "{}",
                "  ℹ Vercel CLI not found. Installing...".yellow().dimmed()
            );
            println!("{}", "  Run: npm install -g vercel".dimmed());
            return Ok(());
        }
    }

    println!("{}", "  Step 3: Deploying facilitator...".dimmed());

    let deploy_result = Command::new("vercel")
        .args(["--prod"])
        .output()
        .context("Failed to execute vercel deploy")?;

    let _output = String::from_utf8_lossy(&deploy_result.stdout);

    if deploy_result.status.success() {
        println!("{}", "  ✓ Deployment initiated".green().dimmed());
        println!(
            "{}",
            "  Check Vercel dashboard for deployment status".dimmed()
        );
        println!("{}", "  Typically: https://vercel.com/dashboard".dimmed());
    } else {
        let error = String::from_utf8_lossy(&deploy_result.stderr);
        println!(
            "{}",
            format!("  ⚠ Deployment may have failed: {}", error)
                .yellow()
                .dimmed()
        );
    }

    println!();
    println!("{}", "Deployment Summary".cyan().bold());
    println!(
        "{}",
        "  Follow the Vercel CLI prompts to complete deployment".dimmed()
    );
    println!(
        "{}",
        "  Your facilitator will be deployed with a public URL".dimmed()
    );

    Ok(())
}
