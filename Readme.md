# Raydium Liquidity Manager

Simple program to manage liquidity positions on Raydium AMM v3.

## Prerequisites

- Rust (1.70.0)
```bash
rustup default 1.70.0
```

- Solana CLI (1.16.13)
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.16.13/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

- Anchor (0.28.0)
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.28.0
avm use 0.28.0
```

- Node.js and Yarn
```bash
npm install -g yarn
```

## Setup

1. Clone repository:
```bash
git clone git@github.com:byzantium19/simple-liquidity-manager.git
cd raydium-simple-liquidity-manager
```

2. Install dependencies:
```bash
yarn install
```

## Testing

1. Start local validator:
```bash
solana-test-validator
```

2. In a new terminal, build program:
```bash
anchor build
```

3. Deploy program:
```bash
anchor deploy
```

4. Run tests:
```bash
anchor test
```

## Test Configuration

The test setup uses SOL-USDC pool on local validator. Configure pool parameters in `tests/raydium-liquidity.ts`:

```typescript
const RAYDIUM_PROGRAM_ID = new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
const SOL_USDC_POOL = new PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
```

## Troubleshooting

- If build fails, try:
```bash
anchor clean
cargo clean
anchor build
```

- For test validator errors:
```bash
solana-test-validator --reset
```

- Verify Solana configuration:
```bash
solana config get
```