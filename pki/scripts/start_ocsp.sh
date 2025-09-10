#!/usr/bin/env bash
set -euo pipefail

HOST=${1:-127.0.0.1}
PORT=${2:-8888}

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
INT_DIR="$BASE_DIR/intermediate"

pushd "$INT_DIR" >/dev/null

# Create OCSP responder key/cert if missing (self-signed for demo)
if [ ! -f private/ocsp.key.pem ]; then
  openssl genrsa -out private/ocsp.key.pem 4096
  openssl req -new -key private/ocsp.key.pem -subj "/CN=OCSP Responder" -out csr/ocsp.csr.pem
  openssl x509 -req -in csr/ocsp.csr.pem -signkey private/ocsp.key.pem -days 365 -out certs/ocsp.cert.pem
fi

openssl ocsp \
  -index index.txt \
  -port "$HOST:$PORT" \
  -rsigner certs/ocsp.cert.pem \
  -rkey private/ocsp.key.pem \
  -CA certs/ca-chain.pem \
  -text

popd >/dev/null


