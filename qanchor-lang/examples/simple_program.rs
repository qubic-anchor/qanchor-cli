//! Simple QAnchor program example

use qanchor_lang::prelude::*;

// Define a simple counter program
#[program]
pub mod simple_counter {
    #[allow(unused_imports)]
    use super::*;

    /// Initialize a new counter
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        qanchor_lang::program::SystemCalls::log("Simple counter initialized")?;
        Ok(())
    }

    /// Update counter value
    pub fn update(_ctx: Context<Update>) -> Result<()> {
        qanchor_lang::program::SystemCalls::log("Counter updated")?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    pub user: Signer<'info>,
}

// Main function for example compilation
fn main() {
    println!("QAnchor Simple Program Example");
    println!("This demonstrates basic program structure compilation.");
}
