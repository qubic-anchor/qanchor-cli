# QIDL (Qubic Interface Definition Language) Specification v1.0.0

## Overview

QIDL (Qubic Interface Definition Language) is a standardized JSON format for describing Qubic smart contracts, inspired by Solana's Anchor IDL. It provides a machine-readable interface definition that enables automatic SDK generation, type-safe client libraries, and development tooling.

## File Format

QIDL files are JSON documents that describe the complete interface of a Qubic smart contract program.

### File Extension
- **Primary**: `.qidl` (for template files)  
- **Build Output**: `.json` (for generated files)

### Encoding
- **Character Set**: UTF-8
- **Formatting**: Pretty-printed JSON with 2-space indentation

## Root Structure

```json
{
  "version": "1.0.0",
  "spec": "1.0.0", 
  "program": { ... },
  "instructions": [ ... ],
  "accounts": [ ... ],
  "types": [ ... ],
  "events": [ ... ],
  "errors": [ ... ],
  "constants": [ ... ],
  "metadata": { ... }
}
```

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `version` | string | Program version (semver) |
| `spec` | string | QIDL specification version |
| `program` | ProgramInfo | Program metadata |
| `instructions` | Instruction[] | Program instructions |
| `accounts` | Account[] | Account type definitions |
| `types` | TypeDef[] | Custom type definitions |
| `events` | Event[] | Event definitions |
| `errors` | ErrorDef[] | Error definitions |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `constants` | Constant[] | Program constants |
| `metadata` | QidlMetadata | Build and generation metadata |

## Data Types

### ProgramInfo

