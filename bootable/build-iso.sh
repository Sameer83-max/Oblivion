#!/bin/bash
# Build script for Secure Disk Erasure Tool bootable ISO

set -e

echo "Building Secure Disk Erasure Tool bootable ISO..."

# Configuration
ISO_NAME="secure-disk-erasure-live"
ISO_VERSION="0.1.0"
BUILD_DIR="build"
LIVE_DIR="$BUILD_DIR/live"
ISO_DIR="$BUILD_DIR/iso"

# Create build directories
mkdir -p "$LIVE_DIR" "$ISO_DIR"

# Install live-build if not present
if ! command -v lb &> /dev/null; then
    echo "Installing live-build..."
    sudo apt-get update
    sudo apt-get install -y live-build
fi

# Create live-build configuration
cd "$LIVE_DIR"

# Initialize live-build
lb config

# Configure live-build
cat > config/chroot_local-hooks/01-install-tools << 'EOF'
#!/bin/bash
# Install required tools for secure disk erasure

apt-get update
apt-get install -y \
    hdparm \
    nvme-cli \
    sg3-utils \
    lsblk \
    parted \
    gdisk \
    dd \
    shred \
    wipe \
    openssl \
    curl \
    wget \
    nano \
    htop \
    tree

# Install Rust (for our tool)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Create application directory
mkdir -p /opt/secure-disk-erasure
EOF

chmod +x config/chroot_local-hooks/01-install-tools

# Configure packages
cat > config/package-lists/tools.list << 'EOF'
hdparm
nvme-cli
sg3-utils
lsblk
parted
gdisk
dd
shred
wipe
openssl
curl
wget
nano
htop
tree
EOF

# Configure desktop environment
echo "lxde" > config/package-lists/desktop.list

# Configure boot parameters
cat > config/bootloaders/syslinux/syslinux.cfg << 'EOF'
default live
label live
  menu label ^Secure Disk Erasure Tool
  kernel /live/vmlinuz
  append initrd=/live/initrd.img boot=live config quiet splash
label live-nomodeset
  menu label Secure Disk Erasure Tool (^nomodeset)
  kernel /live/vmlinuz
  append initrd=/live/initrd.img boot=live config quiet splash nomodeset
EOF

# Create custom application launcher
mkdir -p config/includes.usr.bin
cat > config/includes.usr.bin/secure-disk-erasure << 'EOF'
#!/bin/bash
# Secure Disk Erasure Tool Launcher

# Set up environment
export PATH="$PATH:/root/.cargo/bin"
export RUST_LOG=info

# Create desktop entry
cat > /usr/share/applications/secure-disk-erasure.desktop << 'DESKTOP'
[Desktop Entry]
Version=1.0
Type=Application
Name=Secure Disk Erasure Tool
Comment=Cross-platform secure disk erasure tool
Exec=/opt/secure-disk-erasure/secure-disk-erasure-gui
Icon=/opt/secure-disk-erasure/icon.png
Terminal=false
Categories=System;Security;
DESKTOP

# Create desktop shortcut
mkdir -p /home/user/Desktop
cp /usr/share/applications/secure-disk-erasure.desktop /home/user/Desktop/
chown user:user /home/user/Desktop/secure-disk-erasure.desktop
chmod +x /home/user/Desktop/secure-disk-erasure.desktop
EOF

chmod +x config/includes.usr.bin/secure-disk-erasure

# Configure auto-login
cat > config/includes.etc.lightdm.lightdm.conf.d.50-autologin.conf << 'EOF'
[SeatDefaults]
autologin-user=user
autologin-user-timeout=0
EOF

# Create custom welcome screen
cat > config/includes.etc.skel.Desktop.welcome.desktop << 'EOF'
[Desktop Entry]
Version=1.0
Type=Application
Name=Welcome to Secure Disk Erasure Tool
Comment=Get started with secure disk erasure
Exec=/opt/secure-disk-erasure/welcome.sh
Icon=/opt/secure-disk-erasure/welcome.png
Terminal=false
Categories=System;
EOF

# Build the ISO
echo "Building ISO (this may take a while)..."
sudo lb build

# Check if build was successful
if [ -f "binary.iso" ]; then
    echo "ISO built successfully!"
    
    # Move to output directory
    mv binary.iso "../$ISO_NAME-$ISO_VERSION.iso"
    
    # Create checksums
    cd ..
    sha256sum "$ISO_NAME-$ISO_VERSION.iso" > "$ISO_NAME-$ISO_VERSION.iso.sha256"
    md5sum "$ISO_NAME-$ISO_VERSION.iso" > "$ISO_NAME-$ISO_VERSION.iso.md5"
    
    echo "ISO created: $ISO_NAME-$ISO_VERSION.iso"
    echo "Size: $(du -h "$ISO_NAME-$ISO_VERSION.iso" | cut -f1)"
    echo "Checksums created:"
    echo "  SHA256: $(cat "$ISO_NAME-$ISO_VERSION.iso.sha256")"
    echo "  MD5: $(cat "$ISO_NAME-$ISO_VERSION.iso.md5")"
    
else
    echo "ISO build failed!"
    exit 1
fi

echo "Bootable ISO creation completed!"
echo ""
echo "To use the ISO:"
echo "1. Burn the ISO to a USB drive or DVD"
echo "2. Boot from the USB/DVD"
echo "3. The system will auto-login and show the Secure Disk Erasure Tool"
echo "4. Select your target device and choose wipe mode"
echo "5. Generate certificates for compliance reporting"
