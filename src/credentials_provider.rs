use anyhow::{anyhow, Context, Result};
use git2::IndexAddOption;
use git2::Repository;
use git2::Signature;
use gpgme::{Context as GpgmeContext, Key, Protocol};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use toml;

#[derive(Clone)]
pub struct CredentialsProvider {
    path: PathBuf,
    recipient_email: String,
    repository: Rc<Repository>,
}

impl CredentialsProvider {
    pub fn new(directory_name: &str, recipient_email: &str) -> Self {
        let path = PathBuf::from(directory_name);
        let repository = Self::initialize_repository(&path).expect("Fatal: Failed to open or initialize secrets repository.");
        Self {
            path,
            recipient_email: recipient_email.to_string(),
            repository: Rc::new(repository)
        }
    }

    fn initialize_repository(path: &PathBuf) -> Result<Repository> {
        Ok(Repository::open(path).or_else(|_| Repository::init(path))?)
    }

    fn commit(&self, message: &str) -> Result<()> {
        let parent_commit = self.get_parent_commit()?;

        let tree_id = {
            let mut index = self.repository.index()?;
            index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
            index.write()?;
            index.write_tree()?
        };

        let tree = self.repository.find_tree(tree_id)?;
        let author = Signature::now("Rustillium", "rustillium@app.local")?;

        let parents = parent_commit.map(|c| vec![c]).unwrap_or_default();
        let parent_commits_refs: Vec<&git2::Commit> = parents.iter().collect();

        self.repository.commit(Some("HEAD"), &author, &author, message, &tree, parent_commits_refs.as_slice())?;
        Ok(())
    }


    fn get_parent_commit(&self) -> Result<Option<git2::Commit<'_>>> {
        match self.repository.head() {
            Ok(head) => {
                if let Some(oid) = head.target() {
                    Ok(Some(self.repository.find_commit(oid)?))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                if e.code() == git2::ErrorCode::UnbornBranch || e.code() == git2::ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(e).map_err(|e| e.into())
                }
            }
        }
    }

    pub fn load_secret_names(self: &Self) -> Result<Vec<String>> {
        let file_names = fs::read_dir(&self.path)?;

        let mut secret_names: Vec<String> = file_names
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "gpg"))
            .filter_map(|entry| entry.path().file_stem().and_then(|stem| stem.to_str()).map(|stem| stem.to_string()))
            .collect();

        secret_names.sort();
        Ok(secret_names)
    }

    pub fn load_secrets(self: &Self, secret_name: &str) -> Result<HashMap<String, String>> {
        let file_path = self.path.join(format!("{}.gpg", secret_name));
        let mut context = GpgmeContext::from_protocol(Protocol::OpenPgp)?;
        let mut secrets_file = fs::File::open(&file_path).context(format!("Failed to open secret file {:?}", file_path))?;
        let mut secrets_bytes = Vec::new();
        context.decrypt(&mut secrets_file, &mut secrets_bytes).context("Failed to decrypt GPG content")?;

        let secrets_content = String::from_utf8(secrets_bytes).context("Decrypted content is not valid UTF-8")?;
        toml::from_str(secrets_content.as_str()).context("Failed to parse TOML from decrypted secret")
    }

    fn save_secret(&self, secret_name: &str, secrets: &HashMap<String, String>) -> Result<()> {
        let toml_string = toml::to_string(secrets)?;
        let mut context = GpgmeContext::from_protocol(Protocol::OpenPgp)?;

        let recipients: Vec<Key> = context.find_keys([self.recipient_email.as_str()])?
            .filter_map(Result::ok).collect();

        let recipient = recipients.get(0).ok_or_else(|| anyhow!("Recipient GPG key for '{}' not found.", self.recipient_email))?;

        let mut ciphertext = Vec::new();
        context.encrypt(Some(recipient), toml_string.as_bytes(), &mut ciphertext)?;

        fs::write(self.path.join(format!("{}.gpg", secret_name)), ciphertext)?;

        Ok(())
    }

    pub fn update_secret(&self, original_name: Option<&str>, new_name: &str, secrets_data: &HashMap<String, String>) -> Result<()> {
        let new_path = self.path.join(format!("{}.gpg", new_name));
        let is_renaming = original_name.is_some() && original_name.unwrap() != new_name;
        let is_creating = original_name.is_none();

        if is_renaming || is_creating {
            if new_path.exists() {
                return Err(anyhow!("A secret with the name '{}' already exists.", new_name));
            }
        }

        self.save_secret(new_name, secrets_data)?;

        if is_renaming {
            let old_path = self.path.join(format!("{}.gpg", original_name.unwrap()));
            fs::remove_file(old_path)?;
        }

        self.commit_on_update(original_name, new_name)?;

        Ok(())
    }

    fn commit_on_update(&self, original_name: Option<&str>, new_name: &str) -> Result<()> {
        let is_renaming = original_name.is_some() && original_name.unwrap() != new_name;
        let is_creating = original_name.is_none();

        let message = if is_creating {
            format!("Create secret: {}", new_name)
        } else if is_renaming {
            format!("Rename secret from {} to {}", original_name.unwrap(), new_name)
        } else {
            format!("Update secret: {}", new_name)
        };
        self.commit(&message)?;
        Ok(())
    }

    pub fn delete_secret(&self, secret_name: &str) -> Result<()> {
        let path = self.path.join(format!("{}.gpg", secret_name));
        fs::remove_file(path)?;
        self.commit(&format!("Delete secret: {}", secret_name))?;
        Ok(())
    }
}
