# Rustillium
Rustillium is a GUI-based credential manager written in Rust that helps you store, manage, and version your encrypted secrets.

## Features
*   **Secure Credential Management:** Store, retrieve, and manage your sensitive information with robust encryption.
*   **Intuitive Graphical User Interface (GUI):** Easily interact with your secrets through a user-friendly interface.
*   **Comprehensive Secret Operations:** Create, view, search, modify, rename, and delete secrets.
*   **Clipboard Integration:** Quickly copy secret values to your clipboard for convenience.
*   **Automatic Version Control:** All changes to your secrets are automatically tracked and versioned using Git, providing a complete history and enabling future recovery.
*   **Flexible Configuration:** Customize the location of your encrypted secrets and your GPG recipient email via configuration files or environment variables.

## Stories
I keep track of most of my work in stories (not all) that are located in `stories` directory

## Requirements
I built this on Gentoo Linux. This app is currently guaranteed to work on a Linux distribution that has the following:
- rust with cargo (tested on 1.85 up to 1.88)
- gpgme (tested on 1.24) - *Required for encrypting and decrypting your secrets.*
- git (tested on 2.49.1) - *Required for automatic version control and history tracking of your secrets.*

# Steps to compile/run
1. Check `secrets` folder for examples of secret files and their format. The secrets are stored in TOML format.
1. As an example, encrypt one or more files from `secrets` folder using `gpg`
    - an example command to encrypt `gpg -r recipient@email.com -e bank.toml`
1. Once encrypted, move encrypted files to `enc` folder (make sure they have gpg extension)
1. Either compile rust binary or just run `cargo run`
1. You should see the encrypted files and rustillium should be able to decrypt them using `gpg` when clicked.

# Configuration
There are 2 configuration pieces that are needed:
1. The directory with the encrypted secrets:
    1. By default, it will use `./enc` directory to look for secrets
    1. Via configuration file located at: `$HOME/.config/rustillium/config.toml` with `secrets_directory` defined
    1. Via environment variable: `RUSTILLIUM_SECRETS_DIRECTORY`
1. Recipient email for GPG to be able to encrypt new/existing secrets:
    1. By default, there is no default value, the application will not start.
    1. Via configuration file located at: `$HOME/.config/rustillium/config.toml` with `recipient_email` defined
    1. Via environment variable: `RUSTILLIUM_RECIPIENT_EMAIL`
