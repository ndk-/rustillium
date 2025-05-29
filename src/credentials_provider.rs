use gpgme::{Context, Protocol};
use std::collections::HashMap;
use std::error;
use std::fs;
use std::path::PathBuf;
use toml;

pub struct CredentialsProvider {
    path: PathBuf,
}

impl CredentialsProvider {
    pub fn new(directory_name: &str) -> Self {
        Self { path: PathBuf::from(directory_name) }
    }

    pub fn load_secret_names(self: &Self) -> Result<Vec<String>, Box<dyn error::Error>> {
        let file_names = fs::read_dir(&self.path)?;

        let mut secret_names: Vec<String> = file_names
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "gpg"))
            .filter_map(|entry| entry.path().file_stem().and_then(|stem| stem.to_str()).map(|stem| stem.to_string()))
            .collect();

        secret_names.sort();
        Ok(secret_names)
    }

    pub fn load_secrets(self: &Self, secret_name: &str) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
        let mut context = Context::from_protocol(Protocol::OpenPgp)?;
        let mut secrets_file = fs::File::open(self.path.join(format!("{}.gpg", secret_name)))?;
        let mut secrets_bytes = Vec::new();
        context.decrypt(&mut secrets_file, &mut secrets_bytes)?;

        let secrets_content = String::from_utf8(secrets_bytes)?;
        Ok(toml::from_str(secrets_content.as_str())?)
    }
}
