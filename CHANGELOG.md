# Changelog

All notable changes to QAnchor CLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-08-28

### 🎉 Major Release: Phase 3 Production Features

This is a significant milestone release that brings QAnchor from a development framework to a production-ready tool suite for Qubic blockchain development.

### ✨ Added

#### 🌐 Real Network Integration
- **`qanchor deploy`** now supports real Qubic networks (mainnet/testnet/staging)
- Pre-deployment validation workflow with comprehensive error checking
- RPC health check → build artifact validation → transaction construction/broadcast
- Support for multiple network environments with automatic endpoint management

#### 💳 Wallet Management System
- **`qanchor wallet create`** - Create development wallets with secure key generation
- **`qanchor wallet import`** - Import wallets from seed phrases or private keys
- **`qanchor wallet list`** - List all available wallets
- **`qanchor wallet balance`** - Check wallet balances across networks
- **`qanchor wallet send`** - Send QUBIC tokens for testing and development
- Support for 55-character seed phrases and 64-character hex private keys
- Secure wallet file storage with proper file permissions

#### 📊 Network Diagnostics & Monitoring
- **`qanchor network status`** - Real-time network health checking
- **`qanchor network ping`** - Network performance analysis with statistics
- Support for all Qubic networks (mainnet/testnet/staging)
- Comprehensive network statistics (tick, epoch, skipped ticks, response times)
- Connection quality assessment (EXCELLENT/UNSTABLE/POOR)

#### 📋 Contract Logs Management
- **`qanchor logs`** - View and analyze contract execution logs
- **Real-time log streaming** with `--follow` flag
- **Historical log queries** with `--tail` and `--since` options
- **Advanced filtering** with `--filter` keyword and `--contract` ID
- Support for multiple time formats (1h, 30m, tick numbers)
- Rich log formatting with colors, timestamps, and structured data display

#### ⬆️ Contract Upgrade System
- **`qanchor upgrade`** - Upgrade existing contracts on any network
- Version compatibility checking and upgrade validation
- Integrated with network RPC for seamless contract management

#### 🧪 Enhanced Testing
- **`qanchor test --network`** - Run tests against real networks
- Network-aware test execution with automatic configuration
- Support for both local and remote testing environments

### 🔧 Infrastructure Improvements

#### 🌐 qubic-rpc Crate
- Complete Rust client library for Qubic blockchain interaction
- Support for all major RPC endpoints and operations
- Robust error handling with retry mechanisms and exponential backoff
- Network health monitoring and automatic failover
- Type-safe transaction construction and broadcasting

#### 🛡️ Error Handling & User Experience
- Production-grade error messages with actionable guidance
- Detailed troubleshooting information for common issues
- Progress indicators and status updates for long-running operations
- Colored output with intuitive icons and formatting

#### 📝 qanchor-lang Enhancements
- Improved proc-macro system with better error reporting
- Enhanced Context and Account type system
- Comprehensive error code definitions
- Better documentation and rustdoc integration

### 🚀 Performance & Reliability
- Optimized network communication with connection pooling
- Intelligent caching for improved CLI responsiveness
- Parallel compilation support for faster builds
- Comprehensive integration testing suite

### 📚 Documentation Updates
- Updated README with Phase 3 features and examples
- New command reference for all CLI commands
- Enhanced quick start guide with real network examples
- Complete workflow demonstrations

### 🔄 Breaking Changes
- Version bump to 0.3.0 reflects major feature additions
- CLI commands now default to more secure network operations
- Enhanced validation may require additional build artifacts

### 📦 Dependencies
- Added `qubic-rpc` crate for blockchain interaction
- Added `hex` crate for hexadecimal encoding/decoding
- Enhanced `chrono` integration for time handling
- Updated various dependencies for security and performance

---

## [0.2.0] - 2025-01-25

### Added
- TypeScript SDK generation
- Python SDK generation
- Local test network (HTTP API)
- Enhanced error handling
- Comprehensive integration tests

### Changed
- Improved project template system
- Enhanced CLI user interface
- Better build system integration

---

## [0.1.0] - 2025-01-15

### Added
- Initial release of QAnchor CLI
- Core CLI commands (init, build, deploy, test)
- Basic project template system
- QIDL parser and code generation
- Beautiful user interface with colored output
- Local development workflow support

### Features
- `qanchor init` - Project initialization
- `qanchor build` - Contract compilation
- `qanchor deploy` - Local deployment
- `qanchor test` - Test execution
- `qanchor generate` - SDK generation
- `qanchor localnet` - Local test network
- `qanchor clean` - Cleanup utilities

---

## Development Roadmap

### Phase 3 Remaining (v0.4.0 - Planned)
- QIDL standardization and specification
- VSCode extension with syntax highlighting
- Package registry integration (crates.io, npm)
- Community template library
- Advanced developer tooling

### Future Releases
- Enterprise security features
- Advanced monitoring and analytics
- Multi-chain support expansion
- Performance optimization tools