# 🚀 Deployment Guide

Step-by-step guide to deploy your Stylus ERC20 token.

## Prerequisites

1. ✅ Rust and cargo-stylus installed
2. ✅ Private key with testnet ETH
3. ✅ RPC endpoint (Arbitrum Sepolia or One)

## Get Testnet ETH

### Arbitrum Sepolia Faucet
```bash
# Get Sepolia ETH first
https://sepoliafaucet.com/

# Bridge to Arbitrum Sepolia
https://bridge.arbitrum.io/
```

Or use direct Arbitrum Sepolia faucets:
- https://faucet.quicknode.com/arbitrum/sepolia
- https://www.l2faucet.com/arbitrum

## Deployment Steps

### 1. Prepare Your Private Key

Create a file with your private key (NEVER commit this!):
```bash
echo "your_private_key_here" > .private_key
chmod 600 .private_key
```

Add to `.gitignore`:
```bash
echo ".private_key" >> .gitignore
```

### 2. Deploy to Arbitrum Sepolia (Testnet)
```bash
cargo stylus deploy \
  --private-key-path=.private_key \
  --endpoint=https://sepolia-rollup.arbitrum.io/rpc
```

**Expected Output:**
```
contract size: 12.7 KiB
wasm data fee: 0.00X ETH
✅ contract deployed at: 0x...
```

### 3. Initialize the Contract

After deployment, call `initialize()` to set yourself as owner:
```bash
# Using cast (from Foundry)
cast send <CONTRACT_ADDRESS> "initialize()" \
  --rpc-url https://sepolia-rollup.arbitrum.io/rpc \
  --private-key <YOUR_KEY>
```

### 4. Mint Initial Supply
```bash
# Mint 1,000,000 tokens to yourself
cast send <CONTRACT_ADDRESS> \
  "mint(address,uint256)" \
  <YOUR_ADDRESS> \
  1000000000000000000000000 \
  --rpc-url https://sepolia-rollup.arbitrum.io/rpc \
  --private-key <YOUR_KEY>
```

### 5. Verify on Explorer

Visit Arbiscan to see your contract:
- Testnet: https://sepolia.arbiscan.io/address/<CONTRACT_ADDRESS>
- Mainnet: https://arbiscan.io/address/<CONTRACT_ADDRESS>

## Deploy to Mainnet

⚠️ **Only deploy to mainnet after thorough testing!**
```bash
cargo stylus deploy \
  --private-key-path=.private_key \
  --endpoint=https://arb1.arbitrum.io/rpc
```

## Activation

Stylus contracts need periodic reactivation. To activate:
```bash
cargo stylus activate \
  --address <CONTRACT_ADDRESS> \
  --private-key-path=.private_key \
  --endpoint <RPC_URL>
```

## Interaction Examples

### Using cast (Foundry)
```bash
# Check balance
cast call <CONTRACT> "balanceOf(address)(uint256)" <ADDRESS> \
  --rpc-url <RPC>

# Transfer tokens
cast send <CONTRACT> "transfer(address,uint256)" <TO> <AMOUNT> \
  --rpc-url <RPC> --private-key <KEY>

# Approve spender
cast send <CONTRACT> "approve(address,uint256)" <SPENDER> <AMOUNT> \
  --rpc-url <RPC> --private-key <KEY>
```

### Using ethers.js
```javascript
const contract = new ethers.Contract(
  contractAddress,
  [
    "function transfer(address to, uint256 amount) returns (bool)",
    "function balanceOf(address account) view returns (uint256)"
  ],
  signer
);

// Transfer
await contract.transfer(recipient, amount);

// Check balance
const balance = await contract.balanceOf(address);
```

## Troubleshooting

### "Insufficient funds"
- Make sure you have enough ETH for gas
- Testnet faucets may be rate-limited

### "Invalid signature"
- Check your private key format
- Ensure key has no 0x prefix in file

### "Contract not activated"
- Run `cargo stylus activate` command
- Contracts need activation every ~7 days

## Security Checklist

Before mainnet deployment:

- [ ] Tested on testnet extensively
- [ ] Reviewed all functions
- [ ] Checked access control
- [ ] Verified owner address
- [ ] Considered getting an audit
- [ ] Backed up private keys securely
- [ ] Set up monitoring
- [ ] Prepared emergency response plan

## Cost Estimation

Typical costs on Arbitrum One:
- Deployment: ~0.001-0.003 ETH
- Initialize: ~0.0001 ETH  
- Mint: ~0.0001 ETH per mint
- Transfer: ~0.00005 ETH per transfer

**Much cheaper than Ethereum mainnet!**

## Next Steps

1. ✅ Deploy to testnet
2. ✅ Test all functions
3. ✅ Initialize and mint
4. ✅ Test transfers
5. ✅ Verify on explorer
6. ⚠️ Consider audit for mainnet
7. 🚀 Deploy to mainnet

---

**Questions?** Open an issue on GitHub!
