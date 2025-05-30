# Rustillium

## Stories
I keep track of most of my work in stories (not all) that are located in `stories` directory

## Requirements
I built this on Gentoo Linux. This app is currently will only work on a Linux distribution that has the following:
- rust with cargo (tested on 1.85)
- gpgme (tested on 1.24)

# Steps to compile/run
1. Check `secrets` folder for examples of secret files and their format
1. As an example, encrypt one or more files from `secrets` folder using `gpg`
    - an example command to encrypt `gpg -r recipient@email.com -e bank.toml`
1. Once encrypted, move encrypted files to `enc` folder (make sure they have gpg extension)
1. Either compile rust binary or just run `cargo run`
1. You should see the encrypted files and rustillium should be able to decrypt them using `gpg` when clicked.

# Configuration
There are 3 ways to provide the directory with encrypted secrets with the corresponding precedence (from least to most):
1. By default, it will use `./enc` directory to look for secrets
1. Via configuration file located at: `$HOME/.config/rustillium/config.toml` with `secrets_directory` defined
1. Via environment variable: `RUSTILLIUM_SECRETS_DIRECTORY`
