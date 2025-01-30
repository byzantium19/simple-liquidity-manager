import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { RaydiumSimpleLiquidityManager } from '../target/types/raydium_simple_liquidity_manager';
import { TOKEN_PROGRAM_ID, createMint, createAccount } from '@solana/spl-token';

describe('raydium-simple-liquidity-manager', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.RaydiumSimpleLiquidityManager as Program<RaydiumSimpleLiquidityManager>;

    it('Remove and add liquidity in same transaction', async () => {
        // Create participants
        const user = anchor.web3.Keypair.generate();
        const pool = anchor.web3.Keypair.generate();

        // Airdrop SOL to user
        const signature = await provider.connection.requestAirdrop(
            user.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        );
        await provider.connection.confirmTransaction(signature);

        // Create token mint
        const mint = await createMint(
            provider.connection,
            user,
            user.publicKey,
            null,
            9
        );

        // Create token accounts
        const userToken = await createAccount(
            provider.connection,
            user,
            mint,
            user.publicKey
        );

        const poolToken = await createAccount(
            provider.connection,
            user,
            mint,
            pool.publicKey
        );

        const amount = new anchor.BN(100);

        // Create remove liquidity instruction
        const removeIx = await program.methods
            .removeLiquidity(amount)
            .accounts({
                user: user.publicKey,
                pool: pool.publicKey,
                userToken: userToken,
                poolToken: poolToken,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .instruction();

        // Create add liquidity instruction
        const addIx = await program.methods
            .addLiquidity(amount)
            .accounts({
                user: user.publicKey,
                pool: pool.publicKey,
                userToken: userToken,
                poolToken: poolToken,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .instruction();

        // Create and send transaction with both instructions
        const tx = new anchor.web3.Transaction()
            .add(removeIx)
            .add(addIx);

        await anchor.web3.sendAndConfirmTransaction(
            provider.connection,
            tx,
            [user]
        );
    });
});