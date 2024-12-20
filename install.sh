#!/bin/bash

# Rust installation checks
if ! command -v rustc >/dev/null 2>&1; then
  echo "✗ Rust is not installed on this system!"
  echo "To install Rust, visit: https://rustup.rs"
  echo "Or install via your system's package manager."
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "✗ Cargo is not installed!"
  echo "This is unusual - Cargo typically comes with Rust."
  echo "Consider reinstalling Rust using rustup: https://rustup.rs"
  exit 1
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo "✗ Rustup is not installed!"
  echo "Consider installing Rust via rustup: https://rustup.rs"
  exit 1
fi

# Shell-agnostic PATH setup
SHELL_RC="$HOME/.bashrc"
if [ -n "$ZSH_VERSION" ]; then
  SHELL_RC="$HOME/.zshrc"
fi

if ! echo $PATH | grep -q "$HOME/bin"; then
  echo 'export PATH="$HOME/bin:$PATH"' >> "$SHELL_RC"
  export PATH="$HOME/bin:$PATH"
fi

# Directory setup
mkdir -p ~/bin
mkdir -p ~/.config/stock-trader

# Credentials file setup
if [ ! -f "$HOME/.config/stock-trader/credentials.json" ]; then
    cat > ~/.config/stock-trader/credentials.json << 'EOF'
{
  "apcaApiKey": "",
  "apcaSecretKey": ""
}
EOF
  chmod 600 ~/.config/stock-trader/credentials.json
fi

# Build and install
if ! cargo build --release; then
  echo "Failed to build project"
  exit 1
fi

if ! sudo cp target/release/stock-trader ~/bin/; then
  echo "Failed to copy binary to ~/bin/"
  exit 1
fi

echo "Success!"