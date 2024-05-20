#Requires -Version 5
param($path = "$home/gsm-tool")

Write-Host "GSM tool pack."

Write-Host "Checking requirements..."
if (Get-Command git -ErrorAction SilentlyContinue) {
    Write-Host "SUCCESS: Git is installed."
} else {
    Write-Host "WARNING: Git is not installed, ."
    Write-Host "ERROR: Git not installed. Install Git via your package manager or at: https://git-scm.com/."
    exit 1
}

if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Write-Host "SUCCESS: Rust is installed."
} else {
    Write-Host "ERROR: Rust is either installed incorrectly, or not at all."
    Write-Host "Please download Rust via https://rustup.rs."
    exit 1
}

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "SUCCESS: Cargo is installed."
} else {
    Write-Host "ERROR: Cargo is either installed incorrectly, or not at all."
    Write-Host "Please download Rust and Cargo via https://rustup.rs."
    exit 1
}

Write-Host "Cloning gsm-tool at $path"
git clone -q https://github.com/Lazaurus/gsm-tool $path
if (!($LASTEXITCODE -eq 0)) {
    exit 1
}

Write-Host "Checking version $version..."
Set-Location $path
git checkout -q tags/$version

Write-Host "Installing the 'gsm-tool' executable..."
cargo install --locked --force --path .
if (!(Get-Command gsm-tool -ErrorAction SilentlyContinue)) {
    Write-Host "WARNING: Check that you have '~/.cargo/bin' in your PATH environment variable."
}

# Checking whether Clippy is installed.
$clippy = (rustup component list | Select-String "clippy" | Select-String "installed") | Out-String
if (!$clippy) {
    Write-Host "Installing the 'cargo-clippy' executable..."
    rustup component add clippy
}

Write-Host "Operation finished. Run gsm-tool to get started."
