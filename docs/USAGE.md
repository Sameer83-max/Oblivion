# Secure Disk Erasure Tool - Usage Examples

## Basic Usage

### 1. List Available Devices

```bash
# List all storage devices
./secure-disk-erasure list

# List with detailed information
./secure-disk-erasure list --detailed
```

**Example Output:**
```
Found 2 storage device(s):

1. Samsung SSD 980 PRO
   Path: /dev/nvme0n1
   Size: 1000 GB
   Type: NVMe
   Model: Samsung SSD 980 PRO
   Serial: S5GXNF0N123456
   Secure Erase: true
   TRIM Support: true
   Hidden Areas: 0

2. Western Digital HDD
   Path: /dev/sda
   Size: 2000 GB
   Type: HDD
   Model: WD2003FZEX-00Z4SA0
   Serial: WD-WMC300123456
   Secure Erase: true
   TRIM Support: false
   Hidden Areas: 1
     - HPA: 1024 sectors
```

### 2. Generate Signing Keys

```bash
# Generate Ed25519 key pair
./secure-disk-erasure generate-keys --output ./keys

# Output:
# Key pair generated successfully:
#   Private key: ./keys/private_key.pem
#   Public key: ./keys/public_key.pem
```

### 3. Secure Erase Operations

#### Quick Wipe (Single Pass)
```bash
./secure-disk-erasure wipe \
  --device /dev/sda \
  --mode quick \
  --certificate \
  --output ./certificates
```

#### Full Wipe (Multiple Passes)
```bash
./secure-disk-erasure wipe \
  --device /dev/nvme0n1 \
  --mode full \
  --certificate \
  --output ./certificates
```

#### Advanced Wipe (Hardware Secure Erase)
```bash
./secure-disk-erasure wipe \
  --device /dev/sda \
  --mode advanced \
  --certificate \
  --output ./certificates
```

**Example Output:**
```
WARNING: This operation will permanently destroy all data on the device!
Device: Samsung SSD 980 PRO (/dev/nvme0n1)
Size: 1000 GB
Mode: Advanced

Wipe operation completed!
Duration: 1800 seconds
Bytes written: 1000 GB
Verification: PASSED

Enhanced certificate generated:
  JSON: ./certificates/wipe_certificate.json
  PDF: ./certificates/wipe_certificate.pdf
```

### 4. Verify Certificates

```bash
# Verify with default public key
./secure-disk-erasure verify \
  --certificate ./certificates/wipe_certificate.json

# Verify with specific public key
./secure-disk-erasure verify \
  --certificate ./certificates/wipe_certificate.json \
  --public-key ./keys/public_key.pem
```

**Example Output:**
```
Certificate Verification Result:
================================
✓ Certificate is VALID

Verification Details:
  Signature: ✓ Valid
  Hash: ✓ Valid
  Compliance: ✓ Valid

Verification Details:
  Certificate Age: 0 days
  Device Size: 1000 GB
  Wipe Duration: 1800 seconds
  Verification Ratio: 100.0%
```

## Advanced Usage

### 1. Batch Operations

```bash
#!/bin/bash
# Batch wipe script for multiple devices

DEVICES=("/dev/sda" "/dev/sdb" "/dev/nvme0n1")
OUTPUT_DIR="./certificates"

for device in "${DEVICES[@]}"; do
    echo "Wiping device: $device"
    ./secure-disk-erasure wipe \
        --device "$device" \
        --mode full \
        --certificate \
        --output "$OUTPUT_DIR"
done
```

### 2. Enterprise Deployment

```bash
# Generate organization keys
./secure-disk-erasure generate-keys --output /etc/secure-erase/keys

# Set up certificate templates
mkdir -p /etc/secure-erase/templates
cp organization_template.json /etc/secure-erase/templates/

# Deploy to multiple systems
for system in system1 system2 system3; do
    scp secure-disk-erasure $system:/usr/local/bin/
    scp /etc/secure-erase/keys/public_key.pem $system:/etc/secure-erase/
done
```

### 3. Compliance Reporting

```bash
# Generate compliance report
./secure-disk-erasure verify \
  --certificate ./certificates/wipe_certificate.json \
  --public-key ./keys/public_key.pem > compliance_report.txt

# Check multiple certificates
for cert in ./certificates/*.json; do
    echo "Verifying: $cert"
    ./secure-disk-erasure verify --certificate "$cert"
done
```

