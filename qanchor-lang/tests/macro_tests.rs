//! Tests for QAnchor macros

use qanchor_lang::prelude::*;

// Test basic program macro
#[program]
pub mod test_program {
    #[allow(unused_imports)]
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        qanchor_lang::program::SystemCalls::log("Program initialized")?;
        Ok(())
    }

    pub fn update_data(mut ctx: Context<UpdateData>, value: u64) -> Result<()> {
        ctx.accounts.data_account.value = value;
        qanchor_lang::program::SystemCalls::log(&format!("Data updated to {}", value))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    pub user: Signer<'info>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DataAccount {
    pub value: u64,
    pub authority: [u8; 32],
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_macro_compilation() {
        // This test ensures that the macros compile correctly
        // In a real test environment, we would create mock accounts
        // and test the instruction dispatch logic
        assert!(true);
    }

    #[test]
    fn test_error_codes() {
        use qanchor_lang::error::ErrorCode;
        
        assert_eq!(ErrorCode::AccountDidNotDeserialize.code(), 1000);
        assert_eq!(ErrorCode::ConstraintMut.code(), 2002);
        assert_eq!(ErrorCode::SystemError.code(), 6000);
    }

    #[test]
    fn test_system_calls() {
        use qanchor_lang::program::SystemCalls;
        
        // Test logging
        assert!(SystemCalls::log("Test message").is_ok());
        
        // Test timestamp
        assert!(SystemCalls::get_timestamp().is_ok());
        
        // Test block height
        assert!(SystemCalls::get_block_height().is_ok());
    }
}
