Param(
  [string]$CommonName = "Your Org Issuing CA",
  [int]$Days = 1095
)

$ErrorActionPreference = 'Stop'
$BaseDir = Split-Path $PSScriptRoot -Parent
$RootDir = Join-Path $BaseDir 'root'
$IntDir = Join-Path $BaseDir 'intermediate'

New-Item -ItemType Directory -Force -Path (Join-Path $IntDir 'certs') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $IntDir 'crl') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $IntDir 'csr') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $IntDir 'newcerts') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $IntDir 'private') | Out-Null
Set-Content -Path (Join-Path $IntDir 'index.txt') -Value ''
Set-Content -Path (Join-Path $IntDir 'serial') -Value '1000'

Push-Location $IntDir

& openssl genrsa -aes256 -out (Join-Path 'private' 'intermediate.key.pem') 4096
icacls (Join-Path 'private' 'intermediate.key.pem') /inheritance:r /grant:r "$($env:USERNAME):(R)" | Out-Null

& openssl req -config openssl.cnf -new -sha256 `
  -subj "/CN=$CommonName" `
  -key (Join-Path 'private' 'intermediate.key.pem') -out (Join-Path 'csr' 'intermediate.csr.pem')

Pop-Location

Push-Location $RootDir
& openssl ca -batch -config openssl.cnf -extensions v3_ca -days $Days -notext -md sha256 `
  -in (Join-Path $IntDir 'csr/intermediate.csr.pem') `
  -out (Join-Path $IntDir 'certs/intermediate.cert.pem')
Pop-Location

Get-Content (Join-Path $IntDir 'certs/intermediate.cert.pem') | \
  Set-Content (Join-Path $IntDir 'certs/ca-chain.pem')
Add-Content (Join-Path $IntDir 'certs/ca-chain.pem') (Get-Content (Join-Path $RootDir 'certs/root.cert.pem'))

Write-Host "Intermediate CA created at: $IntDir"