```json
{
  "name": "MyProgram",
  "description": "A sample Qubic program",
  "version": "1.0.0",
  "authors": ["Developer <dev@example.com>"],
  "program_id": "QC4075207365...",
  "license": "MIT",
  "repository": "https://github.com/user/repo"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✅ | Program name |
| `description` | string | ✅ | Program description |
| `version` | string | ✅ | Program version |
| `authors` | string[] | ✅ | List of authors |
| `program_id` | string | ❌ | Deployed program ID |
| `license` | string | ❌ | License identifier |
| `repository` | string | ❌ | Repository URL |

### Instruction

```json
{
  "name": "initializePool",
  "description": "Create a new liquidity pool",
  "args": [ ... ],
  "accounts": [ ... ],
  "returns": { ... },
  "discriminator": [23, 90, 119, 42, 209, 31, 104, 59],
  "example": "await program.methods.initializePool(...).rpc()"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✅ | Instruction name (camelCase) |
| `description` | string | ✅ | Human-readable description |
| `args` | Argument[] | ✅ | Instruction arguments |
| `accounts` | AccountRef[] | ✅ | Required accounts |
| `returns` | ReturnType | ❌ | Return type |
| `discriminator` | u8[8] | ❌ | 8-byte instruction discriminator |
| `example` | string | ❌ | Usage example |

### Argument

```json
{
  "name": "amount",
  "type": "u64",
  "description": "Amount to transfer",
  "validation": {
    "min": 1,
    "max": 1000000
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✅ | Argument name |
| `type` | string | ✅ | Type identifier |
| `description` | string | ❌ | Argument description |
| `validation` | ValidationRules | ❌ | Validation constraints |

### AccountRef

```json
{
  "name": "pool",
  "type": "LiquidityPool", 
  "mutable": true,
  "signer": false,
  "description": "The liquidity pool account",
  "constraints": [
    {
      "type": "init",
      "payer": "authority",
      "space": 256
    }
  ],
  "optional": false
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✅ | Account name |
| `type` | string | ✅ | Account type |
| `mutable` | boolean | ❌ | Whether account is writable |
| `signer` | boolean | ❌ | Whether account must sign |
| `description` | string | ❌ | Account description |
| `constraints` | AccountConstraint[] | ❌ | Account constraints |
| `optional` | boolean | ❌ | Whether account is optional |

### Account

```json
{
  "name": "LiquidityPool",
  "description": "AMM liquidity pool state",
  "fields": [
    {
      "name": "reserveA",
      "type": "u64",
      "description": "Token A reserves"
    }
  ]
}
```

### AccountConstraint

Account constraints define validation rules and initialization parameters:

#### Init Constraint
```json
{
  "type": "init",
  "payer": "authority",
  "space": 256,
  "seeds": ["pool", "tokenA", "tokenB"]
}
```

#### Owner Constraint
```json
{
  "type": "owner",
  "program": "TokenProgram"
}
```

#### Seeds Constraint (PDA)
```json
{
  "type": "seeds",
  "seeds": ["pool", "tokenA", "tokenB"],
  "bump": "poolBump"
}
```

#### Custom Constraint
```json
{
  "type": "custom",
  "expression": "pool.authority == authority.key()",
  "error": "Unauthorized"
}
```

## Built-in Types

### Primitive Types

| Type | Description | TypeScript | Python |
|------|-------------|------------|--------|
| `u8` | 8-bit unsigned integer | `number` | `int` |
| `u16` | 16-bit unsigned integer | `number` | `int` |
| `u32` | 32-bit unsigned integer | `number` | `int` |
| `u64` | 64-bit unsigned integer | `number` | `int` |
| `i8` | 8-bit signed integer | `number` | `int` |
| `i16` | 16-bit signed integer | `number` | `int` |
| `i32` | 32-bit signed integer | `number` | `int` |
| `i64` | 64-bit signed integer | `number` | `int` |
| `bool` | Boolean | `boolean` | `bool` |
| `string` | UTF-8 string | `string` | `str` |
| `bytes` | Byte array | `Uint8Array` | `bytes` |

### Qubic-Specific Types

| Type | Description | Representation |
|------|-------------|----------------|
| `PublicKey` | Qubic public key (32 bytes) | Base58 string |
| `Signature` | Ed25519 signature | Base64 string |
| `Hash` | K12 hash (32 bytes) | Hex string |

### Container Types

| Type | Format | Example |
|------|---------|---------|
| Array | `T[]` | `u64[]` |
| Vector | `Vec<T>` | `Vec<string>` |
| Option | `Option<T>` | `Option<u64>` |
| HashMap | `HashMap<K, V>` | `HashMap<string, u64>` |

## Validation Rules

### ValidationRules

```json
{
  "min": 1,
  "max": 1000000,
  "min_length": 1,
  "max_length": 255
}
```

| Field | Type | Applies To | Description |
|-------|------|------------|-------------|
| `min` | u64 | Numbers | Minimum value |
| `max` | u64 | Numbers | Maximum value |
| `min_length` | usize | Strings/Arrays | Minimum length |
| `max_length` | usize | Strings/Arrays | Maximum length |

## Events

```json
{
  "name": "TokenSwapped",
  "description": "Emitted when tokens are swapped",
  "fields": [
    {
      "name": "user",
      "type": "PublicKey"
    },
    {
      "name": "amountIn",
      "type": "u64"
    }
  ]
}
```

## Error Definitions

```json
{
  "code": 6000,
  "name": "InsufficientLiquidity", 
  "message": "Insufficient liquidity for this operation"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `code` | u32 | Error code (6000+) |
| `name` | string | Error name |
| `message` | string | Error message |

## Constants

```json
{
  "name": "MINIMUM_LIQUIDITY",
  "value": 1000,
  "type": "u64",
  "description": "Minimum initial liquidity"
}
```

## Metadata

```json
{
  "compiler_version": "0.3.1",
  "generated_at": "2025-08-29T09:30:00.000000+00:00",
  "source_hash": "710b0d81f549c593",
  "build_args": ["--optimize"],
  "dependencies": [
    {
      "name": "qanchor-lang",
      "version": "0.3.1",
      "type": "crate"
    }
  ]
}
```

## Naming Conventions

### Instructions
- Use **camelCase** for instruction names
- Start with a verb: `initialize`, `addLiquidity`, `swap`

### Accounts & Types
- Use **PascalCase** for type names
- Be descriptive: `LiquidityPool`, `SwapResult`

### Fields
- Use **camelCase** for field names
- Be concise but clear: `tokenA`, `reserveAmount`

### Constants
- Use **SCREAMING_SNAKE_CASE**
- Use descriptive names: `MINIMUM_LIQUIDITY`, `MAX_FEE_RATE`

## SDK Generation

QIDL enables automatic generation of type-safe client SDKs:

### TypeScript SDK
```typescript
// Generated from QIDL
const result = await program.methods
  .addLiquidity(
    new BN(1000), // amountADesired
    new BN(2000), // amountBDesired  
    new BN(950),  // amountAMin
    new BN(1900)  // amountBMin
  )
  .accounts({
    pool: poolPda,
    userTokenAAccount: userTokenA,
    userTokenBAccount: userTokenB,
    user: wallet.publicKey,
  })
  .rpc();
```

### Python SDK
```python
# Generated from QIDL
result = await program.rpc.add_liquidity(
    amount_a_desired=1000,
    amount_b_desired=2000,
    amount_a_min=950,
    amount_b_min=1900,
    ctx=Context(
        accounts={
            "pool": pool_pda,
            "user_token_a_account": user_token_a,
            "user_token_b_account": user_token_b,
            "user": wallet.public_key,
        }
    )
)
```

## Version Compatibility

### Semantic Versioning

QIDL follows semantic versioning:
- **Major**: Breaking changes to specification
- **Minor**: Backward-compatible additions  
- **Patch**: Bug fixes and clarifications

### Compatibility Matrix

| QIDL Spec | QAnchor CLI | Supported |
|-----------|-------------|-----------|
| 1.0.x | 0.3.1+ | ✅ |
| 0.x.x | < 0.3.1 | ❌ |

## Examples

See the [examples directory](../src/templates/) for complete QIDL examples:

- **Basic Oracle**: Simple price feed contract
- **DeFi AMM**: Automated Market Maker with liquidity pools
- **NFT Collection**: Non-fungible token collection
- **Governance**: DAO governance and voting

## Tools

### QAnchor CLI

```bash
# Generate QIDL from source code
qanchor qidl generate --source src/lib.rs --output target/qidl/contract.json

# Validate QIDL file
qanchor qidl validate target/qidl/contract.json

# Compare QIDL versions
qanchor qidl diff old.qidl new.qidl

# Format QIDL file
qanchor qidl format target/qidl/contract.json --in-place
```

### Build Integration

QIDL generation is automatically integrated into the build process:

```bash
qanchor build  # Automatically generates QIDL
```

## Best Practices

### 1. Documentation
- Provide clear descriptions for all instructions, accounts, and types
- Include usage examples for complex instructions
- Document validation constraints and their rationale

### 2. Versioning
- Use semantic versioning for programs
- Maintain backward compatibility when possible
- Document breaking changes in QIDL updates

### 3. Type Safety
- Use specific types instead of generic ones
- Define custom types for complex data structures
- Leverage validation rules for input constraints

### 4. Error Handling
- Define meaningful error codes and messages
- Use error codes starting from 6000
- Group related errors numerically

### 5. Events
- Emit events for all significant state changes
- Include relevant context in event data
- Use consistent naming for similar events

## Future Extensions

### Planned Features

- **Cross-Program Invocation (CPI)**: Define interfaces for calling other programs
- **Account Relationships**: Express dependencies between accounts
- **State Transitions**: Define valid state change patterns
- **Economic Models**: Describe fee structures and tokenomics

### Extension Points

QIDL is designed to be extensible. Future versions may add:

- Additional constraint types
- New validation rules
- Enhanced metadata fields
- Integration with formal verification tools

---

*This specification is maintained by the QAnchor project. For updates and discussions, visit our [GitHub repository](https://github.com/qubic-anchor/qanchor-cli).*
