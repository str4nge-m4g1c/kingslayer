# Setup Guide for Releases and Homebrew

This guide explains how to create your first release and set up Homebrew installation.

## Creating Your First Release

### 1. Prepare the Release

Ensure your code is ready and all tests pass:

```bash
cargo test
cargo clippy
cargo build --release
```

### 2. Tag the Release

```bash
# Create and push a version tag
git tag -a v0.1.0 -m "Initial release - Solo play mode"
git push origin v0.1.0
```

### 3. GitHub Actions Will Automatically:
- Build binaries for all platforms (macOS x86_64, macOS ARM64, Linux x86_64, Windows x86_64)
- Create a GitHub Release
- Upload all platform binaries as release assets

### 4. Verify the Release

Go to `https://github.com/str4nge-m4g1c/regicide-tui/releases` and verify:
- The release was created
- All platform binaries are attached
- The release notes look correct

## Setting Up Homebrew Tap

### 1. Create the Tap Repository

```bash
# Create a new repository on GitHub named: homebrew-kingslayer
# Clone it locally
git clone https://github.com/str4nge-m4g1c/homebrew-kingslayer.git
cd homebrew-kingslayer
```

### 2. Set Up the Formula

```bash
# Create the Formula directory
mkdir -p Formula

# Copy the formula from this repo
cp ../regicide-tui/homebrew/kingslayer.rb Formula/

# Initial commit
git add Formula/kingslayer.rb
git commit -m "Add kingslayer formula"
git push origin main
```

### 3. Calculate SHA256 Hashes

After the release is created, download each binary and calculate its SHA256:

```bash
# Download the release binaries
curl -L -O https://github.com/str4nge-m4g1c/regicide-tui/releases/download/v0.1.0/kingslayer-macos-aarch64.tar.gz
curl -L -O https://github.com/str4nge-m4g1c/regicide-tui/releases/download/v0.1.0/kingslayer-macos-x86_64.tar.gz
curl -L -O https://github.com/str4nge-m4g1c/regicide-tui/releases/download/v0.1.0/kingslayer-linux-x86_64.tar.gz

# Calculate SHA256 for each
shasum -a 256 kingslayer-macos-aarch64.tar.gz
shasum -a 256 kingslayer-macos-x86_64.tar.gz
shasum -a 256 kingslayer-linux-x86_64.tar.gz
```

### 4. Update the Formula with Real Hashes

Edit `Formula/kingslayer.rb` and replace the placeholder SHA256 values with the real ones:

```ruby
# Replace:
sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_ARM64_MACOS"

# With the actual hash:
sha256 "abc123def456..."
```

### 5. Commit and Push

```bash
git add Formula/kingslayer.rb
git commit -m "Update SHA256 hashes for v0.1.0"
git push origin main
```

### 6. Test the Installation

```bash
# Add your tap
brew tap str4nge-m4g1c/kingslayer

# Install kingslayer
brew install kingslayer

# Test it works
kingslayer
```

## Updating for New Releases

For each new release:

1. **Update version in Cargo.toml**
   ```toml
   version = "0.2.0"
   ```

2. **Create and push the tag**
   ```bash
   git tag -a v0.2.0 -m "Version 0.2.0 - Description of changes"
   git push origin v0.2.0
   ```

3. **Wait for GitHub Actions** to build and create the release

4. **Update the Homebrew formula** in `homebrew-kingslayer`:
   - Update version number
   - Download new binaries
   - Calculate new SHA256 hashes
   - Update the formula
   - Commit and push

5. **Users update** with:
   ```bash
   brew update
   brew upgrade kingslayer
   ```

## Automated Homebrew Updates (Optional)

You can use `brew bump-formula-pr` to automatically update:

```bash
brew bump-formula-pr \
  --url=https://github.com/str4nge-m4g1c/regicide-tui/archive/v0.2.0.tar.gz \
  --sha256=$(curl -sL https://github.com/str4nge-m4g1c/regicide-tui/archive/v0.2.0.tar.gz | shasum -a 256 | cut -d' ' -f1) \
  str4nge-m4g1c/kingslayer/kingslayer
```

## Troubleshooting

### GitHub Actions Failing

- Check the Actions tab in your GitHub repository
- Ensure all platforms can build successfully
- Check that Cargo.toml is valid

### Homebrew Installation Issues

- Verify the formula syntax: `brew audit --strict kingslayer`
- Test locally: `brew install --build-from-source kingslayer`
- Check the formula test: `brew test kingslayer`

### SHA256 Mismatch

- Ensure you're calculating the hash of the `.tar.gz` file, not the extracted binary
- Verify the download completed successfully (check file size)
- Recalculate: `shasum -a 256 filename.tar.gz`

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Creating Homebrew Taps](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
