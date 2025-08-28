//! Error handling for QAnchor programs

use thiserror::Error;

/// Standard error codes for QAnchor programs
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ErrorCode {
    // Account related errors (1000-1999)
    #[error("Account did not deserialize")]
    AccountDidNotDeserialize = 1000,

    #[error("Account did not serialize")]
    AccountDidNotSerialize = 1001,

    #[error("Account is not mutable")]
    AccountNotMutable = 1002,

    #[error("Account is not a signer")]
    AccountNotSigner = 1003,

    #[error("Account reallocation exceeded maximum size")]
    AccountReallocExceeded = 1004,

    #[error("Invalid program ID")]
    InvalidProgramId = 1005,

    #[error("Account not found")]
    AccountNotFound = 1006,

    #[error("Account already exists")]
    AccountAlreadyExists = 1007,

    #[error("Insufficient account balance")]
    InsufficientBalance = 1008,

    #[error("Account size mismatch")]
    AccountSizeMismatch = 1009,

    #[error("Invalid account owner")]
    InvalidAccountOwner = 1010,

    // Constraint related errors (2000-2999)
    #[error("Constraint violation")]
    ConstraintViolation = 2000,

    #[error("Init constraint failed")]
    ConstraintInit = 2001,

    #[error("Mut constraint failed")]
    ConstraintMut = 2002,

    #[error("Signer constraint failed")]
    ConstraintSigner = 2003,

    #[error("Owner constraint failed")]
    ConstraintOwner = 2004,

    #[error("Custom constraint failed")]
    ConstraintCustom = 2005,

    #[error("Space constraint failed")]
    ConstraintSpace = 2006,

    // Program related errors (3000-3999)
    #[error("Program not found")]
    ProgramNotFound = 3000,

    #[error("Invalid instruction data")]
    InvalidInstructionData = 3001,

    #[error("Instruction not found")]
    InstructionNotFound = 3002,

    #[error("Program execution failed")]
    ProgramExecutionFailed = 3003,

    #[error("Program upgrade failed")]
    ProgramUpgradeFailed = 3004,

    #[error("Program is not upgradeable")]
    ProgramNotUpgradeable = 3005,

    // Validation errors (4000-4999)
    #[error("Invalid signature")]
    InvalidSignature = 4000,

    #[error("Invalid transaction")]
    InvalidTransaction = 4001,

    #[error("Invalid account data")]
    InvalidAccountData = 4002,

    #[error("Insufficient accounts")]
    InsufficientAccounts = 4003,

    #[error("Too many accounts")]
    TooManyAccounts = 4004,

    // Math errors (5000-5999)
    #[error("Arithmetic overflow")]
    ArithmeticOverflow = 5000,

    #[error("Division by zero")]
    DivisionByZero = 5001,

    #[error("Invalid calculation")]
    InvalidCalculation = 5002,

    // System errors (6000-6999)
    #[error("System error")]
    SystemError = 6000,

    #[error("Memory allocation failed")]
    MemoryAllocationFailed = 6001,

    #[error("Resource limit exceeded")]
    ResourceLimitExceeded = 6002,

    #[error("Timeout")]
    Timeout = 6003,

    // Custom errors (starting from 10000)
    #[error("Custom error")]
    Custom = 10000,
}

impl ErrorCode {
    /// Get the error code as a u32
    pub fn code(&self) -> u32 {
        *self as u32
    }

