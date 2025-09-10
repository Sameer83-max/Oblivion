# Security Hardening (Scaffold)

## Code Signing & Provenance
- Sign all releases (CLI/GUI/Agent/Server)
- Reproducible builds; SBOM (CycloneDX/SPDX)
- Supply-chain policy (Cargo.lock checks, dependabot)

## Vulnerability Management
- Regular scans (cargo audit, SCA)
- CVE triage SLA; patch cadence

## Penetration Testing
- Internal and 3rd-party pen-tests
- Threat model: stations, server, PKI, ISO

## Keys & Secrets
- HSM-backed Intermediate; station keys in TPM/OS keystore
- Rotate keys; CRL/OCSP publishing

## Privacy & Telemetry
- Opt-in telemetry; data minimization; retention policies
