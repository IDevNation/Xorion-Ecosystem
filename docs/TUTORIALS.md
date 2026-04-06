# Xorion Tutorials

Step-by-step tutorials to help you get started with Xorion.

## Table of Contents

1. [Tutorial 1: Create Your First Wallet](#tutorial-1-create-your-first-wallet)
2. [Tutorial 2: Send a Transaction](#tutorial-2-send-a-transaction)
3. [Tutorial 3: Deploy a Smart Contract](#tutorial-3-deploy-a-smart-contract)
4. [Tutorial 4: Create a dApp](#tutorial-4-create-a-dapp)

---

## Tutorial 1: Create Your First Wallet

**Time**: 5 minutes  
**Difficulty**: Beginner

In this tutorial, you'll create your first multi-chain wallet and learn how to secure it properly.

### Step 1: Install Xorion

Download and install Xorion from the [releases page](https://github.com/IDevNation/xorion-web3-os/releases).

### Step 2: Launch Application

Open Xorion. You'll see the welcome screen.

### Step 3: Create New Wallet

Click **"Create New Wallet"** button.

### Step 4: Set Password

Enter a strong password:
- At least 12 characters
- Mix of uppercase, lowercase, numbers, symbols
- Not used anywhere else

```
✅ Good: "Sunset@Mountain2024!Secure"
❌ Bad: "password123"
```

### Step 5: Backup Recovery Phrase

**⚠️ THIS IS CRITICAL ⚠️**

You'll see 12-24 words. This is your recovery phrase.

**DO:**
- ✅ Write down all words on paper
- ✅ Store in safe place (fireproof safe, safety deposit box)
- ✅ Make multiple copies in different locations
- ✅ Verify spelling carefully

**DON'T:**
- ❌ Take screenshot
- ❌ Save in cloud storage
- ❌ Email to yourself
- ❌ Store in password manager (unless encrypted)
- ❌ Share with anyone

**Example Recovery Phrase:**
```
abandon ability able about above absent
absorb abstract absurd abuse access accident
```

### Step 6: Confirm Backup

Select the words in correct order to prove you've backed them up.

### Step 7: Wallet Created! 🎉

You'll see your dashboard with:
- Ethereum address (0x...)
- Solana address (base58)
- Zero balances (for now)

### Next Steps

- Fund your wallet with some testnet tokens
- Explore the interface
- Try sending a small transaction

---

## Tutorial 2: Send a Transaction

**Time**: 10 minutes  
**Difficulty**: Beginner

Learn to send cryptocurrency on Ethereum and Solana networks.

### Prerequisites

- Wallet created (Tutorial 1)
- Some testnet tokens (get from faucet)

### Part A: Get Test Tokens

#### Ethereum Sepolia Testnet

1. Copy your Ethereum address from Xorion
2. Visit [Sepolia Faucet](https://sepoliafaucet.com/)
3. Paste address and request tokens
4. Wait ~30 seconds for confirmation

#### Solana Devnet

1. Copy your Solana address
2. Run command:
   ```bash
   solana airdrop 2 YOUR_SOLANA_ADDRESS --url devnet
   ```
3. Or visit [Solana Faucet](https://faucet.solana.com/)

### Part B: Send ETH on Sepolia

1. **Click "Send"** button

2. **Enter Recipient Address**
   - Paste friend's address or your own other address
   - Double-check! Transactions can't be reversed

3. **Select Asset**: ETH

4. **Enter Amount**: 0.01 ETH

5. **Review Details**
   ```
   To: 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
   Amount: 0.01 ETH
   Network Fee: ~0.001 ETH
   Total: 0.011 ETH
   ```

6. **Confirm & Sign**
   - Enter password
   - Click "Send"

7. **Track Transaction**
   - See pending status
   - Click tx hash to view on Etherscan
   - Wait for confirmation (~15 seconds on testnet)

### Part C: Send SOL on Devnet

1. **Click "Send"**

2. **Switch Network** to Solana

3. **Enter Details**
   - Recipient: Solana address
   - Amount: 0.5 SOL

4. **Confirm**
   - Much faster than Ethereum (~400ms)
   - Lower fees (< $0.01)

### Understanding Transaction Status

| Status | Meaning |
|--------|---------|
| Pending | Submitted, waiting for confirmation |
| Confirmed | Included in block |
| Finalized | Irreversible (wait for this!) |
| Failed | Transaction reverted (gas still spent) |

### Common Mistakes to Avoid

❌ **Wrong Network**: Sending ETH on Solana network = lost funds  
✅ Always verify network before sending

❌ **Wrong Address**: One character off = lost funds  
✅ Copy-paste, double-check first/last chars

❌ **Insufficient Gas**: Transaction fails but gas spent  
✅ Keep extra ETH for fees

---

## Tutorial 3: Deploy a Smart Contract

**Time**: 30 minutes  
**Difficulty**: Intermediate

Deploy your first ERC-20 token contract using Xorion.

### Prerequisites

- Basic Solidity knowledge
- Node.js installed
- Some Sepolia ETH for gas

### Step 1: Set Up Project

```bash
# Create project directory
mkdir my-token && cd my-token

# Initialize npm project
npm init -y

# Install Hardhat
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox

# Initialize Hardhat
npx hardhat init
# Choose: TypeScript, Add .gitignore, Add VSCode config
```

### Step 2: Write Token Contract

Create `contracts/MyToken.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MyToken is ERC20 {
    constructor(uint256 initialSupply) ERC20("MyToken", "MTK") {
        _mint(msg.sender, initialSupply);
    }
}
```

Install OpenZeppelin:
```bash
npm install @openzeppelin/contracts
```

### Step 3: Configure Deployment

Create `scripts/deploy.ts`:

```typescript
import { ethers } from "hardhat";

async function main() {
  const initialSupply = ethers.parseEther("1000000"); // 1 million tokens
  
  const MyToken = await ethers.getContractFactory("MyToken");
  const myToken = await MyToken.deploy(initialSupply);
  
  await myToken.waitForDeployment();
  
  console.log(`MyToken deployed to: ${await myToken.getAddress()}`);
  console.log(`Initial supply: ${ethers.formatEther(initialSupply)} MTK`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```

### Step 4: Configure Network

Update `hardhat.config.ts`:

```typescript
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.20",
  networks: {
    sepolia: {
      url: process.env.SEPOLIA_RPC_URL || "",
      accounts: process.env.PRIVATE_KEY !== undefined ? [process.env.PRIVATE_KEY] : [],
    },
  },
};

export default config;
```

Create `.env`:
```
SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
PRIVATE_KEY=your_private_key_here
```

⚠️ Never commit `.env` to git!

### Step 5: Deploy Contract

```bash
# Compile contract
npx hardhat compile

# Deploy to Sepolia
npx hardhat run scripts/deploy.ts --network sepolia
```

Output:
```
MyToken deployed to: 0x1234567890123456789012345678901234567890
Initial supply: 1000000.0 MTK
```

### Step 6: Verify on Etherscan

```bash
npx hardhat verify --network sepolia DEPLOYED_CONTRACT_ADDRESS 1000000000000000000000000
```

### Step 7: Add Token to Xorion

1. Open Xorion
2. Go to "Tokens" tab
3. Click "Add Custom Token"
4. Enter contract address
5. Token symbol and decimals auto-fill
6. Click "Add"

Now you can see and transfer your custom token!

### Next Steps

- Add token minting/burning features
- Create token vesting contract
- Build DeFi protocol around your token

---

## Tutorial 4: Create a dApp

**Time**: 45 minutes  
**Difficulty**: Advanced

Build a decentralized application that runs inside Xorion's WASM runtime.

### Prerequisites

- Rust programming knowledge
- Xorion SDK installed
- Basic Web3 concepts

### Step 1: Create Project

```bash
# Create new Rust project
cargo new xorion-dapp-demo
cd xorion-dapp-demo
```

### Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "xorion-dapp-demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
xorion-sdk = "1.0"
xorion-core = "1.0"
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[target.wasm32-unknown-unknown.dependencies]
getrandom = { version = "0.2", features = ["js"] }
```

### Step 3: Write dApp Code

Edit `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;
use xorion_core::{dapp_main, WalletBridge};
use serde::{Serialize, Deserialize};

#[dapp_main]
pub struct BalanceTracker {
    bridge: WalletBridge,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceInfo {
    address: String,
    eth_balance: String,
    sol_balance: String,
}

impl BalanceTracker {
    /// Get user's balances across chains
    pub async fn get_balances(&self) -> JsValue {
        let eth_address = self.bridge.get_eth_address().await;
        let sol_address = self.bridge.get_sol_address().await;
        
        let eth_balance = self.bridge.get_eth_balance(eth_address.clone()).await;
        let sol_balance = self.bridge.get_sol_balance(sol_address.clone()).await;
        
        let info = BalanceInfo {
            address: eth_address,
            eth_balance: ethers::utils::format_ether(eth_balance),
            sol_balance: (sol_balance as f64 / 1e9).to_string(),
        };
        
        JsValue::from_str(&serde_json::to_string(&info).unwrap())
    }
    
    /// Send ETH to address
    pub async fn send_eth(&self, to: String, amount_wei: u64) -> JsValue {
        match self.bridge.send_eth(to, amount_wei).await {
            Ok(tx_hash) => JsValue::from_str(&format!("Success: {}", tx_hash)),
            Err(e) => JsValue::from_str(&format!("Error: {}", e)),
        }
    }
    
    /// Check if user owns specific NFT
    pub async fn owns_nft(&self, contract: String, token_id: u64) -> JsValue {
        let owner = self.bridge.get_nft_owner(contract, token_id).await;
        let my_address = self.bridge.get_eth_address().await;
        
        JsValue::from_bool(owner.to_lowercase() == my_address.to_lowercase())
    }
}
```

### Step 4: Build for WASM

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Build release
cargo build --release --target wasm32-unknown-unknown

# Optimize WASM file
wasm-opt -O3 target/wasm32-unknown-unknown/release/xorion_dapp_demo.wasm \
  -o dist/dapp.wasm
```

### Step 5: Create dApp Manifest

Create `dist/dapp.toml`:

```toml
[dapp]
name = "Balance Tracker"
version = "1.0.0"
description = "Track your multi-chain balances"
author = "Your Name"

[permissions]
wallet_access = true
transaction_signing = true
ipfs_storage = false
network_access = ["ethereum", "solana"]

[ui]
icon = "icon.png"
color = "#4F46E5"
```

### Step 6: Test Locally

```bash
# Use Xorion CLI to test
xorion dapp test dist/dapp.wasm

# Or load in Xorion GUI development mode
xorion gui --dev dist/
```

### Step 7: Deploy to IPFS

```bash
# Upload to IPFS via Xorion
xorion ipfs add dist/dapp.wasm
xorion ipfs add dist/dapp.toml

# Pin for persistence
xorion ipfs pin YOUR_CID
```

### Step 8: Submit to dApp Store

1. Package your dApp files
2. Submit via GitHub issue or dApp portal
3. Include:
   - Description
   - Screenshots
   - Security audit (if handling funds)
   - Source code repository

### Example Frontend Integration

If building with web frontend:

```html
<!DOCTYPE html>
<html>
<head>
    <title>Balance Tracker</title>
</head>
<body>
    <button onclick="getBalances()">Check Balances</button>
    <div id="result"></div>

    <script type="module">
        import { init, getBalances } from './dapp.js';
        
        await init();
        
        window.getBalances = async () => {
            const result = await getBalances();
            document.getElementById('result').innerText = result;
        };
    </script>
</body>
</html>
```

### Congratulations! 🎉

You've built a fully functional dApp for Xorion!

### Next Steps

- Add more features (token swaps, staking)
- Improve UI/UX
- Get security audit
- Market your dApp
- Earn from usage fees

---

## Additional Resources

- **[Developer Guide](DEVELOPER_GUIDE.md)** - More advanced topics
- **[API Reference](../API.md)** - Complete API docs
- **[GitHub Examples](https://github.com/IDevNation/xorion-examples)** - Sample projects
- **[Discord](https://discord.gg/xorion)** - Get help from community

## Need Help?

Stuck on a tutorial? Join our Discord and ask in the #help channel!

---

**Last Updated**: April 2026  
**Version**: 1.0.0
