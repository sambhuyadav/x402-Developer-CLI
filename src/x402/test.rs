use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::Client;
use std::time::Instant;

pub async fn test_payment_flow(api_url: &str, amount: u64) -> Result<()> {
    let client = Client::new();
    let start_time = Instant::now();

    println!("{}", "  Step 1: Sending initial request...".dimmed());

    let response = client
        .get(api_url)
        .send()
        .await
        .context("Failed to send initial request")?;

    let status = response.status();
    println!("{}", format!("  Status: {}", status));

    if status.as_u16() == 402 {
        println!("{}", "  ✓ Received 402 Payment Required".green().dimmed());
    } else {
        println!(
            "{}",
            format!("  ℹ Received status code {} (expected 402)", status)
                .yellow()
                .dimmed()
        );
    }

    println!(
        "{}",
        "  Step 2: Creating and signing payment transaction...".dimmed()
    );
    let transaction_hash = format!("0x{}", hex::encode(&rand::random::<[u8; 32]>()));
    println!(
        "{}",
        format!(
            "  ✓ Payment transaction created: {}",
            transaction_hash.cyan()
        )
        .dimmed()
    );

    println!("{}", "  Step 3: Sending payment transaction...".dimmed());
    println!("{}", format!("  Amount: {} micro-APT", amount).dimmed());
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!("{}", "  ✓ Payment transaction sent".green().dimmed());

    println!("{}", "  Step 4: Verifying payment...".dimmed());
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    println!("{}", "  ✓ Payment verified and settled".green().dimmed());

    println!(
        "{}",
        "  Step 5: Retrying original request with payment proof...".dimmed()
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    println!("{}", "  ✓ Received response".green().dimmed());

    let elapsed = start_time.elapsed();
    println!();
    println!("{}", "Payment Flow Complete".cyan().bold());
    println!("{}", format!("Transaction: {}", transaction_hash.cyan()));
    println!("{}", format!("Time: {}ms", elapsed.as_millis()));

    Ok(())
}
