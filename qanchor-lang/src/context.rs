//! Context types for QAnchor instruction handling
//! 
//! The Context type provides access to accounts and program information
//! for instruction execution.

use std::marker::PhantomData;
use crate::error::ErrorCode;

/// Context for instruction execution
/// 
/// Contains validated accounts and provides access to program state.
/// Generic over the accounts type `T` which must implement `Accounts`.
/// 
/// # Example
/// 
/// ```rust,ignore
/// pub fn my_instruction(ctx: Context<MyAccounts>) -> Result<()> {
///     let accounts = ctx.accounts;
///     // Use accounts...
///     Ok(())
/// }
/// ```
pub struct Context<'info, T> {
    /// The validated accounts for this instruction
    pub accounts: T,
    /// Program ID of the executing program
    pub program_id: &'info [u8; 32],
    /// Remaining accounts not specified in the accounts struct
    pub remaining_accounts: &'info [AccountInfo<'info>],
    _phantom: PhantomData<&'info ()>,
}

impl<'info, T> Context<'info, T> {
    /// Create a new Context
    pub fn new(
        accounts: T,
        program_id: &'info [u8; 32],
        remaining_accounts: &'info [AccountInfo<'info>],
    ) -> Self {
        Self {
            accounts,
            program_id,
            remaining_accounts,
            _phantom: PhantomData,
        }
    }
}

/// Account information from the Qubic runtime
/// 
/// This represents an account passed to the program with its metadata.
#[derive(Debug)]
pub struct AccountInfo<'info> {
    /// The account's public key (32 bytes for Qubic)
    pub key: &'info [u8; 32],
    /// Whether this account signed the transaction
    pub is_signer: bool,
    /// Whether this account is writable
    pub is_writable: bool,
    /// The account's data
    pub data: &'info mut [u8],
    /// The account's balance (in Qubic units)
    pub balance: u64,
    /// The program that owns this account
    pub owner: &'info [u8; 32],
}

impl<'info> AccountInfo<'info> {
    /// Create a new AccountInfo
    pub fn new(
        key: &'info [u8; 32],
        is_signer: bool,
        is_writable: bool,
        data: &'info mut [u8],
        balance: u64,
        owner: &'info [u8; 32],
    ) -> Self {
        Self {
            key,
            is_signer,
            is_writable,
            data,
            balance,
            owner,
        }
    }

    /// Check if this account is owned by the given program
    pub fn is_owned_by(&self, program_id: &[u8; 32]) -> bool {
        self.owner == program_id
    }

    /// Deserialize account data into a type
    pub fn try_deserialize<T>(&self) -> Result<T, ErrorCode> 
    where 
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(self.data)
            .map_err(|_| ErrorCode::AccountDidNotDeserialize)
    }

    /// Serialize and write data to the account
    pub fn try_serialize<T>(&mut self, data: &T) -> Result<(), ErrorCode>
    where
        T: serde::Serialize,
    {
        if !self.is_writable {
            return Err(ErrorCode::AccountNotMutable);
        }

        let serialized = serde_json::to_vec(data)
            .map_err(|_| ErrorCode::AccountDidNotSerialize)?;

        if serialized.len() > self.data.len() {
            return Err(ErrorCode::AccountReallocExceeded);
        }

        self.data[..serialized.len()].copy_from_slice(&serialized);
        Ok(())
    }
}
