//! Account types and traits for QAnchor programs

use std::marker::PhantomData;
use crate::context::AccountInfo;
use crate::error::ErrorCode;

/// Trait for account validation
/// 
/// Types implementing this trait can be used in `Context<T>` for instruction validation.
pub trait Accounts<'info>: Sized {
    /// Try to deserialize and validate accounts from the given account info list
    fn try_accounts(
        program_id: &'info [u8; 32],
        accounts: &mut std::iter::Peekable<std::slice::Iter<'info, AccountInfo<'info>>>,
        remaining_accounts: &'info [AccountInfo<'info>],
    ) -> Result<Self, ErrorCode>;
}

/// Account wrapper for validated account data
/// 
/// This type ensures that the account data has been properly validated
/// and deserialized according to the account constraints.
/// 
/// # Example
/// 
/// ```rust,ignore
/// #[derive(Accounts)]
/// pub struct MyAccounts<'info> {
///     #[account(mut)]
///     pub my_account: Account<'info, MyData>,
/// }
/// ```
pub struct Account<'info, T> {
    /// The underlying account info
    pub account_info: &'info AccountInfo<'info>,
    /// The deserialized account data
    pub data: T,
}

impl<'info, T> Account<'info, T>
where
    T: serde::de::DeserializeOwned,
{
    /// Load and validate an account
    pub fn try_from(account_info: &'info AccountInfo<'info>) -> Result<Self, ErrorCode> {
        let data = account_info.try_deserialize::<T>()?;
        Ok(Self { account_info, data })
    }

    /// Get the account's public key
    pub fn key(&self) -> &'info [u8; 32] {
        self.account_info.key
    }

    /// Reload the account data from the account info
    pub fn reload(&mut self) -> Result<(), ErrorCode> {
        self.data = self.account_info.try_deserialize::<T>()?;
        Ok(())
    }
}

impl<'info, T> std::ops::Deref for Account<'info, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'info, T> std::ops::DerefMut for Account<'info, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Signer account type
/// 
/// Ensures that the account has signed the transaction.
/// 
/// # Example
/// 
/// ```rust,ignore
/// #[derive(Accounts)]
/// pub struct MyAccounts<'info> {
///     #[account(mut)]
///     pub user: Signer<'info>,
/// }
/// ```
pub struct Signer<'info> {
    /// The underlying account info
    pub account_info: &'info AccountInfo<'info>,
}

impl<'info> Signer<'info> {
    /// Create a new Signer, validating that the account signed the transaction
    pub fn try_from(account_info: &'info AccountInfo<'info>) -> Result<Self, ErrorCode> {
        if !account_info.is_signer {
            return Err(ErrorCode::AccountNotSigner);
        }
        Ok(Self { account_info })
    }

    /// Get the signer's public key
    pub fn key(&self) -> &'info [u8; 32] {
        self.account_info.key
    }
}

/// Program account type
/// 
/// Represents a reference to another program.
/// 
/// # Example
/// 
/// ```rust,ignore
/// #[derive(Accounts)]
/// pub struct MyAccounts<'info> {
///     pub system_program: Program<'info, System>,
/// }
/// ```
pub struct Program<'info, T> {
    /// The underlying account info
    pub account_info: &'info AccountInfo<'info>,
    _phantom: PhantomData<T>,
}

impl<'info, T> Program<'info, T>
where
    T: ProgramInterface,
{
    /// Create a new Program reference, validating the program ID
    pub fn try_from(account_info: &'info AccountInfo<'info>) -> Result<Self, ErrorCode> {
        if account_info.key != &T::program_id() {
            return Err(ErrorCode::InvalidProgramId);
        }
        Ok(Self {
            account_info,
            _phantom: PhantomData,
        })
    }

    /// Get the program's ID
    pub fn key(&self) -> &'info [u8; 32] {
        self.account_info.key
    }
}

/// Trait for program interfaces
/// 
/// Types implementing this trait can be used with the Program<'info, T> type.
pub trait ProgramInterface {
    /// The program's ID
    fn program_id() -> [u8; 32];
}

/// System program interface
/// 
/// Represents the Qubic system program for account creation and transfers.
pub struct System;

impl ProgramInterface for System {
    fn program_id() -> [u8; 32] {
        [0u8; 32] // System program ID (placeholder)
    }
}

/// Account constraints for validation
/// 
/// This enum represents different constraints that can be applied to accounts
/// during validation.
#[derive(Debug, Clone)]
pub enum AccountConstraint {
    /// Account must be initialized (created) in this instruction
    Init {
        payer: String,
        space: usize,
    },
    /// Account must be mutable
    Mut,
    /// Account must be a signer
    Signer,
    /// Account must be owned by the specified program
    Owner(String),
    /// Custom constraint with validation logic
    Custom(String),
}

/// Account constraint validation
pub trait ConstraintValidator {
    /// Validate the constraint against the account
    fn validate(&self, account_info: &AccountInfo) -> Result<(), ErrorCode>;
}