## Certificate Formats

### JSON Certificate Structure

```json
{
  "version": "2.0",
  "certificate_id": "WIPE_00000123456789AB_12345678",
  "timestamp": 1703123456,
  "issuer": {
    "name": "Secure Disk Erasure Tool",
    "organization": "Your Organization",
    "email": "admin@yourorg.com",
    "public_key_fingerprint": "ED25519"
  },
  "device_info": {
    "path": "/dev/nvme0n1",
    "name": "Samsung SSD 980 PRO",
    "size": 1000000000000,
    "device_type": "NVMe",
    "model": "Samsung SSD 980 PRO",
    "serial": "S5GXNF0N123456",
    "firmware_version": "5B2QGXA7",
    "interface_type": "PCIe 4.0 x4",
    "hidden_areas": [],
    "capabilities": {
      "supports_secure_erase": true,
      "supports_trim": true,
      "supports_crypto_erase": true,
      "supports_format_unit": true
    }
  },
  "wipe_details": {
    "mode": "Advanced",
    "start_time": 1703120000,
    "end_time": 1703121800,
    "duration_seconds": 1800,
    "bytes_written": 1000000000000,
    "passes_completed": 7,
    "verification_passed": true,
    "errors": [],
    "warnings": [],
    "performance_metrics": {
      "average_speed_mbps": 555.56,
      "peak_speed_mbps": 833.33,
      "sectors_per_second": 1086956,
      "retry_count": 0
    }
  },
  "verification": {
    "hash": "a1b2c3d4e5f6...",
    "algorithm": "SHA-256",
    "verification_method": "Random Sector Sampling",
    "sample_count": 100,
    "verification_ratio": 1.0,
    "forensic_tools_used": ["Internal Verification"]
  },
  "compliance": {
    "standards": [
      "NIST SP 800-88 Rev. 1",
      "DoD 5220.22-M",
      "ISO/IEC 27040:2015"
    ],
    "compliance_level": "High",
    "audit_trail": [
      {
        "timestamp": 1703120000,
        "action": "Wipe Operation Started",
        "result": "Success",
        "details": "Mode: Advanced"
      }
    ]
  },
  "signature": "1234567890abcdef...",
  "metadata": {
    "tool_version": "0.1.0",
    "platform": "linux",
    "architecture": "x86_64",
    "generated_by": "Secure Disk Erasure Tool",
    "qr_code_data": "{\"id\":\"WIPE_00000123456789AB_12345678\",...}"
  }
}
```

## Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   # Linux: Add user to disk group
   sudo usermod -a -G disk $USER
   
   # Windows: Run as Administrator
   # Right-click PowerShell -> "Run as Administrator"
   ```

2. **Device Not Found**
   ```bash
   # Check device exists
   ls -la /dev/sd* /dev/nvme*
   
   # Refresh device list
   ./secure-disk-erasure list --detailed
   ```

3. **Certificate Verification Failed**
   ```bash
   # Check public key
   ls -la public_key.pem
   
   # Regenerate keys if needed
   ./secure-disk-erasure generate-keys
   ```

4. **Slow Wipe Performance**
   ```bash
   # Use hardware secure erase for SSDs
   ./secure-disk-erasure wipe --device /dev/nvme0n1 --mode advanced
   
   # Check device health
   smartctl -a /dev/nvme0n1
   ```

### Logging and Debugging

```bash
# Enable verbose logging
RUST_LOG=debug ./secure-disk-erasure wipe --device /dev/sda --mode full

# Save logs to file
RUST_LOG=debug ./secure-disk-erasure wipe --device /dev/sda --mode full 2>&1 | tee wipe.log
```

## Security Best Practices

1. **Key Management**
   - Store private keys securely (HSM, hardware tokens)
   - Use separate keys for different environments
   - Implement key rotation policies

2. **Certificate Storage**
   - Store certificates in secure locations
   - Implement access controls
   - Regular backup of certificates

3. **Verification**
   - Always verify certificates after generation
   - Use multiple verification methods
   - Maintain audit trails

4. **Compliance**
   - Follow organizational policies
   - Document wipe procedures
   - Regular compliance audits
