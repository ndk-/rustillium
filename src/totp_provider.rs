use anyhow::{Context, Result};
use totp_rs::TOTP;

pub struct TOTPDisplay {
    pub code: String,
    pub remaining_seconds: u64,
}

pub fn generate_totp_display_info(url: &str) -> Result<TOTPDisplay> {
    let totp = TOTP::from_url(url).context("Failed to parse TOTP URL")?;
    
    let code = totp.generate_current().context("Failed to generate TOTP code")?;
    let remaining_seconds = totp.ttl().unwrap_or(0); // pragmatic error handling for very rare edge case
    
    Ok(TOTPDisplay { code, remaining_seconds })
}
