# Secure Disk Erasure Tool - Setup Script

## Prerequisites Installation

### 1. Install Rust

**Windows:**
```powershell
# Download and run rustup-init.exe from https://rustup.rs/
# Or use PowerShell:
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe

# Restart PowerShell after installation
```

**Linux:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional tools
sudo apt-get update
sudo apt-get install hdparm nvme-cli sg3-utils lsblk
```

**macOS:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional tools
brew install hdparm nvme-cli sg3-utils
```

### 2. Verify Installation

```bash
# Check Rust version
rustc --version
cargo --version

# Should output something like:
# rustc 1.70.0 (90c541806 2023-05-31)
# cargo 1.70.0 (ec8a8a0ca 2022-06-08)
```

### 3. Build the Project

```bash
# Navigate to project directory
cd secure-disk-erasure

# Build the project
cargo build

# Run tests
cargo test

# Build release version
cargo build --release
```

### 4. Platform-Specific Setup

**Windows:**
- Run PowerShell as Administrator
- Install Windows SDK if not already installed
- Ensure you have write access to system drives

**Linux:**
- Add user to appropriate groups: `sudo usermod -a -G disk $USER`
- Install additional packages: `sudo apt-get install build-essential pkg-config libssl-dev`

**Android:**
- Install Android NDK
- Set up cross-compilation environment
- Ensure device has root access or device owner privileges

## Development Workflow

### 1. Code Changes
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test
```

### 2. Building for Different Platforms
```bash
# Windows
cargo build --target x86_64-pc-windows-msvc

# Linux
cargo build --target x86_64-unknown-linux-gnu

# Android (requires NDK setup)
cargo build --target aarch64-linux-android
```

### 3. Creating Bootable ISO
```bash
# Install live-build (Debian/Ubuntu)
sudo apt-get install live-build

# Build ISO
cd bootable/live-build
sudo lb build
```

## Troubleshooting

### Common Issues

1. **Permission Denied (Linux)**
   - Solution: Run with `sudo` or add user to `disk` group

2. **Device Not Found (Windows)**
   - Solution: Run PowerShell as Administrator

3. **Missing Dependencies**
   - Solution: Install platform-specific tools listed above

4. **Cross-compilation Issues**
   - Solution: Install target toolchain: `rustup target add <target>`

### Getting Help

- Check the logs: `RUST_LOG=debug cargo run`
- Review platform-specific documentation in `docs/`
- Test with virtual machines before real hardware
- Always backup important data before testing
