# API Reference

Complete API documentation for Xorion Web3 SDK.

## Table of Contents

1. [Wallet API](#wallet-api)
2. [RPC API](#rpc-api)
3. [Contract API](#contract-api)
4. [Privacy API](#privacy-api)
5. [Storage API](#storage-api)

---

## Wallet API

### `from_mnemonic(mnemonic: string, password?: string): Promise<Wallet>`

Creates a wallet instance from a BIP-39 mnemonic phrase.

**Parameters:**
- `mnemonic` (string): 12-24 word recovery phrase
- `password` (string, optional): BIP-39 passphrase for additional security

**Returns:** `Promise<Wallet>`

**Example:**
```typescript
import { Wallet } from '@xorion/sdk';

const mnemonic = 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
const wallet = await Wallet.from_mnemonic(mnemonic);

console.log('Ethereum Address:', await wallet.eth_address());
console.log('Solana Address:', await wallet.solana_address());
```

---

### `eth_address(): Promise<string>`

Returns the Ethereum address derived from the wallet's private key.

**Returns:** `Promise<string>` - Ethereum address in hex format (0x...)

**Example:**
```typescript
const ethAddress = await wallet.eth_address();
console.log(ethAddress); // "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
```

---

### `solana_address(): Promise<string>`

Returns the Solana address derived from the wallet's private key.

**Returns:** `Promise<string>` - Solana base58-encoded address

**Example:**
```typescript
const solAddress = await wallet.solana_address();
console.log(solAddress); // "7EqQdEUaxGGeXNbR1M3P6v3bSfFzWLKq3eMuP8xYzAbc"
```

---

### `sign_transaction(transaction: Transaction): Promise<SignedTransaction>`

Signs a transaction with the wallet's private key.

**Parameters:**
- `transaction` (Transaction): Unsigned transaction object

**Returns:** `Promise<SignedTransaction>`

**Example:**
```typescript
const tx = {
  to: '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb',
  value: ethers.parseEther('1.0'),
  chainId: 1
};

const signedTx = await wallet.sign_transaction(tx);
await provider.sendTransaction(signedTx);
```

---

## RPC API

### `EthereumProvider`

#### Constructor

```typescript
new EthereumProvider(rpcUrl: string, options?: ProviderOptions)
```

**Parameters:**
- `rpcUrl` (string): Ethereum JSON-RPC endpoint
- `options` (ProviderOptions, optional): Configuration options

**Example:**
```typescript
import { EthereumProvider } from '@xorion/sdk';

const provider = new EthereumProvider('https://mainnet.infura.io/v3/YOUR_PROJECT_ID', {
  chainId: 1,
  timeout: 30000,
  retries: 3
});
```

---

#### `getBalance(address: string): Promise<bigint>`

Returns the balance of an Ethereum address in wei.

**Parameters:**
- `address` (string): Ethereum address

**Returns:** `Promise<bigint>` - Balance in wei

**Example:**
```typescript
const balance = await provider.getBalance('0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb');
console.log(ethers.formatEther(balance), 'ETH');
```

---

#### `sendTransaction(signedTx: string): Promise<string>`

Broadcasts a signed transaction to the network.

**Parameters:**
- `signedTx` (string): Hex-encoded signed transaction

**Returns:** `Promise<string>` - Transaction hash

**Example:**
```typescript
const txHash = await provider.sendTransaction(signedTx);
console.log('Transaction sent:', txHash);
```

---

#### `call(contract: string, data: string): Promise<string>`

Executes a read-only contract call.

**Parameters:**
- `contract` (string): Contract address
- `data` (string): Encoded function call data

**Returns:** `Promise<string>` - Return data

**Example:**
```typescript
const result = await provider.call(tokenAddress, '0x70a08231' + addressEncoded);
const balance = ethers.decodeAbiParameters(['uint256'], result)[0];
```

---

### `SolanaProvider`

#### Constructor

```typescript
new SolanaProvider(rpcUrl: string, commitment?: CommitmentLevel)
```

**Parameters:**
- `rpcUrl` (string): Solana RPC endpoint
- `commitment` (CommitmentLevel, optional): 'finalized', 'confirmed', or 'processed'

**Example:**
```typescript
import { SolanaProvider } from '@xorion/sdk';

const provider = new SolanaProvider('https://api.mainnet-beta.solana.com', 'confirmed');
```

---

#### `getBalance(pubkey: string): Promise<number>`

Returns the balance of a Solana account in lamports.

**Parameters:**
- `pubkey` (string): Base58-encoded public key

**Returns:** `Promise<number>` - Balance in lamports

**Example:**
```typescript
const balance = await provider.getBalance('7EqQdEUaxGGeXNbR1M3P6v3bSfFzWLKq3eMuP8xYzAbc');
console.log(balance / 1e9, 'SOL');
```

---

#### `sendTransaction(transaction: Transaction): Promise<string>`

Sends a signed Solana transaction.

**Parameters:**
- `transaction` (Transaction): Signed transaction

**Returns:** `Promise<string>` - Transaction signature

**Example:**
```typescript
const signature = await provider.sendTransaction(signedTx);
console.log('Signature:', signature);
```

---

## Contract API

### `ERC20`

#### Constructor

```typescript
new ERC20(address: string, provider: EthereumProvider)
```

**Example:**
```typescript
import { ERC20 } from '@xorion/sdk';

const usdc = new ERC20('0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48', provider);
```

---

#### `balanceOf(account: string): Promise<bigint>`

Returns the token balance of an account.

**Parameters:**
- `account` (string): Address to check

**Returns:** `Promise<bigint>` - Token balance (in smallest units)

**Example:**
```typescript
const balance = await usdc.balanceOf(userAddress);
console.log(balance.toString(), 'USDC');
```

---

#### `transfer(to: string, amount: bigint): Promise<TransactionReceipt>`

Transfers tokens to another address.

**Parameters:**
- `to` (string): Recipient address
- `amount` (bigint): Amount in smallest units

**Returns:** `Promise<TransactionReceipt>`

**Example:**
```typescript
const receipt = await usdc.transfer(recipientAddress, ethers.parseUnits('100', 6));
console.log('Transfer confirmed in block', receipt.blockNumber);
```

---

#### `approve(spender: string, amount: bigint): Promise<TransactionReceipt>`

Approves another address to spend tokens.

**Parameters:**
- `spender` (string): Address to approve
- `amount` (bigint): Amount to approve

**Returns:** `Promise<TransactionReceipt>`

**Example:**
```typescript
await usdc.approve(uniswapRouterAddress, ethers.MaxUint256);
```

---

### `ERC721`

#### Constructor

```typescript
new ERC721(address: string, provider: EthereumProvider)
```

**Example:**
```typescript
const nft = new ERC721('0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D', provider);
```

---

#### `ownerOf(tokenId: bigint): Promise<string>`

Returns the owner of a specific NFT.

**Parameters:**
- `tokenId` (bigint): Token ID

**Returns:** `Promise<string>` - Owner address

**Example:**
```typescript
const owner = await nft.ownerOf(1234n);
console.log('Owner:', owner);
```

---

#### `safeTransferFrom(from: string, to: string, tokenId: bigint): Promise<TransactionReceipt>`

Safely transfers an NFT.

**Parameters:**
- `from` (string): Current owner
- `to` (string): New owner
- `tokenId` (bigint): Token ID

**Returns:** `Promise<TransactionReceipt>`

**Example:**
```typescript
await nft.safeTransferFrom(fromAddress, toAddress, 1234n);
```

---

### `Uniswap`

#### Constructor

```typescript
new Uniswap(provider: EthereumProvider, version?: 'v2' | 'v3')
```

**Example:**
```typescript
import { Uniswap } from '@xorion/sdk';

const uniswap = new Uniswap(provider, 'v3');
```

---

#### `swapExactTokensForTokens(amountIn: bigint, amountOutMin: bigint, path: string[], to: string): Promise<TransactionReceipt>`

Swaps tokens on Uniswap.

**Parameters:**
- `amountIn` (bigint): Input amount
- `amountOutMin` (bigint): Minimum output amount
- `path` (string[]): Array of token addresses
- `to` (string): Recipient address

**Returns:** `Promise<TransactionReceipt>`

**Example:**
```typescript
const receipt = await uniswap.swapExactTokensForTokens(
  ethers.parseEther('1'), // 1 ETH
  0n, // Accept any amount of USDC
  [WETH_ADDRESS, USDC_ADDRESS],
  userAddress
);
```

---

#### `getAmountsOut(amountIn: bigint, path: string[]): Promise<bigint[]>`

Calculates output amounts for a swap.

**Parameters:**
- `amountIn` (bigint): Input amount
- `path` (string[]): Token path

**Returns:** `Promise<bigint[]>` - Output amounts for each hop

**Example:**
```typescript
const amounts = await uniswap.getAmountsOut(ethers.parseEther('1'), [WETH_ADDRESS, USDC_ADDRESS]);
console.log('Expected USDC:', amounts[1].toString());
```

---

## Privacy API

### `ProofGenerator`

#### Constructor

```typescript
new ProofGenerator(circuitPath: string, zkeyPath: string)
```

**Parameters:**
- `circuitPath` (string): Path to compiled circuit
- `zkeyPath` (string): Path to zk-SNARK proving key

**Example:**
```typescript
import { ProofGenerator } from '@xorion/zk';

const prover = new ProofGenerator('./circuits/private_transfer.wasm', './circuits/trusted_setup.zkey');
```

---

#### `generatePrivateTransactionProof(inputs: PrivateInputs): Promise<Proof>`

Generates a zk-SNARK proof for a private transaction.

**Parameters:**
- `inputs` (PrivateInputs): Circuit inputs (sender, receiver, amount, etc.)

**Returns:** `Promise<Proof>` - Zero-knowledge proof

**Example:**
```typescript
const proof = await prover.generatePrivateTransactionProof({
  senderBalance: 1000n,
  recipient: '0x...',
  amount: 100n,
  nullifier: generateNullifier()
});

// Submit proof to shielded pool contract
await shieldedPool.submitPrivateTransaction(proof);
```

---

#### `verifyProof(proof: Proof, publicInputs: PublicInputs): Promise<boolean>`

Verifies a zk-SNARK proof.

**Parameters:**
- `proof` (Proof): The proof to verify
- `publicInputs` (PublicInputs): Public circuit inputs

**Returns:** `Promise<boolean>` - True if valid

**Example:**
```typescript
const isValid = await prover.verifyProof(proof, publicInputs);
console.log('Proof valid:', isValid);
```

---

## Storage API

### `IpfsClient`

#### Constructor

```typescript
new IpfsClient(gatewayUrl?: string, options?: IpfsOptions)
```

**Parameters:**
- `gatewayUrl` (string, optional): IPFS gateway URL (default: https://ipfs.io)
- `options` (IpfsOptions, optional): Configuration options

**Example:**
```typescript
import { IpfsClient } from '@xorion/ipfs';

const ipfs = new IpfsClient('https://cloudflare-ipfs.com', {
  timeout: 60000,
  pinning: true
});
```

---

#### `add(data: Uint8Array | string, options?: AddOptions): Promise<Cid>`

Adds data to IPFS.

**Parameters:**
- `data` (Uint8Array | string): Data to store
- `options` (AddOptions, optional): Pinning and encryption options

**Returns:** `Promise<Cid>` - Content identifier

**Example:**
```typescript
const cid = await ipfs.add('Hello, Web3!', { pin: true });
console.log('Stored at:', cid.toString()); // bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi
```

---

#### `cat(cid: string): Promise<Uint8Array>`

Retrieves data from IPFS.

**Parameters:**
- `cid` (string): Content identifier

**Returns:** `Promise<Uint8Array>` - Retrieved data

**Example:**
```typescript
const data = await ipfs.cat('bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi');
console.log(new TextDecoder().decode(data)); // "Hello, Web3!"
```

---

#### `pin(cid: string): Promise<void>`

Pins content to prevent garbage collection.

**Parameters:**
- `cid` (string): Content identifier to pin

**Returns:** `Promise<void>`

**Example:**
```typescript
await ipfs.pin('bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi');
```

---

### `Encryption`

#### `encrypt(data: Uint8Array, password: string): Promise<EncryptedData>`

Encrypts data using AES-256-GCM with Argon2id key derivation.

**Parameters:**
- `data` (Uint8Array): Plaintext data
- `password` (string): Encryption password

**Returns:** `Promise<EncryptedData>` - Encrypted data with metadata

**Example:**
```typescript
import { Encryption } from '@xorion/ipfs';

const encrypted = await Encryption.encrypt(
  new TextEncoder().encode('Secret message'),
  'mySecurePassword'
);

// Store encrypted.ciphertext on IPFS
const cid = await ipfs.add(encrypted.ciphertext);
```

---

#### `decrypt(encryptedData: EncryptedData, password: string): Promise<Uint8Array>`

Decrypts data.

**Parameters:**
- `encryptedData` (EncryptedData): Encrypted data object
- `password` (string): Decryption password

**Returns:** `Promise<Uint8Array>` - Decrypted plaintext

**Example:**
```typescript
const decrypted = await Encryption.decrypt(encrypted, 'mySecurePassword');
console.log(new TextDecoder().decode(decrypted)); // "Secret message"
```

---

## Error Handling

All API methods throw standardized errors:

```typescript
import { XorionError, ErrorCode } from '@xorion/sdk';

try {
  await wallet.sign_transaction(tx);
} catch (error) {
  if (error instanceof XorionError) {
    switch (error.code) {
      case ErrorCode.INSUFFICIENT_FUNDS:
        console.log('Not enough balance');
        break;
      case ErrorCode.NETWORK_ERROR:
        console.log('Network connection failed');
        break;
      case ErrorCode.INVALID_MNEMONIC:
        console.log('Invalid recovery phrase');
        break;
    }
  }
}
```

---

## Rate Limiting

API calls are automatically rate-limited:

- Default: 100 requests per second
- Configurable via provider options
- Automatic retry with exponential backoff

```typescript
const provider = new EthereumProvider(rpcUrl, {
  rateLimit: {
    requestsPerSecond: 50,
    retryDelay: 1000,
    maxRetries: 5
  }
});
```
