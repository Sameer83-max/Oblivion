# Secure Disk Erasure Tool

A cross-platform secure disk erasure tool that ensures data sanitization compliant with global standards like NIST SP 800-88.

## Features

- 🔒 Secure erasure of all user data including hidden storage areas (HPA/DCO, SSD reserved sectors)
- 📜 Digitally signed, tamper-proof wipe certificates (PDF & JSON)
- 🖱️ One-click intuitive interface suitable for the general public
- 💾 Offline usability via bootable ISO/USB
- ✅ Third-party verification of wipe certificates
- 🌍 Cross-platform support (Windows, Linux, Android)
- 📊 Compliance with international standards for IT asset disposal

## Architecture

```
├── src/
│   ├── core/           # Core wipe engine
│   ├── crypto/         # Cryptographic operations
│   ├── certificates/   # Certificate generation
│   ├── gui/           # User interface
│   ├── platform/      # Platform-specific implementations
│   └── cli/           # Command-line interface
├── bootable/          # Bootable ISO configuration
├── tests/             # Test suites
└── docs/              # Documentation
```

## Quick Start

### Prerequisites

- Rust 1.70+ 
- Platform-specific tools:
  - Windows: Administrator privileges
  - Linux: `hdparm`, `nvme-cli`, `sg3_utils`
  - Android: Root access or device owner mode

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/secure-disk-erasure.git
cd secure-disk-erasure

# Build the project
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### Usage

```bash
# List available drives
./target/release/secure-disk-erasure list

# Secure erase a drive
./target/release/secure-disk-erasure wipe --drive /dev/sda --mode full

# Verify a certificate
./target/release/secure-disk-erasure verify --certificate wipe_certificate.json
```

## Security Standards

This tool implements secure erasure methods compliant with:

- NIST SP 800-88 Rev. 1 (Guidelines for Media Sanitization)
- DoD 5220.22-M (National Industrial Security Program)
- ISO/IEC 27040:2015 (Storage security)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool performs irreversible data destruction. Always backup important data before use. The authors are not responsible for any data loss.
