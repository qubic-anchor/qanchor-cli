use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QidlProgram {
    pub version: String,
    pub program: ProgramInfo,
    pub instructions: Vec<Instruction>,
    pub accounts: Vec<Account>,
    pub types: Vec<TypeDef>,
    pub events: Vec<Event>,
    pub errors: Vec<ErrorDef>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProgramInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instruction {
    pub name: String,
    pub description: String,
    pub args: Vec<Argument>,
    pub accounts: Vec<AccountRef>,
    pub returns: Option<ReturnType>,
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
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(default)]
    pub mutable: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
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
