# PKI Scaffolding for Secure Disk Erasure Tool

This PKI enables enterprise-grade signing and verification of wipe certificates.

## Components

- Offline Root CA (air-gapped)
- Online Intermediate CA (operational)
- Station certificates (per wiping station)
- CRL/OCSP for revocation

## Directory Layout

```
pki/
├── root/                 # Offline Root CA
│   ├── openssl.cnf
│   ├── private/          # Root private key (offline only)
│   ├── certs/
│   ├── crl/
│   └── index.txt, serial
├── intermediate/         # Online Intermediate CA
│   ├── openssl.cnf
│   ├── private/
│   ├── certs/
│   ├── crl/
│   └── index.txt, serial
└── scripts/
    ├── init_root_ca.sh / .ps1
    ├── init_intermediate_ca.sh / .ps1
    ├── issue_station_cert.sh / .ps1
    ├── generate_crl.sh / .ps1
    └── start_ocsp.sh / .ps1
```

## Security Guidance

- Keep `root/private/root.key.pem` offline and encrypted. Never place online.
- Protect Intermediate CA key with HSM or OS keystore where possible (PKCS#11).
- Rotate Intermediate every 1–3 years. Rotate station certs every 90–180 days.
- Publish CRL and/or run OCSP; embed OCSP URL in issued certs.

## Quickstart (Linux/macOS)

```
cd pki/scripts

# 1) Initialize Root CA (run offline)
./init_root_ca.sh "Your Org Root CA" 3650

# 2) Initialize Intermediate CA (can be online)
./init_intermediate_ca.sh "Your Org Issuing CA" 1095

# 3) Issue a station certificate (CN=station-001)
./issue_station_cert.sh station-001 180

# 4) Generate CRL
./generate_crl.sh

# 5) (Optional) Start OCSP Responder
./start_ocsp.sh 127.0.0.1 8888
```

## Quickstart (Windows PowerShell)

Run from an elevated PowerShell in `pki\scripts`:

```
./init_root_ca.ps1 -CommonName "Your Org Root CA" -Days 3650
./init_intermediate_ca.ps1 -CommonName "Your Org Issuing CA" -Days 1095
./issue_station_cert.ps1 -CommonName "station-001" -Days 180
./generate_crl.ps1
./start_ocsp.ps1 -Host 127.0.0.1 -Port 8888
```

## App Integration

- Distribute `intermediate/certs/ca-chain.pem` with the app for verification.
- Configure the tool to sign certificates with station key (PKCS#11 or PEM),
  embedding the full chain and OCSP/CRL URLs.


