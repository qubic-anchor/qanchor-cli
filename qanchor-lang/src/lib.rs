//! # QAnchor Language Framework
//! 
//! QAnchor Lang is a framework for writing Qubic smart contracts in Rust.
//! It provides proc macros and utilities similar to Solana's Anchor framework.
//! 
//! ## Features
//! 
//! - `#[program]` macro for defining smart contract programs
//! - `#[derive(Accounts)]` macro for account validation
//! - Context types for instruction handling
//! - Error handling with custom error codes
//! - Integration with Qubic blockchain primitives
//! 
//! ## Example
//! 
//! ```rust,ignore
//! use qanchor_lang::prelude::*;
//! 
//! #[program]
//! pub mod my_program {
//!     use super::*;
//! 
//!     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//!         // Your program logic here
//!         Ok(())
//!     }
//! }
//! 
//! #[derive(Accounts)]
//! pub struct Initialize<'info> {
//!     #[account(init, payer = user, space = 8 + 32)]
//!     pub my_account: Account<'info, MyAccount>,
//!     #[account(mut)]
//!     pub user: Signer<'info>,
//!     pub system_program: Program<'info, System>,
//! }
//! ```

// Re-export commonly used types
pub use qanchor_lang_derive::*;

// Core modules
pub mod context;
pub mod accounts;
pub mod error;
pub mod program;

// Prelude module for easy imports
pub mod prelude {
    pub use crate::context::*;
    pub use crate::accounts::*;
    pub use crate::error::*;
    pub use crate::program::*;
    pub use qanchor_lang_derive::{program, Accounts};
    
    // Re-export macros
    pub use crate::{qanchor_error, qanchor_assert};
}

// Re-export for convenience - use different name to avoid conflicts
pub type QAnchorResult<T> = std::result::Result<T, error::ErrorCode>;
