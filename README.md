# 🚀 QAnchor CLI
> The Anchor for Qubic - Modern development framework for Qubic blockchain

QAnchor is a Qubic development framework inspired by Solana Anchor, providing user-friendly CLI tools and a complete development experience.

## 💡 Why QAnchor?

If you're familiar with Solana's Anchor framework, you'll feel right at home with QAnchor.

### Solana vs Qubic Development Experience
| Task | Solana + Anchor | Qubic Native | QAnchor |
|------|----------------|--------------|---------|
| Project setup | `anchor init` (30s) | Manual setup (30min) | `qanchor init` (30s) |
| Build | `anchor build` | Complex C++ compilation | `qanchor build` |
| Deploy | `anchor deploy` | Manual deployment | `qanchor deploy` |
| Test | `anchor test` | No framework | `qanchor test` |

## ⚡ Quick Start

```bash
# Install QAnchor (Development Build)
git clone https://github.com/qubic-anchor/qanchor-cli.git
cd qanchor-cli
cargo build --release

# Install globally (optional)
sudo cp target/release/qanchor /usr/local/bin/
# Or use directly: ./target/release/qanchor

# Create a new project
qanchor init my-oracle
cd my-oracle

# Complete development workflow
qanchor build                                    # Compile contract
qanchor deploy --network local                  # Deploy to local network
qanchor test                                     # Run tests

# Generate client SDKs
qanchor generate --lang ts --output ./ts-sdk    # TypeScript SDK
qanchor generate --lang py --output ./py-sdk    # Python SDK

# Start local development network (optional)
qanchor localnet --port 8899                    # HTTP API on localhost:8899
```

## 📁 Project Structure

### QAnchor CLI Tool Structure
```
qanchor-cli/
├── Cargo.toml                  # Rust dependencies and metadata
├── README.md                   # This documentation
├── CHANGELOG.md                # Version history and changes
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── cli/                    # Command-line interface
│   │   ├── commands/           # Individual CLI commands
│   │   │   ├── init.rs         # Project initialization
│   │   │   ├── build.rs        # Contract compilation
│   │   │   ├── deploy.rs       # Contract deployment
│   │   │   ├── test.rs         # Test execution
│   │   │   ├── generate.rs     # SDK generation
│   │   │   ├── localnet.rs     # Local test network
│   │   │   └── clean.rs        # Cleanup utilities
│   │   └── mod.rs
│   ├── qidl/                   # QIDL parser and types
│   │   ├── types.rs            # QIDL data structures
│   │   └── parser.rs           # QIDL file parsing
│   ├── generators/             # SDK generators
│   │   ├── base.rs             # Common generator interface
│   │   ├── typescript.rs       # TypeScript SDK generator
│   │   ├── python.rs           # Python SDK generator
│   │   └── templates/          # Handlebars templates
│   │       ├── typescript/     # TS templates
│   │       └── python/         # Python templates
│   ├── localnet/               # Local test network
│   │   ├── server.rs           # HTTP server
│   │   ├── api.rs              # REST API endpoints
│   │   └── state.rs            # Blockchain state simulation
│   ├── templates/              # Project templates
│   │   └── basic_oracle/       # Default Oracle template
│   ├── config/                 # Configuration handling
│   ├── utils/                  # Utility functions
│   └── error.rs                # Error definitions
└── tests/
    └── integration_tests.rs    # End-to-end tests
```

### Generated QAnchor Project Structure
```
my-oracle/                      # Your project name
├── qanchor.yaml                # Project configuration
├── README.md                   # Project documentation
├── LICENSE                     # MIT License file
├── .gitignore                  # Git ignore rules
├── src/
│   ├── lib.rs                  # Rust contract implementation
│   └── oracle.qidl             # QIDL interface definition
├── tests/
│   └── oracle.test.ts          # TypeScript tests
├── target/                     # Build artifacts (after qanchor build)
│   ├── debug/
│   │   └── contract.wasm       # Compiled contract
│   └── qidl/
│       └── contract.json       # Parsed QIDL
└── generated/                  # Generated SDKs (after qanchor generate)
    ├── typescript/             # TypeScript SDK
    │   ├── types.ts
    │   ├── client.ts
    │   ├── index.ts
    │   └── package.json
    └── python/                 # Python SDK
        ├── types.py
        ├── client.py
        ├── requirements.txt
        └── __init__.py
```

## 🎯 Features

### ✅ Currently Available
- 🔥 **Zero Learning Curve**: If you know Anchor, you know QAnchor
- ⚡ **30-Second Setup**: From zero to running in 30 seconds
- 🛡️ **Type Safety**: QIDL-driven SDK generation (TypeScript & Python)
- ✅ **Complete CLI Tools** - Anchor-inspired command interface
- ✅ **Project Template System** - Quick Oracle contract scaffolding
- ✅ **Beautiful Output** - Colored text and progress bars
- ✅ **Local Test Network** - HTTP API for development
- ✅ **SDK Generation** - TypeScript and Python client libraries
- ✅ **Comprehensive Testing** - Integration tests and validation

### 🚧 Coming Soon (Phase 3)
- 🧪 **Time Travel Testing**: Snapshot and replay capabilities
- 🎨 **IDE Integration**: Full VSCode support with syntax highlighting
- 🌐 **Real Network Integration**: Mainnet and testnet support
- 📦 **Package Registry**: npm and PyPI publishing

## 📚 Command Reference

### `qanchor init <project-name>`
Create a new QAnchor project with complete project structure and template files.

### `qanchor build`
Compile Rust contracts and generate WASM and QIDL files.

### `qanchor deploy [--network <network>]`
Deploy contracts to specified network (defaults to local).

