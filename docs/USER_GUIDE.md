# Xorion User Guide

Complete guide for using Xorion Web3 Wallet.

## Table of Contents

1. [Installation](#installation)
2. [Creating a Wallet](#creating-a-wallet)
3. [Sending & Receiving Crypto](#sending--receiving-crypto)
4. [Private Transactions (ZK)](#private-transitions-zk)
5. [Installing dApps](#installing-dapps)
6. [IPFS File Management](#ipfs-file-management)
7. [DAO Voting](#dao-voting)
8. [Troubleshooting](#troubleshooting)

---

## Installation

### Desktop Application

#### Windows

1. Download the latest installer from [Releases](https://github.com/IDevNation/xorion-web3-os/releases)
2. Run `Xorion-Setup-x.x.x.exe`
3. Follow installation wizard
4. Launch Xorion from Start Menu

#### macOS

1. Download `Xorion-x.x.x.dmg`
2. Drag Xorion to Applications folder
3. On first launch, right-click → Open (to bypass Gatekeeper)
4. Enter your password when prompted

#### Linux

**Ubuntu/Debian:**
```bash
wget https://github.com/IDevNation/xorion-web3-os/releases/download/v1.0.0/xorion_x.x.x_amd64.deb
sudo apt install ./xorion_x.x.x_amd64.deb
```

**Fedora/RHEL:**
```bash
wget https://github.com/IDevNation/xorion-web3-os/releases/download/v1.0.0/xorion-x.x.x.x86_64.rpm
sudo dnf install ./xorion-x.x.x.x86_64.rpm
```

**AppImage (Any Linux):**
```bash
wget https://github.com/IDevNation/xorion-web3-os/releases/download/v1.0.0/Xorion-x.x.x.AppImage
chmod +x Xorion-x.x.x.AppImage
./Xorion-x.x.x.AppImage
```

### Browser Extension (Coming Soon)

Chrome, Firefox, and Brave extensions are in development.

### Mobile Apps (Planned)

iOS and Android apps are on the roadmap.

---

## Creating a Wallet

### New Wallet

1. **Launch Xorion**
   - Open the application

2. **Click "Create New Wallet"**
   - You'll see this option on the welcome screen

3. **Set Strong Password**
   - Minimum 8 characters
   - Use mix of letters, numbers, symbols
   - This encrypts your local wallet data

4. **Backup Recovery Phrase**
   - ⚠️ **CRITICAL**: Write down all 12-24 words
   - Store in secure location (safe, safety deposit box)
   - Never store digitally (no screenshots, cloud, email)
   - Anyone with these words can steal your funds

5. **Confirm Recovery Phrase**
   - Select words in correct order to verify backup

6. **Wallet Created!**
   - You'll see your dashboard with balances
   - Your wallet supports multiple chains automatically

### Import Existing Wallet

1. Click "Import Wallet"
2. Enter your recovery phrase (12-24 words)
3. Set new password
4. Wallet imported with all addresses

### Hardware Wallet Support

Xorion supports Ledger and Trezor:

1. Connect hardware wallet
2. Select "Connect Hardware Wallet"
3. Follow on-screen instructions
4. Approve connection on device

---

## Sending & Receiving Crypto

### Receive Cryptocurrency

1. **Click "Receive"** button
2. **Select Network** (Ethereum, Solana, etc.)
3. **Copy Address** or share QR code
4. **Share with sender**

⚠️ Always verify you're sending on the correct network!

### Send Cryptocurrency

1. **Click "Send"** button
2. **Enter Recipient Address**
   - Paste address or scan QR code
   - Double-check address (transactions are irreversible)

3. **Select Asset**
   - Choose which token to send

4. **Enter Amount**
   - Input amount or click "Max"
   - See estimated gas fees

5. **Review Transaction**
   - Verify recipient, amount, fees
   - Check network confirmation time

6. **Confirm & Sign**
   - Enter password or use hardware wallet
   - Transaction submitted to blockchain

7. **Track Status**
   - View transaction in history
   - Click tx hash to view on block explorer

### Transaction Fees

- **Ethereum**: Paid in ETH (gas)
- **Solana**: Paid in SOL (very low fees)
- Fees vary based on network congestion

💡 Tip: Send during off-peak hours for lower fees

---

## Private Transactions (ZK)

Xorion includes built-in privacy using zk-SNARKs technology.

### What Are Private Transactions?

- Hide sender, receiver, and amount from public view
- Still verifiable on blockchain
- Optional feature (use when needed)

### How to Send Privately

1. **Enable Privacy Mode**
   - Toggle "Private Transaction" switch
   - First-time setup creates shielded pool deposit

2. **Deposit to Shielded Pool**
   - Convert public tokens to private notes
   - Wait for confirmation (~2 minutes)

3. **Send Privately**
   - Enter recipient's shielded address
   - Amount hidden from public
   - Higher fee than regular transactions

4. **Recipient Receives**
   - Funds arrive in their shielded balance
   - They can keep private or convert to public

### Privacy Best Practices

✅ Do:
- Use private transactions for sensitive payments
- Regularly rotate shielded addresses
- Keep some funds in public balance for regular use

❌ Don't:
- Send entire balance privately (creates linkability)
- Mix private and public funds unnecessarily
- Share shielded addresses publicly

### Viewing Private Transactions

- Only you and recipient can see details
- Use viewing keys for auditors/tax purposes
- Generate proof of payment without revealing amount

---

## Installing dApps

Xorion supports decentralized applications through its WASM runtime.

### Browse dApp Store

1. Click "dApps" tab
2. Browse categories or search
3. Read dApp description and permissions

### Install dApp

1. **Click "Install"** on dApp page
2. **Review Permissions**
   - Wallet access (read addresses)
   - Transaction signing
   - Storage access
   - Network access

3. **Approve Installation**
   - dApp downloads from IPFS
   - Installed locally (runs on your machine)

4. **Launch dApp**
   - Opens in sandboxed environment
   - Interacts with wallet via secure API

### Popular dApps

- **SwapDEX**: Decentralized exchange
- **NFT Gallery**: Manage your NFT collection
- **Lending Protocol**: Earn interest on crypto
- **DAO Dashboard**: Participate in governance

### Managing Installed dApps

- **View**: Settings → dApps
- **Update**: Automatic or manual check
- **Remove**: Uninstall button
- **Permissions**: Modify anytime

### Security

All dApps run in sandboxed environment:
- Cannot access private keys directly
- Require approval for transactions
- Isolated from system resources

---

## IPFS File Management

Store and retrieve files from decentralized IPFS network.

### Upload Files

1. **Go to "Storage" Tab**
2. **Click "Upload"**
3. **Select Files**
   - Drag & drop or browse
   - Max file size: 1GB (larger files chunked)

4. **Choose Options**
   - Encrypt before upload (recommended)
   - Pin to keep available
   - Add metadata/tags

5. **Upload Complete**
   - Get CID (Content Identifier)
   - Shareable link: `ipfs://{CID}`
   - Gateway URL for web access

### Download Files

1. **Enter CID or IPFS URL**
2. **Click "Retrieve"**
3. **File Downloads**
   - Decrypt if encrypted
   - Save to local storage

### Manage Pinned Files

- **View Pins**: Storage → Pinned
- **Unpin**: Remove to save space
- **Repin**: Ensure availability

### Encryption

Files encrypted client-side before upload:
- AES-256-GCM encryption
- Password-based key derivation
- Only you can decrypt

### Use Cases

- Backup important documents
- Share files censorship-resistant
- Host website content
- Store NFT metadata

---

## DAO Voting

Participate in Xorion governance with $XORION tokens.

### Get Voting Power

1. **Hold $XORION Tokens**
   - Purchase on exchanges
   - Earn through participation rewards

2. **Delegate (Optional)**
   - Delegate to trusted party
   - Or vote directly

### View Proposals

1. **Go to "Governance" Tab**
2. **Browse Active Proposals**
   - Title and summary
   - Description and details
   - Current voting results
   - Time remaining

3. **Read Proposal**
   - Full specification
   - Discussion thread
   - Community sentiment

### Cast Your Vote

1. **Select Proposal**
2. **Choose Option**
   - For / Against / Abstain
3. **Enter Voting Amount**
   - All or partial tokens
4. **Sign Transaction**
   - Gas-free voting (Layer 2)
5. **Vote Recorded**
   - Visible on-chain
   - Can't change once cast

### Create Proposal

1. **Check Requirements**
   - Minimum token holding
   - No active proposals by you

2. **Draft Proposal**
   - Use template
   - Clear title and description
   - Implementation plan

3. **Submit On-Chain**
   - Pay small deposit (refundable)
   - Enters discussion period

4. **Campaign**
   - Explain to community
   - Answer questions
   - Gather support

5. **Voting Period**
   - 7 days typically
   - Quorum required

6. **Execution**
   - If passed: implemented
   - If failed: deposit returned

### Voting Tips

- Research before voting
- Participate even with small holdings
- Delegate if you can't stay informed
- Vote on treasury spending

---

## Troubleshooting

### Common Issues

**Can't connect to network:**
- Check internet connection
- Try different RPC endpoint
- Update to latest version

**Transaction stuck pending:**
- Wait for network congestion to clear
- Try increasing gas price
- Contact support if >24 hours

**Forgot password:**
- Password only encrypts local data
- Use recovery phrase to restore wallet
- Set new password

**Wrong network selected:**
- Click network selector at top
- Choose correct network
- Funds appear on respective chain

**dApp not loading:**
- Check internet connection
- Clear dApp cache
- Reinstall dApp

**IPFS upload failing:**
- Check file size limits
- Try different gateway
- Ensure IPFS node running

### Getting Help

- **Documentation**: docs/README.md
- **FAQ**: FAQ.md
- **Discord**: [Join server](https://discord.gg/xorion)
- **GitHub Issues**: Report bugs
- **Email**: support@xorion.io

---

**Last Updated**: April 2026  
**Version**: 1.0.0

For updates, follow [@XorionWeb3](https://twitter.com/XorionWeb3)
