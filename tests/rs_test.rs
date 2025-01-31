use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

// Define known Raydium constants
const RAYDIUM_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const SOL_USDC_POOL_ID: &str = "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const WRAPPED_SOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[tokio::test]
async fn test_remove_and_add_liquidity() -> Result<(), Box<dyn std::error::Error>> {
    // Create program test environment
    let program_id = "3QMb6zycgRf62uMCok5Tw4iRnzuAoLpPWLYszefm2MaB";
    let mut program_test = ProgramTest::new(
        "raydium_simple_liquidity_manager",
        program_id,
        None
    );

    // Start the test context
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create test accounts
    let user = Keypair::new();
    let raydium_program = Pubkey::from_str(RAYDIUM_PROGRAM_ID)?;
    let pool_id = Pubkey::from_str(SOL_USDC_POOL_ID)?;
    let usdc_mint = Pubkey::from_str(USDC_MINT)?;
    let wsol_mint = Pubkey::from_str(WRAPPED_SOL_MINT)?;

    // Create token accounts for testing
    let user_sol_account = create_token_account(
        &mut banks_client,
        &payer,
        &wsol_mint,
        &user.pubkey(),
    ).await?;

    let user_usdc_account = create_token_account(
        &mut banks_client,
        &payer,
        &usdc_mint,
        &user.pubkey(),
    ).await?;

    let user_lp_token_account = create_token_account(
        &mut banks_client,
        &payer,
        &pool_id, // LP token mint is derived from pool
        &user.pubkey(),
    ).await?;

    // Test amounts
    let amount = 1_000_000; // 1 USDC (6 decimals)

    // Create remove liquidity instruction
    let remove_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(pool_id, false),
            AccountMeta::new(user_lp_token_account, false),
            AccountMeta::new(user_sol_account, false),
            AccountMeta::new(user_usdc_account, false),
            AccountMeta::new_readonly(raydium_program, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: serialize_instruction_data("remove_liquidity", amount)?,
    };

    // Create add liquidity instruction
    let add_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(pool_id, false),
            AccountMeta::new(user_sol_account, false),
            AccountMeta::new(user_usdc_account, false),
            AccountMeta::new(user_lp_token_account, false),
            AccountMeta::new_readonly(raydium_program, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: serialize_instruction_data("add_liquidity", amount)?,
    };

    // Create transaction with both instructions
    let mut transaction = Transaction::new_with_payer(
        &[remove_ix, add_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &user], recent_blockhash);

    // Send and confirm transaction
    banks_client.process_transaction(transaction).await?;

    Ok(())
}

// Helper function to create token accounts for testing
async fn create_token_account(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let token_account = Keypair::new();

    let rent = banks_client.get_rent().await?.minimum_balance(TokenAccount::LEN);

    let create_account_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &token_account.pubkey(),
        rent,
        TokenAccount::LEN as u64,
        &spl_token::id(),
    );

    let initialize_account_ix = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &token_account.pubkey(),
        mint,
        owner,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_account_ix],
        Some(&payer.pubkey()),
        &[payer, &token_account],
        banks_client.get_recent_blockhash().await?,
    );

    banks_client.process_transaction(transaction).await?;

    Ok(token_account.pubkey())
}

// Helper function to serialize instruction data
fn serialize_instruction_data(
    instruction_name: &str,
    amount: u64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();

    // Get the correct 8-byte discriminator
    let discriminator = match instruction_name {
        "add_liquidity" => anchor_lang::InstructionData::discriminator("add_liquidity"),
        "remove_liquidity" => anchor_lang::InstructionData::discriminator("remove_liquidity"),
        _ => return Err("Invalid instruction name".into()),
    };

    // Add the 8-byte discriminator
    data.extend_from_slice(&discriminator);

    // Add amount
    data.extend_from_slice(&amount.to_le_bytes());

    Ok(data)
}