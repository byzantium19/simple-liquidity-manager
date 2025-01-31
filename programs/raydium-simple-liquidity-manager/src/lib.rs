use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token, TokenAccount, Mint},
};
use raydium_amm_v3::{self, states::*};

declare_id!("3QMb6zycgRf62uMCok5Tw4iRnzuAoLpPWLYszefm2MaB");

#[program]
pub mod raydium_simple_liquidity_manager {
    use super::*;

    pub fn open_position(
        ctx: Context<OpenPosition>,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.raydium_program.to_account_info();
        let cpi_accounts = raydium_amm_v3::accounts::OpenPositionV2 {
            payer: ctx.accounts.user.to_account_info(),
            position_nft_owner: ctx.accounts.user.to_account_info(),
            position_nft_mint: ctx.accounts.position_nft_mint.to_account_info(),
            position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
            metadata_account: ctx.accounts.metadata_account.to_account_info(),
            pool_state: ctx.accounts.pool_state.to_account_info(),
            protocol_position: ctx.accounts.protocol_position.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info(),
            personal_position: ctx.accounts.personal_position.to_account_info(),
            token_account_0: ctx.accounts.token_account_0.to_account_info(),
            token_account_1: ctx.accounts.token_account_1.to_account_info(),
            token_vault_0: ctx.accounts.token_vault_0.to_account_info(),
            token_vault_1: ctx.accounts.token_vault_1.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            metadata_program: ctx.accounts.metadata_program.to_account_info(),
            token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
            vault_0_mint: ctx.accounts.vault_0_mint.to_account_info(),
            vault_1_mint: ctx.accounts.vault_1_mint.to_account_info(),
        };

        raydium_amm_v3::cpi::open_position_v2(
            CpiContext::new(cpi_program, cpi_accounts),
            tick_lower_index,
            tick_upper_index,
        )
    }

    pub fn increase_liquidity(
        ctx: Context<IncreaseLiquidity>,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.raydium_program.to_account_info();
        let cpi_accounts = raydium_amm_v3::accounts::IncreaseLiquidityV2 {
            nft_owner: ctx.accounts.user.to_account_info(),
            nft_account: ctx.accounts.nft_account.to_account_info(),
            pool_state: ctx.accounts.pool_state.to_account_info(),
            protocol_position: ctx.accounts.protocol_position.to_account_info(),
            personal_position: ctx.accounts.personal_position.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info(),
            token_account_0: ctx.accounts.token_account_0.to_account_info(),
            token_account_1: ctx.accounts.token_account_1.to_account_info(),
            token_vault_0: ctx.accounts.token_vault_0.to_account_info(),
            token_vault_1: ctx.accounts.token_vault_1.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
            vault_0_mint: ctx.accounts.vault_0_mint.to_account_info(),
            vault_1_mint: ctx.accounts.vault_1_mint.to_account_info(),
        };

        raydium_amm_v3::cpi::increase_liquidity_v2(
            CpiContext::new(cpi_program, cpi_accounts),
            liquidity,
            amount_0_max,
            amount_1_max,
        )
    }

    pub fn decrease_liquidity(
        ctx: Context<DecreaseLiquidity>,
        liquidity: u128,
        amount_0_min: u64,
        amount_1_min: u64,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.raydium_program.to_account_info();
        let cpi_accounts = raydium_amm_v3::accounts::DecreaseLiquidityV2 {
            nft_owner: ctx.accounts.user.to_account_info(),
            nft_account: ctx.accounts.nft_account.to_account_info(),
            personal_position: ctx.accounts.personal_position.to_account_info(),
            pool_state: ctx.accounts.pool_state.to_account_info(),
            protocol_position: ctx.accounts.protocol_position.to_account_info(),
            token_vault_0: ctx.accounts.token_vault_0.to_account_info(),
            token_vault_1: ctx.accounts.token_vault_1.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info(),
            recipient_token_account_0: ctx.accounts.recipient_token_account_0.to_account_info(),
            recipient_token_account_1: ctx.accounts.recipient_token_account_1.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
            memo_program: ctx.accounts.memo_program.to_account_info(),
            vault_0_mint: ctx.accounts.vault_0_mint.to_account_info(),
            vault_1_mint: ctx.accounts.vault_1_mint.to_account_info(),
        };

        raydium_amm_v3::cpi::decrease_liquidity_v2(
            CpiContext::new(cpi_program, cpi_accounts),
            liquidity,
            amount_0_min,
            amount_1_min,
        )
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        let cpi_program = ctx.accounts.raydium_program.to_account_info();
        let cpi_accounts = raydium_amm_v3::accounts::ClosePosition {
            nft_owner: ctx.accounts.user.to_account_info(),
            position_nft_mint: ctx.accounts.position_nft_mint.to_account_info(),
            position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
            personal_position: ctx.accounts.personal_position.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        raydium_amm_v3::cpi::close_position(CpiContext::new(cpi_program, cpi_accounts))
    }
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub position_nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub position_nft_account: Account<'info, TokenAccount>,
    /// CHECK: Handled by Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    pub protocol_position: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_lower: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_upper: AccountInfo<'info>,
    #[account(mut)]
    pub personal_position: AccountInfo<'info>,
    #[account(mut)]
    pub token_account_0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_account_1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault_0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault_1: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Metaplex program
    pub metadata_program: UncheckedAccount<'info>,
    /// CHECK: Token 2022 program
    pub token_program_2022: UncheckedAccount<'info>,
    pub vault_0_mint: Account<'info, Mint>,
    pub vault_1_mint: Account<'info, Mint>,
    /// CHECK: Raydium program
    pub raydium_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct IncreaseLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    pub protocol_position: AccountInfo<'info>,
    #[account(mut)]
    pub personal_position: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_lower: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_upper: AccountInfo<'info>,
    #[account(mut)]
    pub token_account_0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_account_1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault_0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault_1: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Token 2022 program
    pub token_program_2022: UncheckedAccount<'info>,
    pub vault_0_mint: Account<'info, Mint>,
    pub vault_1_mint: Account<'info, Mint>,
    /// CHECK: Raydium program
    pub raydium_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct DecreaseLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub personal_position: AccountInfo<'info>,
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    pub protocol_position: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault_1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tick_array_lower: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_upper: AccountInfo<'info>,
    #[account(mut)]
    pub recipient_token_account_0: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account_1: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Token 2022 program
    pub token_program_2022: UncheckedAccount<'info>,
    /// CHECK: Memo program
    pub memo_program: UncheckedAccount<'info>,
    pub vault_0_mint: Account<'info, Mint>,
    pub vault_1_mint: Account<'info, Mint>,
    /// CHECK: Raydium program
    pub raydium_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub position_nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub position_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub personal_position: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Raydium program
    pub raydium_program: UncheckedAccount<'info>,
}