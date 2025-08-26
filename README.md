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
# Install QAnchor (Development)
git clone https://github.com/qubic-anchor/qanchor-cli.git
cd qanchor-cli
cargo build --release
sudo cp target/release/qanchor /usr/local/bin/

# Create a new project
qanchor init my-oracle
cd my-oracle

# Build and deploy
qanchor build
qanchor deploy --network local
qanchor test
```

## 📁 Project Structure

QAnchor projects follow a standard structure:

```
my-oracle/
├── qanchor.yaml          # Project configuration
├── README.md             # Project documentation
├── .gitignore           # Git ignore file
├── src/
│   ├── lib.rs           # Rust contract implementation
│   └── oracle.qidl      # QIDL interface definition
└── tests/
    └── oracle.test.ts   # TypeScript tests
```

## 🎯 Features

- 🔥 **Zero Learning Curve**: If you know Anchor, you know QAnchor
- ⚡ **30-Second Setup**: From zero to running in 30 seconds
- 🛡️ **Type Safety**: QIDL-driven SDK generation
- 🧪 **Time Travel Testing**: Snapshot and replay capabilities
- 🎨 **IDE Integration**: Full VSCode support with syntax highlighting
- ✅ **Complete CLI Tools** - Anchor-inspired command interface
- ✅ **Project Template System** - Quick Oracle contract scaffolding
- ✅ **Beautiful Output** - Colored text and progress bars

## 📚 Command Reference

### `qanchor init <project-name>`
Create a new QAnchor project with complete project structure and template files.

### `qanchor build`
Compile Rust contracts and generate WASM and QIDL files.

### `qanchor deploy [--network <network>]`
Deploy contracts to specified network (defaults to local).

### `qanchor test [--pattern <pattern>]`
Run project tests with support for test pattern filtering.

### `qanchor --version`
Display QAnchor CLI version information.

## 🛠️ Development Status

**Phase 1 (MVP) - ✅ Completed**
- Core CLI commands (init, build, deploy, test)
- Basic project template system
- Beautiful user interface

**Phase 2 (SDK) - 🚧 In Progress**
- SDK generation functionality
- Improved error handling
- Enhanced test coverage

**Phase 3 (Ecosystem) - 📋 Planned**
- Local testnet integration
- VSCode extension
- Community template library

## 🎬 Demo

![QAnchor Demo](qanchor-demo.gif)

Complete demonstration of the development workflow from initialization to deployment.

**30-Second Demo Flow**:
```bash
# Terminal recording script
qanchor --version                  # (2s)
qanchor init hello-qubic          # (5s) 
cd hello-qubic                    # (1s)
qanchor build                     # (10s)
qanchor deploy --network local    # (10s)
qanchor test                      # (2s)
# Success message                 # (total: 30s)
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
