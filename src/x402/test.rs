use anyhow::{Context, Result};
use base64::prelude::*;
use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;

use base64::engine::general_purpose::STANDARD as Engine;

const FACILITATOR_URL: &str = "http://localhost:3001";

#[derive(Serialize, Clone)]
pub struct PaymentPayload {
    pub x402Version: u32,
    pub accepted: PaymentRequirements,
    pub payload: Payload,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct PaymentRequirements {
    pub scheme: String,
    pub network: String,
    pub amount: String,
    pub asset: String,
    pub payTo: String,
    #[serde(flatten)]
    pub extra: Option<Extra>,
}

#[derive(Serialize, Clone, Deserialize, Default)]
pub struct Extra {
    #[serde(default)]
    pub sponsored: Option<bool>,
}

#[derive(Serialize, Clone)]
pub struct Payload {
    pub transaction: String,
    pub senderAuthenticator: String,
}

#[derive(Deserialize)]
struct VerifyResponse {
    pub isValid: bool,
    pub invalidReason: Option<String>,
    pub payer: Option<String>,
}

#[derive(Deserialize)]
struct SettleResponse {
    pub success: bool,
    pub transaction: String,
    pub network: String,
    pub payer: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    pub error: Option<String>,
}

pub async fn test_payment_flow(api_url: &str, _amount: u64) -> Result<()> {
    let client = Client::new();
    let start_time = Instant::now();

    let step1_msg = "  Step 1: Sending initial request...".dimmed();
    println!("{}", step1_msg);

    let response = client
        .get(api_url)
        .send()
        .await
        .context("Failed to send initial request")?;

    let status = response.status();
    println!("{}", format!("  Status: {}", status));

    if status.as_u16() != 402 {
        let status_str = format!("{}", status);
        println!("  ℹ Expected 402, got {}", status_str);
        println!("  ℹ Note: For real x402 testing, API must return 402 Payment Required");
        return Ok(());
    }

    println!("  ✓ Received 402 Payment Required");

    let payment_required_header = response
        .headers()
        .get("PAYMENT-REQUIRED")
        .context("Missing PAYMENT-REQUIRED header")?;

    let header_str = payment_required_header.to_str()?;
    let decoded_bytes = Engine.decode(header_str)
        .map_err(|e| anyhow::anyhow!("Failed to decode PAYMENT-REQUIRED header: {}", e))?;

    let requirements_str = String::from_utf8(decoded_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to convert decoded bytes to UTF-8: {}", e))?;
    let requirements: PaymentRequirements = serde_json::from_str(&requirements_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse PaymentRequirements: {}", e))?;

    println!(
        "{}",
        format!("  Payment Requirements: {} {} to {}",
                 requirements.amount.dimmed().cyan(), requirements.asset, requirements.payTo.dimmed())
    );

    println!("{}", "  Step 2: Building payment payload...".dimmed());

    let random_bytes: [u8; 32] = rand::random();
    let transaction_hash = format!("0x{}", hex::encode(random_bytes));
    let transaction_bytes = vec![0u8; 64];
    let sender_authenticator_bytes = vec![0u8; 64];

    let payload = Payload {
        transaction: Engine.encode(transaction_bytes.as_slice()),
        senderAuthenticator: Engine.encode(sender_authenticator_bytes.as_slice()),
    };

    let payment_payload = PaymentPayload {
        x402Version: 2,
        accepted: requirements.clone(),
        payload,
    };

    println!("{}", format!("  Transaction Hash: {}", transaction_hash.cyan()));

    println!("{}", "  Step 3: Verifying payment with facilitator...".dimmed());

    let verify_request = json!({
        "paymentPayload": payment_payload,
        "paymentRequirements": requirements
    });

    let verify_response = client
        .post(&format!("{}/verify", FACILITATOR_URL))
        .header("Content-Type", "application/json")
        .json(&verify_request)
        .send()
        .await
        .context("Failed to verify payment")?;

    if !verify_response.status().is_success() {
        let error_text = verify_response.text().await.unwrap_or_default();
        println!("{}", format!("  ⚠ Verification failed: {}", error_text).dimmed().yellow());
        return Ok(());
    }

    let verify_result: VerifyResponse = verify_response.json().await
        .context("Failed to parse verify response")?;

    if !verify_result.isValid {
        let reason = verify_result.invalidReason.unwrap_or_else(|| "Unknown".to_string());
        println!("{}", format!("  ✗ Payment invalid: {}", reason.bold().red()));
        return Ok(());
    }

    println!("{}", "  ✓ Payment verified".dimmed().green());

    println!("{}", "  Step 4: Settling payment with facilitator...".dimmed());

    let settle_response = client
        .post(&format!("{}/settle", FACILITATOR_URL))
        .header("Content-Type", "application/json")
        .json(&verify_request)
        .send()
        .await
        .context("Failed to settle payment")?;

    if !settle_response.status().is_success() {
        let error_text = settle_response.text().await.unwrap_or_default();
        println!("{}", format!("  ⚠ Settlement failed: {}", error_text).dimmed().yellow());
        return Ok(());
    }

    let settle_result: SettleResponse = settle_response.json().await
        .context("Failed to parse settle response")?;

    if !settle_result.success {
        println!("{}", "  ✗ Settlement failed".bold().red());
        return Ok(());
    }

    println!("{}", "  ✓ Payment settled".dimmed().green());
    println!("{}", format!("  Transaction: {}", settle_result.transaction.cyan()));
    println!("{}", format!("  Payer: {}", settle_result.payer.cyan()));

    println!("{}", "  Step 5: Retrying original request with payment proof...".dimmed());

    let payload_bytes = serde_json::to_vec(&payment_payload)
        .map_err(|e| anyhow::anyhow!("Failed to serialize payment payload: {}", e))?;
    let payment_signature = Engine.encode(&payload_bytes);

    let final_response = client
        .get(api_url)
        .header("PAYMENT-SIGNATURE", payment_signature)
        .send()
        .await
        .context("Failed to send final request")?;

    if final_response.status().is_success() {
        println!("{}", "  ✓ Received response from API".bold().green());
    } else {
        let final_status = final_response.status();
        println!("{}", format!("  ℹ API returned: {}", final_status).dimmed().yellow());
    }

    let elapsed = start_time.elapsed();
    println!();
    println!("{}", "Payment Flow Complete".cyan().bold());
    println!("{}", format!("Transaction: {}", settle_result.transaction.cyan()));
    println!("{}", format!("Payer: {}", settle_result.payer.cyan()));
    println!("{}", format!("Time: {}ms", elapsed.as_millis()));

    Ok(())
}
