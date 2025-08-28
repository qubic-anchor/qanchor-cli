//! Basic QAnchor program example

use qanchor_lang::prelude::*;
use qanchor_lang::error::Result;

// Define a simple counter program
#[program]
pub mod counter {
    #[allow(unused_imports)]
    use super::*;

    /// Initialize a new counter
    pub fn initialize(mut ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = *ctx.accounts.user.key();
        
        qanchor_lang::program::SystemCalls::log("Counter initialized")?;
        Ok(())
    }

    /// Increment the counter
    pub fn increment(mut ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Check arithmetic overflow
        qanchor_assert!(
            counter.count < u64::MAX,
            ErrorCode::ArithmeticOverflow
        );
        
        counter.count += 1;
        
        qanchor_lang::program::SystemCalls::log(&format!(
            "Counter incremented to {}", 
            counter.count
        ))?;
        
        Ok(())
    }

    /// Decrement the counter
    pub fn decrement(mut ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Check underflow
        qanchor_assert!(
            counter.count > 0,
            ErrorCode::ArithmeticOverflow
        );
        
        counter.count -= 1;
        
        qanchor_lang::program::SystemCalls::log(&format!(
            "Counter decremented to {}", 
            counter.count
        ))?;
        
        Ok(())
    }

    /// Set counter to a specific value (only authority can do this)
    pub fn set_count(mut ctx: Context<SetCount>, new_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Check authority
        qanchor_assert!(
            counter.authority == *ctx.accounts.authority.key(),
            ErrorCode::InvalidAccountOwner
        );
        
        counter.count = new_count;
        
        qanchor_lang::program::SystemCalls::log(&format!(
            "Counter set to {}", 
            counter.count
        ))?;
        
        Ok(())
    }
}

/// Initialize counter account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)] // discriminator + authority + count
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Increment counter
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

/// Decrement counter  
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

/// Set counter value
#[derive(Accounts)]
pub struct SetCount<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

/// Counter account data
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Counter {
    /// Current count value
    pub count: u64,
    /// Authority that can modify the counter
    pub authority: [u8; 32],
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            count: 0,
            authority: [0u8; 32],
        }
    }
}

// Custom error codes for this program
qanchor_error! {
    pub enum CounterError {
        #[msg("Counter overflow")]
        Overflow = 10001,
        
        #[msg("Counter underflow")]
        Underflow = 10002,
        
        #[msg("Unauthorized access")]
        Unauthorized = 10003,
    }
}

// Main function for example compilation
fn main() {
    println!("QAnchor Counter Program Example");
    println!("This is a demonstration of the QAnchor language framework.");
}
