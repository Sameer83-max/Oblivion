#!/usr/bin/env bash
set -euo pipefail

CN=${1:-"Your Org Issuing CA"}
DAYS=${2:-1095}

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
ROOT_DIR="$BASE_DIR/root"
INT_DIR="$BASE_DIR/intermediate"

mkdir -p "$INT_DIR"/{certs,crl,csr,newcerts,private}
chmod 700 "$INT_DIR/private"
touch "$INT_DIR/index.txt"
echo 1000 > "$INT_DIR/serial"

pushd "$INT_DIR" >/dev/null

# Generate Intermediate key
openssl genrsa -aes256 -out private/intermediate.key.pem 4096
chmod 400 private/intermediate.key.pem

# CSR for Intermediate
openssl req -config openssl.cnf -new -sha256 \
  -subj "/CN=$CN" \
  -key private/intermediate.key.pem -out csr/intermediate.csr.pem

# Sign with Root CA
pushd "$ROOT_DIR" >/dev/null
openssl ca -batch -config openssl.cnf -extensions v3_ca -days "$DAYS" -notext -md sha256 \
  -in "$INT_DIR/csr/intermediate.csr.pem" \
  -out "$INT_DIR/certs/intermediate.cert.pem"
chmod 444 "$INT_DIR/certs/intermediate.cert.pem"
popd >/dev/null

# Create chain
cat certs/intermediate.cert.pem "$ROOT_DIR/certs/root.cert.pem" > certs/ca-chain.pem
chmod 444 certs/ca-chain.pem

echo "Intermediate CA created at: $INT_DIR"
popd >/dev/null