### `qanchor test [--pattern <pattern>]`
Run project tests with support for test pattern filtering.

### `qanchor generate --lang <ts|py>`
Generate TypeScript or Python SDK from QIDL interface definition.

### `qanchor localnet [--port <port>]`
Start a local Qubic test network for development.

### `qanchor clean [--cache-only]`
Clean build artifacts and cache files.

### `qanchor --version`
Display QAnchor CLI version information.

## 📖 SDK Usage Examples

### TypeScript SDK

After generating a TypeScript SDK with `qanchor generate --lang ts --output ./ts-sdk`:

```typescript
// Install generated SDK dependencies
cd ts-sdk && npm install

// Use in your application
import { OracleClient, UpdatePriceArgs } from './ts-sdk';

// Initialize client (connects to local Qubic network)
const client = new OracleClient('http://localhost:8899');

// Call contract methods
const args: UpdatePriceArgs = {
  asset_id: 'BTC',
  price: 50000,
  timestamp: Date.now()
};

// Example: Update price data
const result = await client.updatePrice(args);
console.log('Transaction ID:', result.transaction_id);

// Example: Get current price
const price = await client.getPrice({ asset_id: 'BTC' });
console.log('Current BTC price:', price.price);
```

### Python SDK

After generating a Python SDK with `qanchor generate --lang py --output ./py-sdk`:

```python
# Install generated SDK dependencies
cd py-sdk && pip install -r requirements.txt

# Use in your application
import asyncio
import time
from py_sdk import OracleClient, UpdatePriceArgs

async def main():
    # Initialize client (connects to local Qubic network)
    client = OracleClient('http://localhost:8899')
    
    # Call contract methods
    args = UpdatePriceArgs(
        asset_id='BTC',
        price=50000,
        timestamp=int(time.time())
    )
    
    # Example: Update price data
    result = await client.update_price(args)
    print(f'Transaction ID: {result.transaction_id}')
    
    # Example: Get current price
    price = await client.get_price(asset_id='BTC')
    print(f'Current BTC price: {price.price}')

# Run the example
asyncio.run(main())
```

## 🌐 Local Development Network

Start a local Qubic test network for development:

```bash
# Start local network (runs on http://localhost:8899)
qanchor localnet

# In another terminal, deploy your contract
qanchor deploy --network local
```

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Network health status |
| `/contracts` | POST | Deploy a new contract |
| `/contracts/:id` | GET | Get contract information |
| `/contracts/:id/call` | POST | Call contract method |
| `/blocks` | GET | Get latest block |

### Contract Deployment

**Request Example:**
```bash
curl -X POST http://localhost:8899/contracts \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-oracle",
    "code": "SGVsbG8gUXViaWMgQ29udHJhY3Q=",
    "description": "Price Oracle Contract"
  }'
```

**Success Response (201 Created):**
```json
{
  "contract_id": "QC4075207365",
  "status": "deployed",
  "block_height": 0,
  "transaction_id": "5c62aa5b-c96e-4331-a334-7c50e0e1ed74",
  "deployed_at": 1756262706
}
```

**Error Response (400 Bad Request):**
```json
{
  "error": "Invalid Base64 encoding",
  "code": 400,
  "details": "Base64 decode error: Invalid symbol 45, offset 7."
}
```

### Contract Interaction

**Call Contract Method:**
```bash
curl -X POST http://localhost:8899/contracts/QC4075207365/call \
  -H "Content-Type: application/json" \
  -d '{
    "method": "get_price",
    "args": {"asset_id": "BTC"}
  }'
```

**Response:**
```json
{
  "result": {
    "symbol": "BTC",
    "price": 50000,
    "timestamp": 1756262706
  },
  "transaction_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

### Health Check

```bash
curl http://localhost:8899/health
```

**Response:**
```json
{
  "status": "healthy",
  "network": "local", 
  "block_height": 0,
  "contracts_count": 1
}
```

## 🛠️ Development Status

**Phase 1 (MVP) - ✅ Completed**
- Core CLI commands (init, build, deploy, test)
- Basic project template system
- Beautiful user interface

**Phase 2 (SDK) - ✅ Completed**
- ✅ TypeScript SDK generation
- ✅ Python SDK generation  
- ✅ Local test network (HTTP API)
- ✅ Enhanced error handling
- ✅ Comprehensive integration tests

**Phase 3 (Ecosystem) - 📋 Planned**
- Real Qubic network integration
- VSCode extension
- Package registry (crates.io, npm)
- Community template library

## 🎬 Demo

![QAnchor Demo](qanchor-demo.gif)

Complete demonstration of the development workflow from initialization to deployment.

**Complete Development Workflow**:
```bash
# Full feature demonstration
qanchor --version                           # Show version
qanchor init my-oracle                      # Create project (5s)
cd my-oracle                               
qanchor build                               # Compile contract (8s)
qanchor deploy --network local              # Deploy contract (5s)
qanchor test                                # Run tests (3s)
qanchor generate --lang ts --output ./ts-sdk  # Generate TypeScript SDK (3s)
qanchor generate --lang py --output ./py-sdk  # Generate Python SDK (3s)
qanchor localnet &                          # Start local network
curl http://localhost:8899/health           # Test API
qanchor clean                               # Cleanup
```

## 🤝 Contributing

QAnchor is an open source project, contributions are welcome!

1. Fork this project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Links

- [QAnchor Website](https://qanchor.dev) (Coming Soon)
- [Qubic Official Documentation](https://qubic.org)
- [Qubic Development Tools](https://github.com/qubic/qubic-dev-kit)
- X: @qanchor_dev

---

**Making Qubic development simple!** 🚀
⭐ **Star us on GitHub if QAnchor helps your Qubic development!**
