# 🦀 QAnchor CLI

> The Anchor for Qubic - Modern development framework

QAnchor 是一個模仿 Solana Anchor 的 Qubic 開發框架，提供友善的 CLI 工具和完整的開發體驗。

## 🚀 快速開始

### 安裝

```bash
# 從原始碼編譯安裝 (開發階段)
git clone https://github.com/qubic-anchor/qanchor-cli.git
cd qanchor-cli
cargo build --release
sudo cp target/release/qanchor /usr/local/bin/
```

### 使用方式

```bash
# 建立新專案
qanchor init my-oracle

# 進入專案目錄
cd my-oracle

# 編譯合約
qanchor build

# 部署到本地網路
qanchor deploy --network local

# 執行測試
qanchor test
```

## 📁 專案結構

QAnchor 專案採用標準的結構：

```
my-oracle/
├── qanchor.yaml          # 專案配置
├── README.md             # 專案說明
├── .gitignore           # Git 忽略檔案
├── src/
│   ├── lib.rs           # Rust 合約實作
│   └── oracle.qidl      # QIDL 介面定義
└── tests/
    └── oracle.test.ts   # TypeScript 測試
```

## 🎯 功能特色

- ✅ **完整 CLI 工具** - 模仿 Anchor 的指令介面
- ✅ **專案範本系統** - 快速建立 Oracle 合約
- ✅ **QIDL 支援** - Qubic 介面定義語言
- ✅ **美觀的輸出** - 彩色文字和進度條
- ✅ **TypeScript 測試** - 完整的測試環境
- 🚧 **SDK 生成** - 自動生成 TypeScript/Python SDK
- 🚧 **本地測試網** - 一鍵啟動開發環境
- 🚧 **IDE 整合** - VSCode 擴充套件

## 📚 指令說明

### `qanchor init <project-name>`
建立新的 QAnchor 專案，包含完整的專案結構和範本檔案。

### `qanchor build`
編譯 Rust 合約，生成 WASM 和 QIDL 檔案。

### `qanchor deploy [--network <network>]`
部署合約到指定網路（預設為 local）。

### `qanchor test [--pattern <pattern>]`
執行專案測試，支援測試模式過濾。

### `qanchor --version`
顯示 QAnchor CLI 版本資訊。

## 🛠️ 開發狀態

**Phase 1 (MVP) - ✅ 已完成**
- 核心 CLI 指令 (init, build, deploy, test)
- 基本專案範本系統
- 美觀的使用者介面

**Phase 2 (SDK) - 🚧 開發中**
- SDK 生成功能
- 改善錯誤處理
- 增加測試覆蓋

**Phase 3 (生態) - 📋 規劃中**
- 本地測試網整合
- VSCode 擴充套件
- 社群範本庫

## 🎬 Demo

![QAnchor Demo](../qanchor-demo.gif)

完整展示了從初始化到部署的開發流程。

## 🤝 貢獻

QAnchor 是開源專案，歡迎貢獻！

1. Fork 這個專案
2. 建立功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交變更 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 開啟 Pull Request

## 📄 授權

此專案採用 MIT 授權 - 詳見 [LICENSE](LICENSE) 檔案。

## 🔗 相關連結

- [QAnchor 官網](https://qanchor.dev) (規劃中)
- [Qubic 官方文檔](https://qubic.org)
- [Qubic 開發工具](https://github.com/qubic/qubic-dev-kit)

---

**讓 Qubic 開發變得簡單！** 🚀
