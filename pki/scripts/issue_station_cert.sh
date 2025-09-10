#!/usr/bin/env bash
set -euo pipefail

CN=${1:-"station-001"}
DAYS=${2:-180}

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
INT_DIR="$BASE_DIR/intermediate"

pushd "$INT_DIR" >/dev/null

mkdir -p csr

# Generate station key
openssl genrsa -out private/${CN}.key.pem 4096
chmod 400 private/${CN}.key.pem

# CSR
openssl req -config openssl.cnf -new -sha256 \
  -subj "/CN=$CN" \
  -key private/${CN}.key.pem -out csr/${CN}.csr.pem

# Issue station certificate
openssl ca -batch -config openssl.cnf -extensions usr_cert -days "$DAYS" -notext -md sha256 \
  -in csr/${CN}.csr.pem -out certs/${CN}.cert.pem
chmod 444 certs/${CN}.cert.pem

# Bundle chain
cat certs/${CN}.cert.pem certs/ca-chain.pem > certs/${CN}.fullchain.pem

echo "Issued station certificate: $INT_DIR/certs/${CN}.fullchain.pem"
popd >/dev/null


