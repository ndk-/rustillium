use toml::value::Table;
use std::fs;

// Function to load and parse the credential data from the TOML file
pub fn load_credential_data(path: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let data: Table = toml::from_str(&contents)?;
    Ok(data)
}
