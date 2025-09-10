# Bootable ISO Configuration

This directory contains the configuration files and scripts needed to create a bootable ISO for offline disk erasure operations.

## Overview

The bootable ISO provides a complete, self-contained environment for secure disk erasure that can be used on any system without requiring installation or administrative privileges on the host system.

## Features

- **Complete Linux Environment**: Based on Debian Live with LXDE desktop
- **Pre-installed Tools**: All necessary disk utilities (hdparm, nvme-cli, sg3-utils, etc.)
- **Secure Disk Erasure Tool**: Our Rust-based tool pre-compiled and ready to use
- **Auto-login**: Streamlined user experience
- **Offline Operation**: No network dependencies
- **Certificate Generation**: Full certificate support for compliance

## Building the ISO

### Prerequisites

```bash
# Install required tools
sudo apt-get update
sudo apt-get install -y live-build curl

# Ensure you have sufficient disk space (at least 4GB)
df -h
```

### Build Process

```bash
# Make the build script executable
chmod +x build-iso.sh

# Run the build script
./build-iso.sh
```

The build process will:
1. Create a Debian Live environment
2. Install all required disk utilities
3. Install Rust and compile our tool
4. Configure the desktop environment
5. Create the bootable ISO
6. Generate checksums for verification

### Build Output

After successful build, you'll find:
- `secure-disk-erasure-live-0.1.0.iso` - The bootable ISO
- `secure-disk-erasure-live-0.1.0.iso.sha256` - SHA256 checksum
- `secure-disk-erasure-live-0.1.0.iso.md5` - MD5 checksum

## Using the ISO

### Creating Bootable Media

#### USB Drive
```bash
# Find your USB device
lsblk

# Write ISO to USB (replace /dev/sdX with your USB device)
sudo dd if=secure-disk-erasure-live-0.1.0.iso of=/dev/sdX bs=4M status=progress

# Sync to ensure data is written
sync
```

#### DVD
```bash
# Burn ISO to DVD
growisofs -dvd-compat -Z /dev/sr0=secure-disk-erasure-live-0.1.0.iso
```

### Booting and Using

1. **Boot from Media**: Insert USB/DVD and boot from it
2. **Auto-login**: System will automatically log in as 'user'
3. **Launch Tool**: Double-click the Secure Disk Erasure Tool icon on desktop
4. **Select Device**: Choose target device from the list
5. **Configure Wipe**: Select wipe mode and options
6. **Execute Wipe**: Start the secure erase operation
7. **Generate Certificate**: Create compliance certificate
8. **Save Results**: Save certificate to USB or print

## Customization

### Adding Additional Tools

Edit `build-iso.sh` and add tools to the package list:

```bash
# In the tools.list section
cat > config/package-lists/tools.list << 'EOF'
hdparm
nvme-cli
sg3-utils
# Add your tools here
your-custom-tool
EOF
```

### Modifying Desktop Environment

Change the desktop environment by modifying:

```bash
# In build-iso.sh
echo "xfce" > config/package-lists/desktop.list  # Instead of lxde
```

### Custom Boot Parameters

Modify `config/bootloaders/syslinux/syslinux.cfg` to add custom boot options:

```
append initrd=/live/initrd.img boot=live config quiet splash your-custom-option
```

## Security Considerations

### Secure Boot

The ISO supports UEFI Secure Boot. To enable:

1. Sign the bootloader with your organization's key
2. Add the key to the ISO's EFI directory
3. Configure Secure Boot in the build process

### Key Management

For production deployments:

1. **Pre-install Keys**: Include organization keys in the ISO
2. **Secure Storage**: Store private keys in encrypted containers
3. **Key Rotation**: Implement regular key rotation procedures

### Network Security

The ISO operates offline by default, but if network access is needed:

1. Disable unnecessary network services
2. Use secure protocols only
3. Implement firewall rules
4. Monitor network activity

## Troubleshooting

### Common Issues

1. **Build Fails**
   ```bash
   # Check disk space
   df -h
   
   # Check live-build version
   lb --version
   
   # Clean previous builds
   sudo lb clean
   ```

2. **ISO Won't Boot**
   ```bash
   # Verify ISO integrity
   sha256sum -c secure-disk-erasure-live-0.1.0.iso.sha256
   
   # Check USB/DVD creation
   sudo fdisk -l /dev/sdX
   ```

3. **Tool Won't Start**
   ```bash
   # Check Rust installation
   /root/.cargo/bin/rustc --version
   
   # Check tool permissions
   ls -la /opt/secure-disk-erasure/
   ```

### Debug Mode

To enable debug mode:

1. Boot with `debug` parameter
2. Check logs in `/var/log/`
3. Use `journalctl` for system logs

## Enterprise Deployment

### Mass Distribution

For enterprise environments:

1. **Custom Branding**: Modify logos and welcome screens
2. **Centralized Keys**: Use organization-wide signing keys
3. **Compliance Templates**: Pre-configure compliance settings
4. **Audit Logging**: Enable comprehensive logging
5. **Network Integration**: Connect to central certificate storage

### Integration with Asset Management

The ISO can be integrated with existing asset management systems:

1. **API Endpoints**: Configure tool to report to central systems
2. **Certificate Upload**: Automatic certificate upload to central storage
3. **Asset Tracking**: Integration with asset tracking systems
4. **Compliance Reporting**: Automated compliance report generation

## Maintenance

### Regular Updates

1. **Security Updates**: Regularly update base system packages
2. **Tool Updates**: Update the secure disk erasure tool
3. **Certificate Updates**: Rotate signing keys periodically
4. **Compliance Updates**: Update compliance standards and templates

### Version Control

Maintain version control for:
- ISO builds
- Configuration files
- Custom scripts
- Certificate templates
- Documentation