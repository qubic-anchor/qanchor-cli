use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum QAnchorError {
    #[error("Project directory '{0}' already exists")]
    DirectoryExists(String),
    
    #[error("Template '{0}' not found")]
    TemplateNotFound(String),
    
    #[error("Invalid project name '{0}'. Project names must be valid identifiers.")]
    InvalidProjectName(String),
    
    #[error("Failed to create directory: {0}")]
    DirectoryCreation(String),
    
    #[error("Failed to write file '{0}': {1}")]
    FileWrite(String, String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Build error: {0}")]
    Build(String),
    
    #[error("Deployment error: {0}")]
    Deploy(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, QAnchorError>;

