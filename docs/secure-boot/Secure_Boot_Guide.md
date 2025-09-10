# Secure Boot Guide (Scaffold)

## Goals
- Ensure ISO boot chain is signed and verifiable under UEFI Secure Boot

## Approach
- Sign shim/grub and kernel with org key
- Provide instructions to enroll org keys into UEFI db (or use Microsoft UEFI CA if applicable)

## Steps
1. Generate signing keypair (separate from Root/Intermediate CA)
2. Sign bootloader and kernel images
3. Embed certificates into ISO
4. Test on UEFI systems with Secure Boot enabled

## Key Enrollment
- Document BIOS/UEFI steps to enroll keys (db, KEK)
- Provide signed ISO variant for org rollout
