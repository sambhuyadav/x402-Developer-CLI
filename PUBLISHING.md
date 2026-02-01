# Publishing to crates.io Guide

This guide explains how to publish x402-cli to crates.io.

## Prerequisites

1. **Create a crates.io account**
   - Go to https://crates.io
   - Click "Log in with GitHub" or create an account
   - Verify your email

2. **Get an API token**
   - Go to https://crates.io/settings/tokens
   - Click "New API token"
   - Name it something like "x402-cli-publish"
   - Select "Publish new crates and versions" scope
   - Copy the token (you won't see it again!)

3. **Login to cargo**
   ```bash
   cargo login
   ```
   - Paste your API token when prompted
   - Store the token in `~/.cargo/credentials.toml`

## Publishing Steps

### 1. Ensure all changes are committed
```bash
git status
# If there are uncommitted changes:
git add .
git commit -m "chore: prepare for crates.io publication"
```

### 2. Run cargo publish
```bash
cargo publish
```

This will:
- Package the crate
- Upload to crates.io
- Verify the package compiles on their servers
- Make it available at https://crates.io/crates/x402-cli

## Installation for Users

Once published, users can install with:

```bash
# Using cargo
cargo install x402-cli

# Then use it
x402-cli --help
```

## Troubleshooting

### "crate is already uploaded"
If you get an error that the crate exists:
- Check if version exists: https://crates.io/crates/x402-cli/versions
- If version 1.0.0 exists, bump to 1.0.1 in Cargo.toml
- Commit and try again

### "version ... is already uploaded"
This means the same version exists:
- Update version in Cargo.toml
- Run `cargo publish` again

### API token not working
If cargo login fails:
- Delete `~/.cargo/credentials.toml`
- Run `cargo login` again
- Use a fresh API token

## After Publication

Your crate will be available at:
- https://crates.io/crates/x402-cli
- https://docs.rs/x402-cli (auto-generated documentation)

## CI/CD Setup (Optional)

You can automate future releases with GitHub Actions:

```yaml
# .github/workflows/publish.yml
name: Publish to crates.io

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable
      - uses: katyo/publish-crates@v2
        with:
          token: ${{ secrets.CRATES_IO_TOKEN }}
```

Set `CRATES_IO_TOKEN` in your GitHub repository secrets.