    /// Get a human-readable description of the error
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::AccountDidNotDeserialize => "Failed to deserialize account data",
            ErrorCode::AccountDidNotSerialize => "Failed to serialize account data",
            ErrorCode::AccountNotMutable => "Account is marked as immutable",
            ErrorCode::AccountNotSigner => "Account did not sign the transaction",
            ErrorCode::AccountReallocExceeded => "Account reallocation size exceeded maximum",
            ErrorCode::InvalidProgramId => "Program ID does not match expected value",
            ErrorCode::AccountNotFound => "Required account was not provided",
            ErrorCode::AccountAlreadyExists => "Account already exists and cannot be initialized",
            ErrorCode::InsufficientBalance => "Account has insufficient balance for operation",
            ErrorCode::AccountSizeMismatch => "Account size does not match expected value",
            ErrorCode::InvalidAccountOwner => "Account is not owned by the expected program",
            ErrorCode::ConstraintViolation => "Account constraint validation failed",
            ErrorCode::ConstraintInit => "Init constraint validation failed",
            ErrorCode::ConstraintMut => "Mut constraint validation failed",
            ErrorCode::ConstraintSigner => "Signer constraint validation failed",
            ErrorCode::ConstraintOwner => "Owner constraint validation failed",
            ErrorCode::ConstraintCustom => "Custom constraint validation failed",
            ErrorCode::ConstraintSpace => "Space constraint validation failed",
            ErrorCode::ProgramNotFound => "Program not found or not loaded",
            ErrorCode::InvalidInstructionData => "Instruction data is invalid or malformed",
            ErrorCode::InstructionNotFound => "Instruction handler not found",
            ErrorCode::ProgramExecutionFailed => "Program execution encountered an error",
            ErrorCode::ProgramUpgradeFailed => "Program upgrade operation failed",
            ErrorCode::ProgramNotUpgradeable => "Program is not marked as upgradeable",
            ErrorCode::InvalidSignature => "Transaction signature is invalid",
            ErrorCode::InvalidTransaction => "Transaction format or content is invalid",
            ErrorCode::InvalidAccountData => "Account data format is invalid",
            ErrorCode::InsufficientAccounts => "Not enough accounts provided for instruction",
            ErrorCode::TooManyAccounts => "Too many accounts provided for instruction",
            ErrorCode::ArithmeticOverflow => "Arithmetic operation resulted in overflow",
            ErrorCode::DivisionByZero => "Attempted division by zero",
            ErrorCode::InvalidCalculation => "Mathematical calculation is invalid",
            ErrorCode::SystemError => "System-level error occurred",
            ErrorCode::MemoryAllocationFailed => "Failed to allocate required memory",
            ErrorCode::ResourceLimitExceeded => "System resource limit was exceeded",
            ErrorCode::Timeout => "Operation timed out",
            ErrorCode::Custom => "Custom program-specific error",
        }
    }
}

/// Result type alias for QAnchor programs  
pub type Result<T> = std::result::Result<T, ErrorCode>;

/// Macro for defining custom error codes
/// 
/// # Example
/// 
/// ```rust,ignore
/// qanchor_error! {
///     pub enum MyProgramError {
///         #[msg("Invalid amount")]
///         InvalidAmount = 10001,
///         
///         #[msg("Insufficient funds")]
///         InsufficientFunds = 10002,
///     }
/// }
/// ```
#[macro_export]
macro_rules! qanchor_error {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[msg($msg:expr)])?
                $variant:ident = $code:expr
            ),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u32)]
        pub enum $name {
            $(
                $(#[error($msg)])?
                $variant = $code,
            )*
        }

        impl $name {
            pub fn code(&self) -> u32 {
                *self as u32
            }
        }

        impl From<$name> for $crate::error::ErrorCode {
            fn from(_: $name) -> Self {
                $crate::error::ErrorCode::Custom
            }
        }
    };
}

/// Assert macro for program validation
/// 
/// # Example
/// 
/// ```rust,ignore
/// qanchor_assert!(amount > 0, ErrorCode::InvalidAmount);
/// ```
#[macro_export]
macro_rules! qanchor_assert {
    ($condition:expr, $error:expr) => {
        if !($condition) {
            return Err($error);
        }
    };
    ($condition:expr, $error:expr, $($arg:tt)*) => {
        if !($condition) {
            return Err($error);
        }
    };
}
