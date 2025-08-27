use async_trait::async_trait;
use anyhow::Result;
use std::path::Path;
use crate::qidl::QidlProgram;

/// SDK 生成器 trait
#[async_trait]
pub trait SdkGenerator {
    /// 生成 SDK
    async fn generate(&self, qidl: &QidlProgram, output_dir: &Path) -> Result<()>;
    
    /// 生成器支援的語言
    fn language(&self) -> &'static str;
    
    /// 檔案副檔名
    #[allow(dead_code)]
    fn file_extension(&self) -> &'static str;
    
    /// 驗證輸出目錄
    fn validate_output_dir(&self, output_dir: &Path) -> Result<()> {
        if output_dir.exists() && !output_dir.is_dir() {
            anyhow::bail!("Output path exists but is not a directory: {}", output_dir.display());
        }
        
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }
        
        Ok(())
    }
}
