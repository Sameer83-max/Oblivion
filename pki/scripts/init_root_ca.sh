#!/usr/bin/env bash
set -euo pipefail

CN=${1:-"Your Org Root CA"}
DAYS=${2:-3650}

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)/root"
mkdir -p "$ROOT_DIR"/{certs,crl,newcerts,private}
chmod 700 "$ROOT_DIR/private"
touch "$ROOT_DIR/index.txt"
echo 1000 > "$ROOT_DIR/serial"

pushd "$ROOT_DIR" >/dev/null

# Generate Root key (encrypted)
openssl genrsa -aes256 -out private/root.key.pem 4096
chmod 400 private/root.key.pem

# Self-signed Root certificate
openssl req -config openssl.cnf \
  -key private/root.key.pem \
  -new -x509 -days "$DAYS" -sha256 -extensions v3_ca \
  -subj "/CN=$CN" \
  -out certs/root.cert.pem
chmod 444 certs/root.cert.pem

echo "Root CA created at: $ROOT_DIR"
popd >/dev/null


