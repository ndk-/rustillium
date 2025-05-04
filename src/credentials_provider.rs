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
        let mut secret_names = Vec::new();
        let file_names = fs::read_dir(Path::new(&self.directory_name))?;
        let mut entries = (file_names.collect::<Result<Vec<_>, _>>())?;
        entries.sort_by_key(|dir| dir.path());

        for entry in entries {
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "toml" {
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(secret_name) = file_stem.to_str() {
                                secret_names.push(secret_name.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(secret_names)
    }

    pub fn load_secrets(self: &Self, secret_name: &str) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
        let file_path = Path::new(&self.directory_name).join(format!("{}.toml", secret_name));
        
        let contents = fs::read_to_string(file_path)?;

        let data: HashMap<String, String> = toml::from_str(&contents)?;

        Ok(data)
    }
}
