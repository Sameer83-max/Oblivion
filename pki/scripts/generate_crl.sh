#!/usr/bin/env bash
set -euo pipefail

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
INT_DIR="$BASE_DIR/intermediate"

pushd "$INT_DIR" >/dev/null
openssl ca -config openssl.cnf -gencrl -out crl/intermediate.crl.pem
chmod 444 crl/intermediate.crl.pem
echo "CRL generated: $INT_DIR/crl/intermediate.crl.pem"
popd >/dev/null


