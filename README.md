# Turbin3 NFT Marketplace

A decentralized NFT marketplace built on Solana using the Anchor framework. This project enables users to list, purchase, and trade NFTs with automated escrow functionality and platform fee collection.

## Features

- **NFT Listing**: Users can list their NFTs for sale with custom pricing
- **Secure Escrow**: NFTs are held in program-controlled vaults during listings
- **Automated Transactions**: Complete purchase flow with automatic fee distribution
- **Collection Verification**: Built-in support for verified NFT collections
- **Fee Management**: Configurable platform fees with treasury collection
- **Rent Optimization**: Automatic account cleanup and rent refunds

## Architecture

The marketplace consists of several key components:

- **Marketplace Account**: Central configuration and fee management
- **Listing Accounts**: Individual NFT sale listings with escrow vaults
- **Treasury Account**: Platform fee collection and management
- **Rewards System**: Built-in token incentives for platform participation

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (latest stable version)
- [Anchor Framework](https://www.anchor-lang.com/docs/getting-started/installation) (latest version)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Yarn](https://yarnpkg.com/) or npm

## Installation

1. Clone the repository:
```bash
git clone https://github.com/kernlog/turbin3-nft-marketplace.git
cd turbin3-nft-marketplace
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

## Development

### Local Development

1. Start a local Solana validator:
```bash
solana-test-validator
```

2. Deploy the program:
```bash
anchor deploy
```

3. Run tests:
```bash
anchor test
```

### Program Structure

```
programs/turbin3-nft-marketplace/src/
├── lib.rs                 # Main program entry point
├── instruction/           # Instruction handlers
│   ├── initialize.rs      # Marketplace initialization
│   ├── list.rs           # NFT listing functionality
│   ├── delist.rs         # NFT delisting functionality
│   └── purchase.rs       # Purchase transaction handling
└── state/                # Account state structures
    ├── marketplace.rs     # Marketplace configuration
    └── listing.rs        # Individual listing data
```

## Usage

### Initializing a Marketplace

```typescript
// Initialize a new marketplace with 2.5% fee
await program.methods
  .initialize("My NFT Market", 250)
  .accounts({
    admin: wallet.publicKey,
    marketplace: marketplacePda,
    treasury: treasuryPda,
    rewardsMint: rewardsMintPda,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Listing an NFT

```typescript
// List an NFT for 1 SOL
await program.methods
  .listing("My NFT Market", new BN(1_000_000_000))
  .accounts({
    maker: wallet.publicKey,
    marketplace: marketplacePda,
    makerMint: nftMint,
    makerAta: makerAta,
    listing: listingPda,
    vault: vaultPda,
    collectionMint: collectionMint,
    metadata: metadataPda,
    masterEdition: masterEditionPda,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    metadataProgram: METADATA_PROGRAM_ID,
  })
  .rpc();
```

### Purchasing an NFT

```typescript
// Purchase a listed NFT
await program.methods
  .purchase()
  .accounts({
    taker: buyer.publicKey,
    maker: seller.publicKey,
    makerMint: nftMint,
    marketplace: marketplacePda,
    takerAta: buyerAta,
    vault: vaultPda,
    rewards: rewardsMintPda,
    listing: listingPda,
    treasury: treasuryPda,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

## Testing

Run the test suite:

```bash
anchor test
```

The test suite includes:
- Marketplace initialization
- NFT listing and delisting
- Purchase transactions
- Fee distribution
- Account cleanup

## Security Considerations

- All NFT transfers use verified metadata and master edition validation
- Escrow vaults are program-controlled with proper authority checks
- Fee calculations use checked arithmetic to prevent overflow
- Account closures include proper rent refunds
- PDA-derived addresses ensure deterministic account creation
