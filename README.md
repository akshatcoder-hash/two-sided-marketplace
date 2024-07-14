# Two-Sided Marketplace

A decentralized two-sided marketplace built on the Solana blockchain. This project allows users to list services as NFTs, purchase services, and resell them.

## Program Deployment

The program is deployed on Solana devnet with the following address:

```
AamrL6gaYNEiFhRreM1JJTb9ZH4uBeNcryuW6Lp5FFR8
```

## Features

- Initialize a marketplace
- List services as NFTs
- Purchase services
- Resell services

## Prerequisites

- Rust
- Solana CLI
- Anchor Framework

## Setup

1. Clone the repository:

   ```
   git clone https://github.com/akshatcoder-hash/two-sided-marketplace.git
   cd two-sided-marketplace
   ```

2. Install dependencies:

   ```
   npm install
   ```

3. Build the program:

   ```
   anchor build
   ```

4. Test the program:
   ```
   anchor test
   ```

## Usage

To interact with the deployed program on devnet, you can use the Solana CLI or create a client application using libraries like `@solana/web3.js` and `@project-serum/anchor`.

Example of initializing the marketplace (pseudocode):

```javascript
const tx = await program.methods
  .initializeMarketplace(5) // 5% fee
  .accounts({
    authority: authorityPublicKey,
    marketplace: marketplacePda,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

## License

[MIT License](LICENSE)

## Contact

For any queries, please open an issue or contact the repository owner.

GitHub: [@akshatcoder-hash](https://github.com/akshatcoder-hash)
