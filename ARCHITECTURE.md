# Architecture

## Design Principles

Xorion is built on the following core design principles:

1. **Security First**: All cryptographic operations use constant-time implementations, private keys never leave secure enclaves, and all code undergoes rigorous security audits.

2. **Modularity**: Each component is designed as an independent crate/module with well-defined interfaces, enabling easy maintenance and testing.

3. **Cross-Chain Native**: Multi-chain support is not an afterthought—it's baked into the core architecture from day one.

4. **Privacy by Default**: zk-SNARKs privacy features are available for all transactions without compromising usability.

5. **Decentralized Storage**: IPFS integration ensures user data and dApps can be stored in a censorship-resistant manner.

6. **Community Governance**: DAO governance allows token holders to participate in protocol decisions.

## 10-Phase Breakdown

### Phase 1: Multi-Chain Wallet Core
```
┌─────────────────────────────────────┐
│         Wallet Core                 │
├─────────────────────────────────────┤
│  - Mnemonic Generation (BIP-39)     │
│  - HD Key Derivation (BIP-44)       │
│  - Address Derivation               │
│    - Ethereum (secp256k1 + Keccak)  │
│    - Solana (Ed25519)               │
└─────────────────────────────────────┘
```

### Phase 2: RPC Integration Layer
```
┌─────────────────────────────────────┐
│         RPC Layer                   │
├─────────────────────────────────────┤
│  - EthereumProvider                 │
│  - SolanaProvider                   │
│  - Auto Network Switching           │
│  - Request Batching & Caching       │
└─────────────────────────────────────┘
```

### Phase 3: Smart Contract Integration
```
┌─────────────────────────────────────┐
│      Contract Layer                 │
├─────────────────────────────────────┤
│  - ERC20 Interface                  │
│  - ERC721 Interface                 │
│  - Uniswap V2/V3 Router             │
│  - ABI Encoder/Decoder              │
└─────────────────────────────────────┘
```

### Phase 4: Redox Scheme Implementation
```
┌─────────────────────────────────────┐
│      Redox Scheme                   │
├─────────────────────────────────────┤
│  - wallet:/ URL Handler             │
│  - Cross-Chain Atomic Swaps         │
│  - Transaction State Machine        │
│  - Rollback & Recovery              │
└─────────────────────────────────────┘
```

### Phase 5: GUI Development
```
┌─────────────────────────────────────┐
│         Desktop GUI                 │
├─────────────────────────────────────┤
│  - Tauri/Electron Framework         │
│  - Real-time Balance Display        │
│  - Transaction History              │
│  - Multi-Account Management         │
│  - Dark/Light Themes                │
└─────────────────────────────────────┘
```

### Phase 6: WASM Runtime
```
┌─────────────────────────────────────┐
│       WASM Runtime                  │
├─────────────────────────────────────┤
│  - Wasmtime Sandbox                 │
│  - Permission System                │
│  - Gas Metering                     │
│  - Host Functions Bridge            │
└─────────────────────────────────────┘
```

### Phase 7: zk-SNARKs Privacy Layer
```
┌─────────────────────────────────────┐
│      Privacy Layer                  │
├─────────────────────────────────────┤
│  - Groth16 Proving System           │
│  - Private Transactions             │
│  - Shielded Pools                   │
│  - Balance Proofs                   │
└─────────────────────────────────────┘
```

### Phase 8: IPFS Distributed Storage
```
┌─────────────────────────────────────┐
│      IPFS Storage                   │
├─────────────────────────────────────┤
│  - File Add/Retrieve                │
│  - Client-Side Encryption           │
│  - Pinning Service                  │
│  - Gateway Fallback                 │
└─────────────────────────────────────┘
```

### Phase 9: DAO Governance
```
┌─────────────────────────────────────┐
│      DAO Governance                 │
├─────────────────────────────────────┤
│  - Proposal System                  │
│  - Token-Weighted Voting            │
│  - Treasury Management              │
│  - Timelock Execution               │
└─────────────────────────────────────┘
```

### Phase 10: Beta Release
```
┌─────────────────────────────────────┐
│       Production Ready              │
├─────────────────────────────────────┤
│  - Security Audits Complete         │
│  - Documentation Complete           │
│  - CI/CD Pipeline                   │
│  - Community Beta Program           │
└─────────────────────────────────────┘
```

