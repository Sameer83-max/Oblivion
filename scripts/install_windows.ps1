Param(
  [switch]$IncludeServer = $false,
  [switch]$IncludeISO = $false
)

$ErrorActionPreference = 'Stop'
Write-Host "== Secure Disk Erasure Tool - Windows Setup ==" -ForegroundColor Cyan

# 1) Install Rust (rustup)
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
  Write-Host "Installing Rust..." -ForegroundColor Yellow
  $rustup = "$PWD\\rustup-init.exe"
  Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustup
  & $rustup -y
  Remove-Item $rustup -Force
}

# 2) MSVC build tools (VS Build Tools)
Write-Host "Ensure Microsoft C++ Build Tools are installed (Desktop development with C++)" -ForegroundColor Yellow
Write-Host "If missing, download: https://aka.ms/vs/17/release/vs_BuildTools.exe" -ForegroundColor DarkYellow

# 3) Node.js LTS
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
  Write-Host "Installing Node.js LTS..." -ForegroundColor Yellow
  winget install -e --id OpenJS.NodeJS.LTS --accept-source-agreements --accept-package-agreements | Out-Null
}

# 4) Tauri CLI
Write-Host "Installing Tauri CLI..." -ForegroundColor Yellow
npm install -g @tauri-apps/cli | Out-Null

# 5) OpenSSL (for PKI scripts)
if (-not (Get-Command openssl -ErrorAction SilentlyContinue)) {
  Write-Host "Installing OpenSSL (Light)..." -ForegroundColor Yellow
  winget install -e --id ShiningLight.OpenSSL.Light --accept-source-agreements --accept-package-agreements | Out-Null
}

# Optional: server/agent extra tools
if ($IncludeServer) {
  Write-Host "Server/Agent: no extra deps beyond Rust on Windows." -ForegroundColor Yellow
}

# Optional: ISO build prerequisites (Windows note)
if ($IncludeISO) {
  Write-Host "ISO build is supported on Debian/Ubuntu. Use WSL or a Linux VM for live-build." -ForegroundColor Yellow
}

Write-Host "\nAll set. Restart the terminal to load Rust tools into PATH if needed." -ForegroundColor Green
