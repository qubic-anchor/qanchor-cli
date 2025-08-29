# {{project_name_pascal}} AMM

> A **Decentralized Finance (DeFi) Automated Market Maker (AMM)** built with QAnchor

{{project_name_pascal}} is a fully-featured AMM that enables decentralized token swaps through automated liquidity pools. Built using the QAnchor framework, it provides constant product market making with configurable fees.

## Features

- 🏊 **Liquidity Pools**: Create and manage token pair pools
- 🔄 **Token Swaps**: Automated price discovery and execution
- 💰 **Liquidity Mining**: Earn fees by providing liquidity
- ⚙️ **Configurable Fees**: Set custom fee rates (0.01% - 100%)
- 🛡️ **Slippage Protection**: Built-in slippage tolerance checks
- 📊 **Real-time Events**: Track all pool activities

## Quick Start

### Prerequisites

- [QAnchor CLI](https://github.com/qubic-anchor/qanchor-cli) installed
- Qubic wallet with some QUBIC tokens for testing

### Build & Deploy

```bash
# Build the AMM contract
qanchor build

# Deploy to local testnet
qanchor deploy --network local

# Deploy to Qubic mainnet
qanchor deploy --network mainnet --yes
```

### Testing

```bash
# Run all tests
qanchor test

# Run tests with network integration
qanchor test --network local --verbose
```

## Contract Architecture

### Core Concepts

#### Liquidity Pool
A liquidity pool holds reserves of two tokens and uses the **constant product formula** (x × y = k) to determine swap prices:

- **Token A & B**: The two tokens in the trading pair
- **Reserves**: Amount of each token held in the pool
- **LP Tokens**: Represent a liquidity provider's share of the pool
- **Fee Rate**: Trading fee charged to swappers (paid to LPs)

#### Price Discovery
Prices are automatically determined by the ratio of token reserves:
```
Price of Token A = Reserve B / Reserve A
Price of Token B = Reserve A / Reserve B
```

### Smart Contract Functions

#### 1. Initialize Pool
```rust
pub fn initialize_pool(
    ctx: Context<InitializePool>,
    token_a: [u8; 32],
    token_b: [u8; 32],
    fee_rate: u16, // Basis points (e.g., 30 = 0.3%)
) -> Result<()>
```

Creates a new liquidity pool for a token pair.

#### 2. Add Liquidity
```rust
pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    amount_a_desired: u64,
    amount_b_desired: u64,
    amount_a_min: u64,    // Slippage protection
    amount_b_min: u64,    // Slippage protection
) -> Result<LiquidityResult>
```

Provides liquidity to an existing pool and receives LP tokens.

#### 3. Remove Liquidity
```rust
pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    lp_token_amount: u64,
    amount_a_min: u64,    // Slippage protection
    amount_b_min: u64,    // Slippage protection
) -> Result<()>
```

Burns LP tokens and withdraws underlying tokens.

#### 4. Swap Tokens
```rust
pub fn swap(
    ctx: Context<Swap>,
    amount_in: u64,
    amount_out_min: u64,  // Slippage protection
    token_a_to_b: bool,   // Swap direction
) -> Result<SwapResult>
```

Exchanges one token for another through the pool.

## QIDL Interface Definition

The AMM contract uses a comprehensive QIDL definition (`src/amm.qidl`) that includes:

### Key Configuration (`qanchor.yaml`)
```yaml
[contract]
name = "{{project_name_pascal}}AMM"
source = "src/lib.rs"
qidl = "src/amm.qidl"    # ← AMM-specific QIDL file

[networks]
local = "http://localhost:8080"
testnet = "https://testnet-rpc.qubic.org"
mainnet = "https://rpc.qubic.org"
```

### QIDL Highlights
- **4 Instructions**: `initializePool`, `addLiquidity`, `removeLiquidity`, `swap`
- **Complex Account Constraints**: PDA derivation, initialization constraints
- **Rich Type Definitions**: `LiquidityResult`, `SwapResult`, `PoolStatistics`
- **Event System**: Track all pool activities with structured events
- **Error Handling**: 10 comprehensive error codes
- **Constants**: `MINIMUM_LIQUIDITY`, fee rate limits

## Usage Examples

### Creating a Pool

```typescript
import { Program, AnchorProvider } from '@project-serum/anchor';
import { AMM } from './target/types/amm';

const program = new Program<AMM>(idl, programId, provider);

// Create USDC/QUBIC pool with 0.3% fee
await program.methods
  .initializePool(
    usdcMint.publicKey,
    qubicMint.publicKey,
    30 // 0.3% fee
  )
  .accounts({
    pool: poolPda,
    authority: wallet.publicKey,
    tokenAVault: usdcVault,
    tokenBVault: qubicVault,
  })
  .rpc();
```

### Adding Liquidity

```typescript
// Add 1000 USDC and 500 QUBIC to pool
const result = await program.methods
  .addLiquidity(
    new BN(1000_000000), // 1000 USDC (6 decimals)
    new BN(500_000000),  // 500 QUBIC
    new BN(950_000000),  // Min 950 USDC (5% slippage)
    new BN(475_000000)   // Min 475 QUBIC (5% slippage)
  )
  .accounts({
    pool: poolPda,
    userTokenAAccount: userUsdcAccount,
    userTokenBAccount: userQubicAccount,
    poolTokenAVault: poolUsdcVault,
    poolTokenBVault: poolQubicVault,
    user: wallet.publicKey,
  })
  .rpc();

console.log('LP tokens received:', result.lpTokensMinted.toString());
```

### Swapping Tokens

```typescript
// Swap 100 USDC for QUBIC
const swapResult = await program.methods
  .swap(
    new BN(100_000000), // 100 USDC input
    new BN(45_000000),  // Min 45 QUBIC output (10% slippage)
    true                // USDC → QUBIC direction
  )
  .accounts({
    pool: poolPda,
    userSourceAccount: userUsdcAccount,
    userDestinationAccount: userQubicAccount,
    poolSourceVault: poolUsdcVault,
    poolDestinationVault: poolQubicVault,
    user: wallet.publicKey,
  })
  .rpc();

console.log('Received QUBIC:', swapResult.amountOut.toString());
console.log('Price impact:', swapResult.priceImpact / 100, '%');
```

## Advanced Features

### Fee Structure
- Trading fees are charged on input amount
- Fees are added to pool reserves (benefit LPs)
- Fee rates configurable from 0.01% to 100%
- Typical DeFi rates: 0.05% (Uniswap v3), 0.3% (Uniswap v2)

### Slippage Protection
All functions include slippage protection:
- `amount_a_min`/`amount_b_min` for liquidity operations
- `amount_out_min` for swaps
- Transactions revert if slippage exceeds tolerance

### Price Impact Calculation
```rust
price_impact = (amount_out * 10000 / reserve_out) as u32; // In basis points
```

### Event Monitoring

Listen to AMM events for real-time updates:

```typescript
// Listen for swaps
program.addEventListener('TokenSwapped', (event) => {
  console.log('Swap detected:', {
    user: event.user.toString(),
    amountIn: event.amountIn.toString(),
    amountOut: event.amountOut.toString(),
    feeAmount: event.feeAmount.toString(),
  });
});

// Listen for liquidity changes
program.addEventListener('LiquidityAdded', (event) => {
  console.log('Liquidity added:', {
    user: event.user.toString(),
    amountA: event.amountA.toString(),
    amountB: event.amountB.toString(),
    lpTokens: event.lpTokensMinted.toString(),
  });
});
```

## Security Considerations

### Implemented Protections
- ✅ **Slippage Protection**: User-defined minimum outputs
- ✅ **Overflow Protection**: Safe arithmetic operations
- ✅ **Input Validation**: Non-zero amounts, valid fee rates
- ✅ **Token Pair Validation**: Prevent same-token pools
- ✅ **Reserve Checks**: Prevent draining pools

### Audit Recommendations
- Formal verification of AMM formula implementation
- Flash loan attack vector analysis
- MEV (Maximal Extractable Value) consideration
- Integration testing with mainnet conditions

## Development

### Project Structure
```
{{project_name}}/
├── src/
│   ├── lib.rs           # Main AMM contract
│   └── amm.qidl         # Interface definition
├── tests/
│   └── amm.test.ts      # Integration tests
├── target/
│   ├── debug/
│   │   └── contract.wasm
│   └── qidl/
│       └── contract.json # Generated QIDL
└── qanchor.yaml         # Project configuration
```

### QIDL Toolchain

```bash
# Validate QIDL structure
qanchor qidl validate src/amm.qidl

# Generate QIDL from source
qanchor qidl generate --source src/lib.rs --output target/qidl/contract.json

# Compare QIDL versions
qanchor qidl diff target/qidl/contract.v1.json target/qidl/contract.v2.json

# Format QIDL file
qanchor qidl format src/amm.qidl --in-place
```

### Building & Testing

```bash
# Full development cycle
qanchor build              # Compile + generate QIDL
qanchor test --verbose     # Run comprehensive tests
qanchor deploy --network local --yes  # Deploy to testnet
```

## Mathematical Foundation

### Constant Product Formula

The AMM implements the constant product market maker model:

```
x × y = k
```

Where:
- `x` = Reserve of Token A
- `y` = Reserve of Token B  
- `k` = Constant (invariant)

### Swap Calculation

For a swap of `Δx` tokens A for `Δy` tokens B:

```
(x + Δx) × (y - Δy) = k
```

Solving for `Δy`:
```
Δy = (y × Δx) / (x + Δx)
```

With fees:
```
Δx_with_fee = Δx × (1 - fee_rate)
Δy = (y × Δx_with_fee) / (x + Δx_with_fee)
```

### Liquidity Calculations

**Initial liquidity:**
```
L = √(x × y)
```

**Subsequent liquidity:**
```
L_new = L_old × (Δx / x) = L_old × (Δy / y)
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- 📖 [QAnchor Documentation](https://docs.qanchor.dev)
- 💬 [Community Discord](https://discord.gg/qanchor)
- 🐛 [Issue Tracker](https://github.com/qubic-anchor/qanchor-cli/issues)
- 📧 [Email Support](mailto:support@qanchor.dev)

---

**Built with ❤️ using [QAnchor](https://github.com/qubic-anchor/qanchor-cli) - The Anchor for Qubic**
