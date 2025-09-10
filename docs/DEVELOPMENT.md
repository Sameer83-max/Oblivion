# Secure Disk Erasure Tool - Development Guide

## Project Structure

```
secure-disk-erasure/
├── src/
│   ├── main.rs              # Main entry point and CLI
│   ├── lib.rs               # Library interface and tests
│   ├── core/                # Core wipe engine
│   │   └── mod.rs           # Core types and traits
│   ├── crypto/              # Cryptographic operations
│   │   └── mod.rs           # Ed25519 signing and verification
│   ├── certificates/        # Certificate generation
│   │   └── mod.rs           # PDF and JSON certificates
│   ├── platform/            # Platform-specific implementations
│   │   ├── mod.rs           # Platform module declarations
│   │   ├── windows.rs       # Windows-specific code
│   │   ├── linux.rs         # Linux-specific code
│   │   └── android.rs       # Android-specific code
│   ├── cli/                 # Command-line interface
│   │   └── mod.rs           # CLI command handlers
│   └── error.rs             # Error types and handling
├── bootable/                # Bootable ISO configuration
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── Cargo.toml              # Rust project configuration
├── README.md               # Project overview
└── LICENSE                 # MIT License
```

## Development Setup

### Prerequisites

1. **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
2. **Platform-specific tools:**
   - **Windows**: PowerShell, Administrator privileges
   - **Linux**: `hdparm`, `nvme-cli`, `sg3_utils`, `lsblk`
   - **Android**: Android NDK, device with root access

### Building the Project

```bash
# Clone and build
git clone <repository-url>
cd secure-disk-erasure
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Generate documentation
cargo doc --open
```

## Architecture Overview

### Core Components

1. **Core Wipe Engine** (`src/core/`)
   - Defines device types and wipe operations
   - Platform-agnostic interfaces
   - Result types and error handling

2. **Platform Implementations** (`src/platform/`)
   - Windows: PowerShell, Win32 API, diskpart
   - Linux: hdparm, nvme-cli, dd, shred
   - Android: Limited functionality, device owner APIs

3. **Cryptographic Layer** (`src/crypto/`)
   - Ed25519 key generation and signing
   - Certificate verification
   - Hash generation for integrity

4. **Certificate System** (`src/certificates/`)
   - JSON certificate generation
   - PDF certificate with QR codes
   - Digital signature embedding

5. **CLI Interface** (`src/cli/`)
   - Device listing and selection
   - Wipe operation execution
   - Certificate verification

## Security Considerations

### Key Management
- Private keys should be stored securely (HSM, hardware tokens)
- Public keys can be distributed for verification
- Consider key rotation policies

### Certificate Verification
- Certificates include cryptographic hashes
- Digital signatures prevent tampering
- Timestamps ensure freshness

### Platform Security
- Windows: Requires Administrator privileges
- Linux: May require root access for hardware operations
- Android: Limited by app sandboxing

## Testing Strategy

### Unit Tests
- Core data structures and algorithms
- Cryptographic operations
- Error handling paths

### Integration Tests
- Platform-specific device operations
- Certificate generation and verification
- End-to-end wipe workflows

### Security Tests
- Certificate tampering detection
- Key validation
- Wipe verification accuracy

## Deployment Options

### Native Applications
- Windows: MSI installer with admin privileges
- Linux: DEB/RPM packages with proper permissions
- Android: APK with device owner capabilities

### Bootable ISO
- Minimal Linux distribution
- Pre-installed wipe tools
- Offline operation capability

## Contributing Guidelines

1. **Code Style**: Follow Rust conventions (`cargo fmt`)
2. **Documentation**: Document all public APIs
3. **Testing**: Add tests for new functionality
4. **Security**: Review cryptographic implementations
5. **Platform Support**: Test on target platforms

## Future Enhancements

### Phase 2 Features
- Cloud certificate storage
- Blockchain-based verification
- Enterprise management dashboard
- Multi-language support

### Phase 3 Features
- AI-based anomaly detection
- Advanced forensic verification
- Compliance reporting
- Integration with asset management systems
