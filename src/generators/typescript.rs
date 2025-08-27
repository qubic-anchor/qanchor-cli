use async_trait::async_trait;
use anyhow::Result;
use std::path::Path;
use handlebars::Handlebars;
use serde_json::json;

use crate::qidl::{QidlProgram, TypeMapper};
use super::SdkGenerator;

pub struct TypeScriptGenerator {
    handlebars: Handlebars<'static>,
}

impl TypeScriptGenerator {
    pub fn new() -> Result<Self> {
        let mut hb = Handlebars::new();
        
        // 註冊 TypeScript 範本 (稍後實作)
        hb.register_template_string(
            "types", 
            include_str!("templates/typescript/types.hbs")
        )?;
        
        hb.register_template_string(
            "client", 
            include_str!("templates/typescript/client.hbs")
        )?;
        
        hb.register_template_string(
            "package", 
            include_str!("templates/typescript/package.json.hbs")
        )?;
        
        Ok(Self { handlebars: hb })
    }
}

#[async_trait]
impl SdkGenerator for TypeScriptGenerator {
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
                            "type": TypeMapper::to_typescript(&arg.arg_type),
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
                            "type": TypeMapper::to_typescript(&f.field_type),
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
        std::fs::write(output_dir.join("types.ts"), types_content)?;
        
        let client_content = self.handlebars.render("client", &context)?;
        std::fs::write(output_dir.join("client.ts"), client_content)?;
        
        let package_content = self.handlebars.render("package", &context)?;
        std::fs::write(output_dir.join("package.json"), package_content)?;
        
        // 建立 index.ts
        let index_content = r#"export * from './types';
export * from './client';
"#;
        std::fs::write(output_dir.join("index.ts"), index_content)?;
        
        Ok(())
    }
    
    fn language(&self) -> &'static str {
        "TypeScript"
    }
    
    fn file_extension(&self) -> &'static str {
        "ts"
    }
}
