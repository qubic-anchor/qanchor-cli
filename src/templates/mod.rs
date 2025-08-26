use anyhow::{Result, Context};
use handlebars::Handlebars;
use serde_json::json;
use std::path::Path;

pub struct Template {
    name: String,
    handlebars: Handlebars<'static>,
}

impl Template {
    pub fn new(template_name: &str) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        
        // 註冊內嵌範本
        match template_name {
            "basic-oracle" => {
                handlebars.register_template_string("qanchor.yaml", include_str!("basic_oracle/qanchor.yaml"))?;
                handlebars.register_template_string("lib.rs", include_str!("basic_oracle/lib.rs"))?;
                handlebars.register_template_string("oracle.qidl", include_str!("basic_oracle/oracle.qidl"))?;
                handlebars.register_template_string("README.md", include_str!("basic_oracle/README.md"))?;
                handlebars.register_template_string("test.ts", include_str!("basic_oracle/test.ts"))?;
                handlebars.register_template_string(".gitignore", include_str!("basic_oracle/gitignore"))?;
            }
            _ => anyhow::bail!("Unknown template: {}", template_name),
        }
        
        Ok(Template {
            name: template_name.to_string(),
            handlebars,
        })
    }
    
    pub async fn generate(&self, target_path: &Path, project_name: &str) -> Result<()> {
        let context = json!({
            "project_name": project_name,
            "project_name_snake": project_name.replace("-", "_"),
            "project_name_upper": project_name.to_uppercase(),
            "project_name_pascal": to_pascal_case(project_name),
        });
        
        // 建立目錄結構
        std::fs::create_dir_all(target_path.join("src"))?;
        std::fs::create_dir_all(target_path.join("tests"))?;
        
        // 生成檔案
        self.render_file("qanchor.yaml", &target_path.join("qanchor.yaml"), &context)?;
        self.render_file("lib.rs", &target_path.join("src/lib.rs"), &context)?;
        self.render_file("oracle.qidl", &target_path.join("src/oracle.qidl"), &context)?;
        self.render_file("README.md", &target_path.join("README.md"), &context)?;
        self.render_file("test.ts", &target_path.join("tests/oracle.test.ts"), &context)?;
        self.render_file(".gitignore", &target_path.join(".gitignore"), &context)?;
        
        Ok(())
    }
    
    fn render_file(&self, template_name: &str, output_path: &Path, context: &serde_json::Value) -> Result<()> {
        let content = self.handlebars.render(template_name, context)
            .context("Failed to render template")?;
        
        std::fs::write(output_path, content)
            .context("Failed to write file")?;
        
        Ok(())
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

