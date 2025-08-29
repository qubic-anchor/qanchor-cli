//! # {{project_name_pascal}} AMM
//! 
//! A Decentralized Finance (DeFi) Automated Market Maker (AMM) built with QAnchor.
//! This contract provides liquidity pools for token pairs and automated price discovery.

use qanchor_lang::prelude::*;

#[program]
pub mod {{project_name_snake}}_amm {
    use super::*;

    /// Initialize a new liquidity pool for two tokens
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        token_a: [u8; 32],
        token_b: [u8; 32],
        fee_rate: u16,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        // Validate fee rate (1-10000 basis points = 0.01%-100%)
        require!(fee_rate >= 1 && fee_rate <= 10000, ErrorCode::InvalidFeeRate);
        
        // Ensure tokens are different
        require!(token_a != token_b, ErrorCode::InvalidTokenPair);
        
        // Initialize pool state
        pool.token_a = token_a;
        pool.token_b = token_b;
        pool.reserve_a = 0;
        pool.reserve_b = 0;
        pool.total_lp_supply = 0;
        pool.fee_rate = fee_rate;
        pool.authority = ctx.accounts.authority.key();
        pool.token_a_vault = ctx.accounts.token_a_vault.key();
        pool.token_b_vault = ctx.accounts.token_b_vault.key();
        pool.created_at = ctx.accounts.clock.unix_timestamp as u64;
        
        emit!(PoolInitialized {
            pool: pool.key(),
            token_a,
            token_b,
            fee_rate,
            timestamp: pool.created_at,
        });
        
        Ok(())
    }
    
    /// Add liquidity to an existing pool
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a_desired: u64,
        amount_b_desired: u64,
        amount_a_min: u64,
        amount_b_min: u64,
    ) -> Result<LiquidityResult> {
        let pool = &mut ctx.accounts.pool;
        
        // Validate input amounts
        require!(amount_a_desired > 0 && amount_b_desired > 0, ErrorCode::InsufficientInputAmount);
        
        let (amount_a, amount_b, lp_tokens_minted) = if pool.total_lp_supply == 0 {
            // First liquidity addition - use desired amounts
            let liquidity = (amount_a_desired * amount_b_desired).sqrt();
            require!(liquidity > 1000, ErrorCode::InsufficientLiquidity); // Minimum liquidity
            
            (amount_a_desired, amount_b_desired, liquidity - 1000) // Lock minimum liquidity
        } else {
            // Subsequent additions - maintain price ratio
            let amount_b_optimal = amount_a_desired * pool.reserve_b / pool.reserve_a;
            
            if amount_b_optimal <= amount_b_desired {
                require!(amount_b_optimal >= amount_b_min, ErrorCode::SlippageToleranceExceeded);
                let liquidity = amount_a_desired * pool.total_lp_supply / pool.reserve_a;
                (amount_a_desired, amount_b_optimal, liquidity)
            } else {
                let amount_a_optimal = amount_b_desired * pool.reserve_a / pool.reserve_b;
                require!(amount_a_optimal >= amount_a_min, ErrorCode::SlippageToleranceExceeded);
                let liquidity = amount_b_desired * pool.total_lp_supply / pool.reserve_b;
                (amount_a_optimal, amount_b_desired, liquidity)
            }
        };
        
        // Update pool reserves
        pool.reserve_a += amount_a;
        pool.reserve_b += amount_b;
        pool.total_lp_supply += lp_tokens_minted;
        
        // Calculate user's share of pool
        let share_of_pool = if pool.total_lp_supply > 0 {
            (lp_tokens_minted * 10000 / pool.total_lp_supply) as u32
        } else {
            0
        };
        
        emit!(LiquidityAdded {
            pool: pool.key(),
            user: ctx.accounts.user.key(),
            amount_a,
            amount_b,
            lp_tokens_minted,
            timestamp: ctx.accounts.clock.unix_timestamp as u64,
        });
        
        Ok(LiquidityResult {
            amount_a,
            amount_b,
            lp_tokens_minted,
            share_of_pool,
        })
    }
    
    /// Remove liquidity from a pool
    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_token_amount: u64,
        amount_a_min: u64,
        amount_b_min: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        require!(lp_token_amount > 0, ErrorCode::InsufficientInputAmount);
        require!(pool.total_lp_supply > 0, ErrorCode::InsufficientLiquidity);
        
        // Calculate amounts to return
        let amount_a = lp_token_amount * pool.reserve_a / pool.total_lp_supply;
        let amount_b = lp_token_amount * pool.reserve_b / pool.total_lp_supply;
        
        require!(amount_a >= amount_a_min, ErrorCode::SlippageToleranceExceeded);
        require!(amount_b >= amount_b_min, ErrorCode::SlippageToleranceExceeded);
        
        // Update pool state
        pool.reserve_a -= amount_a;
        pool.reserve_b -= amount_b;
        pool.total_lp_supply -= lp_token_amount;
        
        emit!(LiquidityRemoved {
            pool: pool.key(),
            user: ctx.accounts.user.key(),
            amount_a,
            amount_b,
            lp_tokens_burned: lp_token_amount,
            timestamp: ctx.accounts.clock.unix_timestamp as u64,
        });
        
        Ok(())
    }
    
    /// Swap tokens using the AMM
    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u64,
        amount_out_min: u64,
        token_a_to_b: bool,
    ) -> Result<SwapResult> {
        let pool = &mut ctx.accounts.pool;
        
        require!(amount_in > 0, ErrorCode::InsufficientInputAmount);
        require!(pool.reserve_a > 0 && pool.reserve_b > 0, ErrorCode::InsufficientLiquidity);
        
        let (reserve_in, reserve_out) = if token_a_to_b {
            (pool.reserve_a, pool.reserve_b)
        } else {
            (pool.reserve_b, pool.reserve_a)
        };
        
        // Calculate fee amount
        let fee_amount = amount_in * pool.fee_rate as u64 / 10000;
        let amount_in_with_fee = amount_in - fee_amount;
        
        // AMM formula: x * y = k (constant product)
        let amount_out = reserve_out * amount_in_with_fee / (reserve_in + amount_in_with_fee);
        
        require!(amount_out >= amount_out_min, ErrorCode::SlippageToleranceExceeded);
        require!(amount_out < reserve_out, ErrorCode::InsufficientLiquidity);
        
        // Calculate price impact
        let price_impact = (amount_out * 10000 / reserve_out) as u32;
        
        // Update reserves
        if token_a_to_b {
            pool.reserve_a += amount_in;
            pool.reserve_b -= amount_out;
        } else {
            pool.reserve_b += amount_in;
            pool.reserve_a -= amount_out;
        }
        
        let (token_in, token_out) = if token_a_to_b {
            (pool.token_a, pool.token_b)
        } else {
            (pool.token_b, pool.token_a)
        };
        
        emit!(TokenSwapped {
            pool: pool.key(),
            user: ctx.accounts.user.key(),
            token_in,
            token_out,
            amount_in,
            amount_out,
            fee_amount,
            timestamp: ctx.accounts.clock.unix_timestamp as u64,
        });
        
        Ok(SwapResult {
            amount_in,
            amount_out,
            fee_amount,
            price_impact,
            new_reserve_a: pool.reserve_a,
            new_reserve_b: pool.reserve_b,
        })
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 2 + 32 + 32 + 32 + 32 + 8 + 1,
        seeds = [b"liquidity_pool", token_a_vault.key().as_ref(), token_b_vault.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, LiquidityPool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Token A vault account
    pub token_a_vault: AccountInfo<'info>,
    
    /// Token B vault account
    pub token_b_vault: AccountInfo<'info>,
    
    /// Clock sysvar for timestamps
    pub clock: Sysvar<'info, Clock>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, LiquidityPool>,
    
    #[account(mut)]
    pub user_token_a_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_token_b_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub pool_token_a_vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub pool_token_b_vault: AccountInfo<'info>,
    
    pub user: Signer<'info>,
    
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, LiquidityPool>,
    
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_token_a_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_token_b_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub pool_token_a_vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub pool_token_b_vault: AccountInfo<'info>,
    
    pub user: Signer<'info>,
    
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, LiquidityPool>,
    
    #[account(mut)]
    pub user_source_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_destination_account: AccountInfo<'info>,
    
    #[account(mut)]
    pub pool_source_vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub pool_destination_vault: AccountInfo<'info>,
    
    pub user: Signer<'info>,
    
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct LiquidityPool {
    /// First token in the pair
    pub token_a: [u8; 32],
    /// Second token in the pair
    pub token_b: [u8; 32],
    /// Reserve amount of token A
    pub reserve_a: u64,
    /// Reserve amount of token B
    pub reserve_b: u64,
    /// Total supply of LP tokens
    pub total_lp_supply: u64,
    /// Fee rate in basis points
    pub fee_rate: u16,
    /// Pool authority
    pub authority: [u8; 32],
    /// Token A vault account
    pub token_a_vault: [u8; 32],
    /// Token B vault account
    pub token_b_vault: [u8; 32],
    /// LP token mint account
    pub lp_token_mint: [u8; 32],
    /// Pool creation timestamp
    pub created_at: u64,
    /// PDA bump seed
    pub bump: u8,
}

/// Result returned when adding liquidity
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LiquidityResult {
    /// Actual amount of token A added
    pub amount_a: u64,
    /// Actual amount of token B added
    pub amount_b: u64,
    /// Amount of LP tokens minted
    pub lp_tokens_minted: u64,
    /// User's share of pool in basis points
    pub share_of_pool: u32,
}

/// Result returned when performing a swap
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapResult {
    /// Actual input amount
    pub amount_in: u64,
    /// Actual output amount
    pub amount_out: u64,
    /// Fee amount charged
    pub fee_amount: u64,
    /// Price impact in basis points
    pub price_impact: u32,
    /// Pool's new reserve A after swap
    pub new_reserve_a: u64,
    /// Pool's new reserve B after swap
    pub new_reserve_b: u64,
}

#[event]
pub struct PoolInitialized {
    pub pool: [u8; 32],
    pub token_a: [u8; 32],
    pub token_b: [u8; 32],
    pub fee_rate: u16,
    pub timestamp: u64,
}

#[event]
pub struct LiquidityAdded {
    pub pool: [u8; 32],
    pub user: [u8; 32],
    pub amount_a: u64,
    pub amount_b: u64,
    pub lp_tokens_minted: u64,
    pub timestamp: u64,
}

#[event]
pub struct LiquidityRemoved {
    pub pool: [u8; 32],
    pub user: [u8; 32],
    pub amount_a: u64,
    pub amount_b: u64,
    pub lp_tokens_burned: u64,
    pub timestamp: u64,
}

#[event]
pub struct TokenSwapped {
    pub pool: [u8; 32],
    pub user: [u8; 32],
    pub token_in: [u8; 32],
    pub token_out: [u8; 32],
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_amount: u64,
    pub timestamp: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient liquidity for this operation")]
    InsufficientLiquidity,
    #[msg("Slippage tolerance exceeded")]
    SlippageToleranceExceeded,
    #[msg("Invalid token pair: tokens must be different")]
    InvalidTokenPair,
    #[msg("Pool for this token pair already exists")]
    PoolAlreadyExists,
    #[msg("Insufficient input amount")]
    InsufficientInputAmount,
    #[msg("Insufficient output amount")]
    InsufficientOutputAmount,
    #[msg("Fee rate must be between 0.01% and 100%")]
    InvalidFeeRate,
    #[msg("Mathematical operation resulted in overflow")]
    MathOverflow,
    #[msg("Pool has not been initialized")]
    PoolNotInitialized,
}

// Constants
pub const MINIMUM_LIQUIDITY: u64 = 1000;
pub const MAX_FEE_RATE: u16 = 10000;
pub const MIN_FEE_RATE: u16 = 1;
