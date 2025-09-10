$ErrorActionPreference = 'Stop'
$BaseDir = Split-Path $PSScriptRoot -Parent
$IntDir = Join-Path $BaseDir 'intermediate'

Push-Location $IntDir
& openssl ca -config openssl.cnf -gencrl -out (Join-Path 'crl' 'intermediate.crl.pem')
Pop-Location
Write-Host "CRL generated: $IntDir/crl/intermediate.crl.pem"


