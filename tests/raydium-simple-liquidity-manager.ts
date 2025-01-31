import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { RaydiumSimpleLiquidityManager } from '../target/types/raydium_simple_liquidity_manager';
import { TOKEN_PROGRAM_ID, createMint, createAccount } from '@solana/spl-token';
import { Liquidity, Market } from '@raydium-io/raydium-sdk';

describe('raydium-simple-liquidity-manager', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.RaydiumSimpleLiquidityManager as Program<RaydiumSimpleLiquidityManager>;

    // SOL-USDC pool ID (mainnet)
    const POOL_ID = new anchor.web3.PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");

    it('Remove and add liquidity in same transaction', async () => {
        // Fetch pool info
        const poolInfo = await Liquidity.fetchInfo({
            connection: provider.connection,
            poolKeys: await Liquidity.fetchPoolKeys(connection, POOL_ID),
        });

        // Now we have access to:
        const {
            baseVault,     // pool's token A account
            quoteVault,    // pool's token B account
            lpMint,        // LP token mint
            marketId,      // Associated market ID
        } = poolInfo;

        // Create participants
        const user = anchor.web3.Keypair.generate();

        // Airdrop SOL to user
        const signature = await provider.connection.requestAirdrop(
            user.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        );
        await provider.connection.confirmTransaction(signature);

        // Create user's LP token account
        const userLpToken = await createAccount(
            provider.connection,
            user,
            lpMint,
            user.publicKey
        );

        // Create user's token accounts
        const userTokenA = await createAccount(
            provider.connection,
            user,
            poolInfo.baseVault.mint, // Token A mint
            user.publicKey
        );

        const userTokenB = await createAccount(
            provider.connection,
            user,
            poolInfo.quoteVault.mint, // Token B mint
            user.publicKey
        );

        const amount = new anchor.BN(100);

        // Create remove liquidity instruction
        const removeIx = await program.methods
            .removeLiquidity(amount)
            .accounts({
                user: user.publicKey,
                poolInfo: POOL_ID,
                userLpToken: userLpToken,
                poolTokenA: poolInfo.baseVault,
                poolTokenB: poolInfo.quoteVault,
                userTokenA: userTokenA,
                userTokenB: userTokenB,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .instruction();

        // Create add liquidity instruction
        const addIx = await program.methods
            .addLiquidity(amount)
            .accounts({
                user: user.publicKey,
                poolInfo: POOL_ID,
                userTokenA: userTokenA,
                userTokenB: userTokenB,
                poolTokenA: poolInfo.baseVault,
                poolTokenB: poolInfo.quoteVault,
                lpMint: lpMint,
                userLpToken: userLpToken,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .instruction();

        // Create and send transaction
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