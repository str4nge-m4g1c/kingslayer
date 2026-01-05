# Homebrew Formula for Kingslayer

This directory contains the Homebrew formula for installing Kingslayer via Homebrew.

## Setting Up the Tap

To make this formula available via `brew tap`, you need to create a separate repository called `homebrew-kingslayer`.

### Steps:

1. **Create a new GitHub repository** named `homebrew-kingslayer` (the name must start with `homebrew-`)

2. **Copy the formula** to the tap repository:
   ```bash
   # In your homebrew-kingslayer repository
   mkdir -p Formula
   cp kingslayer.rb Formula/
   ```

3. **Update SHA256 hashes** after creating a release:
   ```bash
   # Download the release artifacts
   # For each platform, calculate the SHA256:
   shasum -a 256 kingslayer-macos-aarch64.tar.gz
   shasum -a 256 kingslayer-macos-x86_64.tar.gz
   shasum -a 256 kingslayer-linux-x86_64.tar.gz

   # Update the hashes in Formula/kingslayer.rb
   ```

4. **Commit and push** the formula to the tap repository:
   ```bash
   git add Formula/kingslayer.rb
   git commit -m "Add kingslayer formula v0.1.0"
   git push origin main
   ```

5. **Users can now install** with:
   ```bash
   brew tap str4nge-m4g1c/kingslayer
   brew install kingslayer
   ```

## Updating the Formula

When you release a new version:

1. Update the `version` in the formula
2. Update the `url` fields to point to the new release
3. Download the new release artifacts
4. Calculate new SHA256 hashes
5. Update the `sha256` fields in the formula
6. Commit and push the changes

### Automated Updates

You can use `brew bump-formula-pr` to automatically update the formula:

```bash
brew bump-formula-pr --url=https://github.com/str4nge-m4g1c/regicide-tui/archive/v0.2.0.tar.gz kingslayer
```

## Testing the Formula

Before publishing updates, test the formula locally:

```bash
# Audit the formula for issues
brew audit --strict kingslayer

# Test installation
brew install --build-from-source kingslayer

# Run the formula test
brew test kingslayer
```
