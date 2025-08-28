//! Program execution and instruction dispatch

use crate::context::{Context, AccountInfo};
use crate::error::ErrorCode;

/// Trait for QAnchor programs
/// 
/// This trait is automatically implemented by the `#[program]` macro
/// and provides the entry point for instruction execution.
pub trait ProgramEntry {
    /// Execute an instruction with the given instruction data and accounts
    fn execute(
        program_id: &[u8; 32],
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> Result<(), ErrorCode>;
}

/// Program instruction dispatcher
/// 
/// This is used internally by the `#[program]` macro to dispatch
/// instructions to their handlers.
pub struct InstructionDispatcher;

impl InstructionDispatcher {
    /// Dispatch an instruction to its handler
    pub fn dispatch<'info, T>(
        program_id: &'info [u8; 32],
        accounts: &'info [AccountInfo<'info>],
        _instruction_data: &[u8],
        handler: fn(Context<'info, T>) -> Result<(), ErrorCode>,
    ) -> Result<(), ErrorCode>
    where
        T: crate::accounts::Accounts<'info>,
    {
        // Parse instruction data to extract parameters
        let (accounts_data, remaining_accounts) = Self::parse_accounts(accounts)?;
        
        // Validate and deserialize accounts
        let mut account_iter = accounts_data.iter().peekable();
        let validated_accounts = T::try_accounts(
            program_id,
            &mut account_iter,
            remaining_accounts,
        )?;

        // Create context
        let ctx = Context::new(validated_accounts, program_id, remaining_accounts);

        // Execute the instruction handler
        handler(ctx)
    }

    /// Parse accounts into validated accounts and remaining accounts
    fn parse_accounts<'info>(accounts: &'info [AccountInfo<'info>]) -> Result<(&'info [AccountInfo<'info>], &'info [AccountInfo<'info>]), ErrorCode> {
        // For now, return all accounts as validated accounts
        // In a real implementation, this would parse the account metadata
        // to determine which accounts are for validation and which are remaining
        Ok((accounts, &[]))
    }
}

/// Entry point macro for QAnchor programs
/// 
/// This macro generates the main entry point function that the Qubic
/// runtime will call to execute instructions.
#[macro_export]
macro_rules! program_entry {
    ($program_type:ty) => {
        // Placeholder for program entry - in a real implementation,
        // this would generate the appropriate entry point for the Qubic runtime
        pub fn _qanchor_program_entry() {
            // This is a placeholder that will be expanded when we have
            // the actual Qubic runtime integration
        }
    };
}

/// Instruction parameter extraction
pub struct InstructionParams;

impl InstructionParams {
    /// Extract parameters from instruction data
    pub fn extract<T>(instruction_data: &[u8]) -> Result<T, ErrorCode>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(instruction_data)
            .map_err(|_| ErrorCode::InvalidInstructionData)
    }
}

/// System calls interface for Qubic programs
pub struct SystemCalls;

impl SystemCalls {
    /// Log a message (placeholder for Qubic system call)
    pub fn log(message: &str) -> Result<(), ErrorCode> {
        // In a real implementation, this would call the Qubic logging system call
        println!("QAnchor Log: {}", message);
        Ok(())
    }

    /// Get current block height (placeholder for Qubic system call)
    pub fn get_block_height() -> Result<u64, ErrorCode> {
        // Placeholder implementation
        Ok(0)
    }

    /// Get current timestamp (placeholder for Qubic system call)
    pub fn get_timestamp() -> Result<u64, ErrorCode> {
        // Placeholder implementation
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(|_| ErrorCode::SystemError)
    }

    /// Create a new account (placeholder for Qubic system call)
    pub fn create_account(
        payer: &[u8; 32],
        new_account: &[u8; 32],
        space: u64,
        owner: &[u8; 32],
    ) -> Result<(), ErrorCode> {
        // Placeholder implementation
        Self::log(&format!(
            "Creating account: {:?}, payer: {:?}, space: {}, owner: {:?}",
            new_account, payer, space, owner
        ))?;
        Ok(())
    }

    /// Transfer funds between accounts (placeholder for Qubic system call)
    pub fn transfer(
        from: &[u8; 32],
        to: &[u8; 32],
        amount: u64,
    ) -> Result<(), ErrorCode> {
        // Placeholder implementation
        Self::log(&format!(
            "Transfer: {} from {:?} to {:?}",
            amount, from, to
        ))?;
        Ok(())
    }
}
