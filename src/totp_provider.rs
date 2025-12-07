use totp_rs::TOTP;
use std::error;

pub struct TOTPDisplay {
    pub code: String,
    pub remaining_seconds: u64,
}

pub fn generate_totp_display_info(url: &str) -> Result<TOTPDisplay, Box<dyn error::Error>> {
    let totp = TOTP::from_url(url).expect("Invalid TOTP URL format");
    
    let code = totp.generate_current().expect("Failed to generate TOTP code");
    let remaining_seconds = totp.ttl().expect("Cannot find time to live");
    
    Ok(TOTPDisplay { code, remaining_seconds })
}
