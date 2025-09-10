Param(
  [string]$Host = '127.0.0.1',
  [int]$Port = 8888
)

$ErrorActionPreference = 'Stop'
$BaseDir = Split-Path $PSScriptRoot -Parent
$IntDir = Join-Path $BaseDir 'intermediate'

Push-Location $IntDir

if (-not (Test-Path (Join-Path 'private' 'ocsp.key.pem'))) {
  & openssl genrsa -out (Join-Path 'private' 'ocsp.key.pem') 4096
  & openssl req -new -key (Join-Path 'private' 'ocsp.key.pem') -subj '/CN=OCSP Responder' -out (Join-Path 'csr' 'ocsp.csr.pem')
  & openssl x509 -req -in (Join-Path 'csr' 'ocsp.csr.pem') -signkey (Join-Path 'private' 'ocsp.key.pem') -days 365 -out (Join-Path 'certs' 'ocsp.cert.pem')
}

& openssl ocsp `
  -index (Join-Path '.' 'index.txt') `
  -port "$Host:$Port" `
  -rsigner (Join-Path 'certs' 'ocsp.cert.pem') `
  -rkey (Join-Path 'private' 'ocsp.key.pem') `
  -CA (Join-Path 'certs' 'ca-chain.pem') `
  -text

Pop-Location


