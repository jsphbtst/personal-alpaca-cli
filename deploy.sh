#!/bin/bash

if ! grep -q 'export PATH="$HOME/bin:$PATH"' ~/.zshrc; then
  echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc
fi

if [ ! -d "$HOME/bin" ]; then
  mkdir -p ~/bin
fi

if [ ! -d "$HOME/.config/stock-trader" ]; then
  mkdir -p ~/.config/stock-trader
fi

if [ ! -f "$HOME/.config/stock-trader/credentials.json" ]; then
  cat > ~/.config/stock-trader/credentials.json << 'EOF'
{
  "apcaApiKey": "",
  "apcaSecretKey": ""
}
EOF
  chmod 600 ~/.config/stock-trader/credentials.json
fi

# sudo cp target/release/stock-trader /usr/local/bin/ &&
cargo build --release &&
sudo cp target/release/stock-trader ~/bin/ &&
echo "Success!"