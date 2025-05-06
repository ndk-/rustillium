use std::collections::HashMap;
use std::error;
use std::fs;
use std::path::Path;
use toml;
use gpgme::{Context, Protocol};

pub struct CredentialsProvider {
    directory_name: String
}

impl CredentialsProvider {
    pub fn new(directory_name: &str) -> Self {
        Self { directory_name: directory_name.to_string() }
    }

    pub fn load_secret_names(self: &Self) -> Result<Vec<String>, Box<dyn error::Error>> {
        
        let file_names = fs::read_dir(Path::new(&self.directory_name))?;
        let mut directory_entries = (file_names.collect::<Result<Vec<_>, _>>())?;
        directory_entries.sort_by_key(|dir| dir.path());

        let secret_names: Vec<String> = directory_entries.iter()
            .filter(|entry| entry.path().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "gpg"))
            .filter_map(|entry| entry.path().file_stem().and_then(|stem| stem.to_str()).map(|stem| stem.to_string()))
            .collect();

        Ok(secret_names)
    }

    pub fn load_secrets(self: &Self, secret_name: &str) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
        let mut context = Context::from_protocol(Protocol::OpenPgp)?;
        let mut secrets_file = fs::File::open(Path::new(&self.directory_name).join(
            format!("{}.gpg", secret_name)
        ))?;

        let mut secrets_bytes = Vec::new();
        context.decrypt(&mut secrets_file, &mut secrets_bytes).map_err(|err| format!("Error while decrypting {}", err))?;


        let secrets_content = String::from_utf8(secrets_bytes)?;
        Ok(toml::from_str(secrets_content.as_str())?)
    }
}
