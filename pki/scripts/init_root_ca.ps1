Param(
  [string]$CommonName = "Your Org Root CA",
  [int]$Days = 3650
)

$ErrorActionPreference = 'Stop'
$RootDir = Join-Path (Split-Path $PSScriptRoot) '..' | Join-Path -ChildPath 'root'
New-Item -ItemType Directory -Force -Path (Join-Path $RootDir 'certs') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $RootDir 'crl') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $RootDir 'newcerts') | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $RootDir 'private') | Out-Null
Set-Content -Path (Join-Path $RootDir 'index.txt') -Value ''
Set-Content -Path (Join-Path $RootDir 'serial') -Value '1000'

Push-Location $RootDir

& openssl genrsa -aes256 -out (Join-Path 'private' 'root.key.pem') 4096
icacls (Join-Path 'private' 'root.key.pem') /inheritance:r /grant:r "$($env:USERNAME):(R)" | Out-Null

& openssl req -config openssl.cnf `
  -key (Join-Path 'private' 'root.key.pem') `
  -new -x509 -days $Days -sha256 -extensions v3_ca `
  -subj "/CN=$CommonName" `
  -out (Join-Path 'certs' 'root.cert.pem')

Pop-Location
Write-Host "Root CA created at: $RootDir"


