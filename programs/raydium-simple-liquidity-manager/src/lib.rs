use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("3QMb6zycgRf62uMCok5Tw4iRnzuAoLpPWLYszefm2MaB");

#[program]
pub mod raydium_simple_liquidity_manager {
    use super::*;

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount: u64,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_token.to_account_info(),
                    to: ctx.accounts.pool_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        amount: u64,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.pool_token.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.pool.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the pool account that we're going to initialize
    #[account(mut)]
    pub pool: AccountInfo<'info>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the pool account that we're using for liquidity removal
    #[account(mut)]
    pub pool: AccountInfo<'info>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}