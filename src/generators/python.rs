use async_trait::async_trait;
use anyhow::Result;
use std::path::Path;
use handlebars::Handlebars;
use serde_json::json;

use crate::qidl::{QidlProgram, TypeMapper};
use super::SdkGenerator;

pub struct PythonGenerator {
    handlebars: Handlebars<'static>,
}

impl PythonGenerator {
    pub fn new() -> Result<Self> {
        let mut hb = Handlebars::new();
        
        // 註冊 Python 範本 (稍後實作)
        hb.register_template_string(
            "types", 
            include_str!("templates/python/types.py.hbs")
        )?;
        
        hb.register_template_string(
            "client", 
            include_str!("templates/python/client.py.hbs")
        )?;
        
        hb.register_template_string(
            "requirements", 
            include_str!("templates/python/requirements.txt.hbs")
        )?;
        
        Ok(Self { handlebars: hb })
    }
}

#[async_trait]
impl SdkGenerator for PythonGenerator {
    async fn generate(&self, qidl: &QidlProgram, output_dir: &Path) -> Result<()> {
        self.validate_output_dir(output_dir)?;
        
        // 準備範本數據
        let context = json!({
            "program": qidl.program,
            "instructions": qidl.instructions.iter().map(|inst| {
                json!({
                    "name": inst.name,
                    "description": inst.description,
                    "args": inst.args.iter().map(|arg| {
                        json!({
                            "name": arg.name,
                            "type": TypeMapper::to_python(&arg.arg_type),
                            "description": arg.description
                        })
                    }).collect::<Vec<_>>(),
                    "accounts": inst.accounts,
                    "returns": inst.returns
                })
            }).collect::<Vec<_>>(),
            "types": qidl.types.iter().map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "fields": t.fields.iter().map(|f| {
                        json!({
                            "name": f.name,
                            "type": TypeMapper::to_python(&f.field_type),
                            "description": f.description
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>(),
            "events": qidl.events,
            "errors": qidl.errors
        });
        
        // 生成檔案
        let types_content = self.handlebars.render("types", &context)?;
        std::fs::write(output_dir.join("types.py"), types_content)?;
        
        let client_content = self.handlebars.render("client", &context)?;
        std::fs::write(output_dir.join("client.py"), client_content)?;
        
        let requirements_content = self.handlebars.render("requirements", &context)?;
        std::fs::write(output_dir.join("requirements.txt"), requirements_content)?;
        
        // 建立 __init__.py
        let init_content = r#"""QAnchor Python SDK"""
from .types import *
from .client import *

__version__ = "0.1.0"
"#;
        std::fs::write(output_dir.join("__init__.py"), init_content)?;
        
        Ok(())
    }
    
    fn language(&self) -> &'static str {
        "Python"
    }
    
    fn file_extension(&self) -> &'static str {
        "py"
    }
}
