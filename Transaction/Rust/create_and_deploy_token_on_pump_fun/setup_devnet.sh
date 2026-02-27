#!/bin/bash

echo "🔧 Setting up Pump.fun Devnet Environment"
echo "=========================================="

if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install:"
    echo "   sh -c \"$(curl -sSfL https://release.solana.com/stable/install)\""
    exit 1
fi

solana config set --url devnet

# Create new keypair if needed
if [ ! -f ~/.config/solana/id.json ]; then
    echo "📝 Creating new wallet..."
    solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
fi

ADDRESS=$(solana address)
echo "📝 Wallet address: $ADDRESS"

# Request airdrop
echo "💧 Requesting airdrop..."
solana airdrop 2
solana airdrop 2

# Check balance
BALANCE=$(solana balance)
echo "💰 Balance: $BALANCE"

# Export private key for .env
echo "📋 Exporting private key for .env..."
echo "Your public key: $(solana-keygen pubkey ~/.config/solana/id.json)"
echo ""
echo "Add to .env:"
echo "PRIVATE_KEY=$(cat ~/.config/solana/id.json)"
echo "RPC_URL=https://api.devnet.solana.com"

echo ""
echo "✅ Devnet setup complete!"
