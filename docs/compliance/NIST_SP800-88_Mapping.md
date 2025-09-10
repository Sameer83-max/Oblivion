# NIST SP 800-88 Rev.1 Mapping (Scaffold)

This document maps product functions to NIST SP 800-88 requirements.

## Media Sanitization Methods
- Clear: TRIM / single-pass overwrite
- Purge: ATA Secure Erase / NVMe Sanitize/Format
- Destroy: Out of scope (physical destruction), provide guidance

## Validation & Verification
- Internal verification: random sector sampling, checksums
- External validation: forensic tools; lab validation (STQC)

## Audit & Records
- JSON/PDF certificate, Ed25519 signature, chain to org CA
- OCSP/CRL endpoints for revocation

## Residual Data Areas
- HPA/DCO handling documented; SSD remapped blocks via device sanitize

(Provide line-by-line mapping in next revision.)
