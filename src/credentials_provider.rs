use std::collections::HashMap;
use std::error;
use std::fs;
use std::path::Path;
use toml;

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
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "toml"))
            .filter_map(|entry| entry.path().file_stem().and_then(|stem| stem.to_str()).map(|stem| stem.to_string()))
            .collect();

        Ok(secret_names)
    }

    pub fn load_secrets(self: &Self, secret_name: &str) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
        let contents = fs::read_to_string(
            Path::new(&self.directory_name).join(
                format!("{}.toml", secret_name)
            )
        )?;

        Ok(toml::from_str(&contents)?)
    }
}
