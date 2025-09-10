Param(
  [string]$CommonName = "station-001",
  [int]$Days = 180
)

$ErrorActionPreference = 'Stop'
$BaseDir = Split-Path $PSScriptRoot -Parent
$IntDir = Join-Path $BaseDir 'intermediate'

Push-Location $IntDir
New-Item -ItemType Directory -Force -Path (Join-Path $IntDir 'csr') | Out-Null

& openssl genrsa -out (Join-Path 'private' "$CommonName.key.pem") 4096
icacls (Join-Path 'private' "$CommonName.key.pem") /inheritance:r /grant:r "$($env:USERNAME):(R)" | Out-Null

& openssl req -config openssl.cnf -new -sha256 `
  -subj "/CN=$CommonName" `
  -key (Join-Path 'private' "$CommonName.key.pem") -out (Join-Path 'csr' "$CommonName.csr.pem")

& openssl ca -batch -config openssl.cnf -extensions usr_cert -days $Days -notext -md sha256 `
  -in (Join-Path 'csr' "$CommonName.csr.pem") -out (Join-Path 'certs' "$CommonName.cert.pem")

Get-Content (Join-Path 'certs' "$CommonName.cert.pem") | Set-Content (Join-Path 'certs' "$CommonName.fullchain.pem")
Add-Content (Join-Path 'certs' "$CommonName.fullchain.pem") (Get-Content (Join-Path 'certs' 'ca-chain.pem'))

Pop-Location
Write-Host "Issued station certificate: $IntDir/certs/$CommonName.fullchain.pem"


