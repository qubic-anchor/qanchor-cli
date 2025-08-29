use serde::{Deserialize, Serialize};

/// QIDL 程式的根結構
/// 
/// 這是 QIDL (Qubic Interface Definition Language) 的核心結構，
/// 參考了 Solana Anchor IDL 的設計，並針對 Qubic 進行了適配。
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QidlProgram {
    /// QIDL 規範版本
    pub version: String,
    /// QIDL 規範格式版本
    #[serde(default = "default_spec_version")]
    pub spec: String,
    /// 程式基本資訊
    pub program: ProgramInfo,
    /// 指令定義列表
    pub instructions: Vec<Instruction>,
    /// 帳戶結構定義
    pub accounts: Vec<Account>,
    /// 自定義類型定義
    pub types: Vec<TypeDef>,
    /// 事件定義
    pub events: Vec<Event>,
    /// 錯誤定義
    pub errors: Vec<ErrorDef>,
    /// 常數定義
    #[serde(default)]
    pub constants: Vec<Constant>,
    /// 元資料
    #[serde(default)]
    pub metadata: QidlMetadata,
}

fn default_spec_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProgramInfo {
    /// 程式名稱
    pub name: String,
    /// 程式描述
    pub description: String,
    /// 程式版本
    pub version: String,
    /// 作者列表
    pub authors: Vec<String>,
    /// 程式 ID (Qubic 地址)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
    /// 授權許可
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// 專案 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instruction {
    /// 指令名稱
    pub name: String,
    /// 指令描述
    pub description: String,
    /// 指令參數
    pub args: Vec<Argument>,
    /// 相關帳戶
    pub accounts: Vec<AccountRef>,
    /// 回傳類型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<ReturnType>,
    /// 指令判別器 (8 bytes hash)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<[u8; 8]>,
    /// 文檔範例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Argument {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationRules>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ValidationRules {
    pub min: Option<u64>,
    pub max: Option<u64>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountRef {
    /// 帳戶名稱
    pub name: String,
    /// 帳戶類型
    #[serde(rename = "type")]
    pub account_type: String,
    /// 是否可變
    #[serde(default)]
    pub mutable: bool,
    /// 是否為簽名者
    #[serde(default)]
    pub signer: bool,
    /// 帳戶描述
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// 帳戶約束
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<AccountConstraint>,
    /// 是否可選
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReturnType {
    #[serde(rename = "type")]
    pub return_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub name: String,
    pub description: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeDef {
    pub name: String,
    pub description: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorDef {
    pub code: u32,
    pub name: String,
    pub message: String,
}

/// 類型映射工具
pub struct TypeMapper;

impl TypeMapper {
    /// 將 QIDL 類型映射到 TypeScript 類型
    pub fn to_typescript(qidl_type: &str) -> String {
        match qidl_type {
            "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => "number".to_string(),
            "bool" => "boolean".to_string(),
            "string" => "string".to_string(),
            "PublicKey" => "string".to_string(), // Base58 編碼的字串
            t if t.starts_with("HashMap<") => {
                // 簡化處理：HashMap<String, T> -> Record<string, T>
                if let Some(inner) = t.strip_prefix("HashMap<String, ").and_then(|s| s.strip_suffix('>')) {
                    format!("Record<string, {}>", Self::to_typescript(inner))
                } else {
                    "Record<string, any>".to_string()
                }
            }
            t if t.starts_with("Vec<") => {
                // Vec<T> -> T[]
                if let Some(inner) = t.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
                    format!("{}[]", Self::to_typescript(inner))
                } else {
                    "any[]".to_string()
                }
            }
            // 自定義類型保持原名
            _ => qidl_type.to_string(),
        }
    }
    
    /// 將 QIDL 類型映射到 Python 類型
    pub fn to_python(qidl_type: &str) -> String {
        match qidl_type {
            "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => "int".to_string(),
            "bool" => "bool".to_string(),
            "string" => "str".to_string(),
            "PublicKey" => "str".to_string(), // Base58 編碼的字串
            t if t.starts_with("HashMap<") => {
                // HashMap<String, T> -> Dict[str, T]
                if let Some(inner) = t.strip_prefix("HashMap<String, ").and_then(|s| s.strip_suffix('>')) {
                    format!("Dict[str, {}]", Self::to_python(inner))
                } else {
                    "Dict[str, Any]".to_string()
                }
            }
            t if t.starts_with("Vec<") => {
                // Vec<T> -> List[T]
                if let Some(inner) = t.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
                    format!("List[{}]", Self::to_python(inner))
                } else {
                    "List[Any]".to_string()
                }
            }
            // 自定義類型保持原名
            _ => qidl_type.to_string(),
        }
    }
}

/// 帳戶約束定義
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AccountConstraint {
    /// 初始化約束
    Init {
        /// 付費者帳戶
        payer: String,
        /// 空間大小
        space: usize,
        /// 種子 (對於 PDA)
        #[serde(skip_serializing_if = "Option::is_none")]
        seeds: Option<Vec<String>>,
    },
    /// 所有者約束
    Owner {
        /// 擁有者程式 ID
        program: String,
    },
    /// 種子約束 (PDA)
    Seeds {
        /// 種子值
        seeds: Vec<String>,
        /// bump
        #[serde(skip_serializing_if = "Option::is_none")]
        bump: Option<String>,
    },
    /// 關聯約束
    Associated {
        /// 關聯目標
        with: String,
        /// 關聯類型
        #[serde(skip_serializing_if = "Option::is_none")]
        associated_type: Option<String>,
    },
    /// 自定義約束
    Custom {
        /// 約束表達式
        expression: String,
        /// 錯誤訊息
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
}

/// 常數定義
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Constant {
    /// 常數名稱
    pub name: String,
    /// 常數值
    pub value: ConstantValue,
    /// 常數類型
    #[serde(rename = "type")]
    pub const_type: String,
    /// 常數描述
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// 常數值
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ConstantValue {
    /// 字串值
    String(String),
    /// 數字值
    Number(i64),
    /// 布林值
    Boolean(bool),
    /// 陣列值
    Array(Vec<ConstantValue>),
}

/// QIDL 元資料
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct QidlMetadata {
    /// 編譯器版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_version: Option<String>,
    /// 生成時間
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    /// 源碼雜湊值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_hash: Option<String>,
    /// 建構參數
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_args: Option<Vec<String>>,
    /// 相依性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
}

/// 相依性定義
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dependency {
    /// 相依性名稱
    pub name: String,
    /// 版本
    pub version: String,
    /// 相依性類型
    #[serde(rename = "type", default = "default_dependency_type")]
    pub dep_type: String,
}

fn default_dependency_type() -> String {
    "crate".to_string()
}

/// QIDL 版本兼容性檢查
impl QidlProgram {
    /// 檢查版本兼容性
    pub fn check_compatibility(&self, required_version: &str) -> bool {
        // 簡單的版本比較邏輯
        // 在實際實現中，應該使用 semver 庫
        self.spec.as_str() >= required_version
    }
    
    /// 取得所有指令名稱
    pub fn instruction_names(&self) -> Vec<&str> {
        self.instructions.iter().map(|i| i.name.as_str()).collect()
    }
    
    /// 取得所有帳戶類型名稱
    pub fn account_type_names(&self) -> Vec<&str> {
        self.accounts.iter().map(|a| a.name.as_str()).collect()
    }
    
    /// 取得所有自定義類型名稱
    pub fn custom_type_names(&self) -> Vec<&str> {
        self.types.iter().map(|t| t.name.as_str()).collect()
    }
}

/// QIDL 建構器 - 用於程式化建構 QIDL
pub struct QidlBuilder {
    program: QidlProgram,
}

impl QidlBuilder {
    /// 建立新的 QIDL 建構器
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            program: QidlProgram {
                version: version.to_string(),
                spec: "1.0.0".to_string(),
                program: ProgramInfo {
                    name: name.to_string(),
                    description: String::new(),
                    version: version.to_string(),
                    authors: Vec::new(),
                    program_id: None,
                    license: None,
                    repository: None,
                },
                instructions: Vec::new(),
                accounts: Vec::new(),
                types: Vec::new(),
                events: Vec::new(),
                errors: Vec::new(),
                constants: Vec::new(),
                metadata: QidlMetadata::default(),
            },
        }
    }
    
    /// 設定程式描述
    pub fn description(mut self, description: &str) -> Self {
        self.program.program.description = description.to_string();
        self
    }
    
    /// 添加作者
    pub fn author(mut self, author: &str) -> Self {
        self.program.program.authors.push(author.to_string());
        self
    }
    
    /// 添加指令
    pub fn instruction(mut self, instruction: Instruction) -> Self {
        self.program.instructions.push(instruction);
        self
    }
    
    /// 添加帳戶類型
    pub fn account_type(mut self, account: Account) -> Self {
        self.program.accounts.push(account);
        self
    }
    
    /// 建構 QIDL 程式
    pub fn build(self) -> QidlProgram {
        self.program
    }
}
