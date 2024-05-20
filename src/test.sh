#!/usr/bin/env bash
set -euo pipefail

echo -e "\nGSM tool pack"

echo "Checking requirements..."
if [ -x "$(command  -v git)" ]
then
    echo "SUCCESS: Git is installed."
else
    echo "ERROR: Git not installed. Install Git via your package manager, or: https://git-scm.com/."
    exit 1
fi

if [ -x "$(command -v rustc)" ]
then
    echo "SUCCESS: Rust is installed."
else
    echo "ERROR: Rust is either installed incorrectly, or not at all."
    echo "Please download Rust using rustup https://www.rust-lang.org/tools/install."
    exit 1
fi


if [ -x "$(command -v rustup)" ]
then
    echo "SUCCESS: Rustup is installed."
else
    echo "ERROR: Please download rustup via https://rustup.rs."
    exit 1
fi

if [ -x "$(command -v cargo)" ]
then
    echo "SUCCESS: Cargo is installed."
else
    echo "ERROR: Cargo is either incorrectly installed, or not at all."
    echo "Please download Rust and Cargo via rustup."
    exit 1
fi

Path=${1:-gsm-tool/}
echo "Cloning gsm-tool at $Path..."
git clone -q https://github.com/Lazaurus/gsm-tool.git "$Path"

cd "$Path"
CargoBin="${CARGO_HOME:-$HOME/.cargo}/bin"

if [[ -z ${Version} ]]
then
    echo "The latest tag version could not be fetched remotely."
    echo "Using the local git repository..."
    Version=$(ls -tr .git/refs/tags/ | tail -1)
    if [[ -z ${Version}  ]]
    then
        echo "No valid tag version found"
        echo "Gsm-tool will be installed using the main branch"
        Version="main"
    else
        Version="tags/${Version}"
    fi
else
    Version="tags/${Version}"
fi

echo "Checking out version $Version..."
git checkout -q ${Version}

echo "Installing the 'gsm-tool' executable..."
cargo install --locked --force --path .

if ! [ -x "$(command -v gsm-tool)" ]
then
    echo "WARNING: Please check that you have '$CargoBin' in your PATH environment variable."
fi

Clippy=$(rustup component list | grep "clippy" | grep "installed")
if [ -z "$Clippy" ]
then
    echo "Installing the 'cargo-clippy' executable..."
    rustup component add clippy
fi

echo "Finished. Run gsm-tool to get started."

