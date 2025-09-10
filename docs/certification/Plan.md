# Certification & Validation Plan (Scaffold)

## Objectives
- Independent validation against NIST SP 800-88 Rev.1
- Product assurance for ministries (STQC), optional CC EAL2/3, selective FIPS alignment

## Candidates
- **STQC** (India): Functional validation of sanitization workflows
- **Common Criteria**: EAL2/EAL3 augmented (data sanitization utility profile)
- **FIPS 140-3**: If crypto module boundary is in scope, use validated module or HSM

## Scope
- Media: HDD, SATA SSD, NVMe, USB, eMMC
- Methods: Clear, Purge; ATA Secure Erase/Enhanced, NVMe Sanitize/Format, TRIM
- Platforms: Windows, Linux, Bootable ISO; Android documented limitations
- Evidence: test plans, logs, signed certificates, SBOM, build provenance

## Timeline (indicative)
- Week 0–2: Scope freeze, RFP to labs, gap assessment
- Week 3–6: Internal dry-run, evidence prep, SBOM, signing
- Week 7–10: Lab testing window
- Week 11–12: Remediation & final certificate

## Deliverables
- Compliance mapping, SOPs, user/admin guides
- Signed test evidence; reproducible builds; SBOM (CycloneDX/SPDX)
- Security target (for CC) and threat model
