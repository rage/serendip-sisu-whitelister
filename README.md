# Serendip Sisu Whitelister

A tool to whitelist students in Serendip based on Sisu data.

## Installation

1. Go to [Releases](https://github.com/rage/serendip-sisu-whitelister/releases)
2. Download the version for your platform:
   - Windows: `serendip-sisu-whitelister-vX.X.X-windows.exe`
   - MacOS: `serendip-sisu-whitelister-vX.X.X-macos`
   - Linux: `serendip-sisu-whitelister-vX.X.X-linux`
3. Make the file executable:
   - MacOS: Right-click the file, select "Properties" or "Get Info", and check "Allow executing file as program" or ensure the file permissions include "Execute"
   - Linux: Right-click the file, select "Properties", go to "Permissions" tab, and check "Allow executing file as program"
4. Run the executable

## For Developers

### Making a Release

1. Update version in `Cargo.toml`
2. Create and push a new tag:
   ```bash
   git tag v1.0.0  # Use appropriate version
   git push origin v1.0.0
   ```
3. GitHub Actions will automatically build and publish the release to the Releases page.