## Component Interaction Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                         Xorion Architecture                       │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐          │
│  │   GUI       │    │   WASM      │    │   CLI       │          │
│  │  (Tauri)    │    │   Runtime   │    │  Tools      │          │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘          │
│         │                  │                  │                   │
│         └──────────────────┼──────────────────┘                   │
│                            │                                      │
│                   ┌────────▼────────┐                             │
│                   │   Wallet SDK    │                             │
│                   │   (xorion-sdk)  │                             │
│                   └────────┬────────┘                             │
│                            │                                      │
│         ┌──────────────────┼──────────────────┐                   │
│         │                  │                  │                   │
│  ┌──────▼──────┐   ┌──────▼──────┐   ┌──────▼──────┐            │
│  │  Ethereum   │   │   Solana    │   │   Other     │            │
│  │  Provider   │   │  Provider   │   │  Providers  │            │
│  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘            │
│         │                 │                  │                   │
│         └─────────────────┼──────────────────┘                   │
│                           │                                      │
│              ┌────────────▼────────────┐                         │
│              │    Blockchain Networks   │                        │
│              │  (ETH, SOL, MATIC, ...)  │                        │
│              └──────────────────────────┘                        │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐     │
│  │              Supporting Services                        │     │
│  ├─────────────────────────────────────────────────────────┤     │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐              │     │
│  │  │ zk-SNARK │  │  IPFS    │  │   DAO    │              │     │
│  │  │  Layer   │  │ Storage  │  │Governance│              │     │
│  │  └──────────┘  └──────────┘  └──────────┘              │     │
│  └─────────────────────────────────────────────────────────┘     │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

## Data Flow: Transaction Lifecycle

```
1. User Initiates Transaction
         │
         ▼
2. GUI/CLI Captures Details
   (recipient, amount, chain)
         │
         ▼
3. Wallet SDK Validates
   - Balance check
   - Gas estimation
   - Nonce management
         │
         ▼
4. Transaction Construction
   - Build transaction object
   - Set gas limits & prices
         │
         ▼
5. Signing (Secure Enclave)
   - Retrieve private key
   - Sign transaction
   - Clear key from memory
         │
         ▼
6. Optional: Privacy Layer
   - Generate zk-SNARK proof
   - Create shielded transaction
         │
         ▼
7. Broadcast via RPC
   - Select optimal RPC endpoint
   - Submit transaction
   - Get transaction hash
         │
         ▼
8. Monitoring & Confirmation
   - Poll for confirmations
   - Update UI status
   - Store in history
         │
         ▼
9. Completion
   - Notify user
   - Update balance
   - Log transaction
```

## Security Model

### Threat Mitigation

| Threat | Mitigation |
|--------|------------|
| Private Key Theft | Keys stored in OS secure enclave, never in plaintext |
| Man-in-the-Middle | TLS for all RPC connections, certificate pinning |
| Replay Attacks | Nonce management, chain ID verification |
| Front-running | Private transactions via zk-SNARKs |
| DoS on RPC | Rate limiting, multiple endpoint failover |
| Supply Chain Attacks | Dependency auditing, reproducible builds |
| Memory Scraping | Constant-time crypto, immediate key clearing |

### Trust Assumptions

1. **RPC Providers**: Trusted for liveness, not privacy (use private transactions for sensitive ops)
2. **IPFS Nodes**: Content-addressed storage ensures integrity
3. **zk-SNARK Setup**: Trusted setup ceremony conducted transparently
4. **Smart Contracts**: Audited contracts only, community verification encouraged

## Decision Rationale

### Why Rust?
- Memory safety without garbage collection
- Excellent cryptography libraries
- WebAssembly compilation target
- Growing ecosystem in blockchain space

### Why Multiple Chains?
- Users hold assets across chains
- Avoid vendor lock-in
- Leverage best features of each chain
- Future-proof architecture

### Why zk-SNARKs?
- True privacy for transactions
- Regulatory compliance potential (selective disclosure)
- Competitive differentiation
- Aligns with Web3 privacy values

### Why IPFS?
- Censorship resistance
- Reduced hosting costs
- Alignment with decentralization ethos
- Content addressing ensures integrity

### Why DAO Governance?
- Community ownership
- Transparent decision-making
- Aligned incentives
- Sustainable long-term development
