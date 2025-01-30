use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};
use test_instruction::program::*;

#[tokio::test]
async fn test_remove_and_add_liquidity() {
    // Setup
    let program_id = test_instruction::ID;
    let mut program_test = ProgramTest::new(
        "test_instruction",
        program_id,
        processor!(test_instruction::entry),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let user = Keypair::new();
    let pool = Keypair::new();

    // Create remove liquidity instruction
    let remove_ix = test_instruction::instruction::remove_liquidity(
        RemoveLiquidity {
            user: user.pubkey(),
            pool: pool.pubkey(),
            user_token: user_token.pubkey(),
            pool_token: pool_token.pubkey(),
            token_program: token::ID,
        },
        100, // amount
    );

    // Create add liquidity instruction
    let add_ix = test_instruction::instruction::add_liquidity(
        AddLiquidity {
            user: user.pubkey(),
            pool: pool.pubkey(),
            user_token: user_token.pubkey(),
            pool_token: pool_token.pubkey(),
            token_program: token::ID,
        },
        100, // amount
    );

    // Combine both instructions in one transaction
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[remove_ix, add_ix], // Array of instructions
        Some(&payer.pubkey()),
        &[&payer, &user],
        recent_blockhash,
    );

    // Send and confirm transaction
    banks_client.process_transaction(tx).await.unwrap();
}