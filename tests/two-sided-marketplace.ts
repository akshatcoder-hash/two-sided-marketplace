import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TwoSidedMarketplace } from "../target/types/two_sided_marketplace";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, } from "@solana/spl-token";
import { assert } from "chai";
import * as fs from 'fs';

describe("two-sided-marketplace", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TwoSidedMarketplace as Program<TwoSidedMarketplace>;

  const marketplaceKeypair = Keypair.generate();
  const vendorKeypair = Keypair.generate();
  const buyerKeypair = Keypair.generate();

  let marketplacePda: PublicKey;
  let nftMint: PublicKey;
  let serviceListing: PublicKey;

  it("Initializes the marketplace", async () => {
    const [marketplacePdaAddress] = await PublicKey.findProgramAddress(
      [Buffer.from("marketplace")],
      program.programId
    );
    marketplacePda = marketplacePdaAddress;

    // Read the keypair file
    const keypairFile = process.env.ANCHOR_WALLET;
    if (!keypairFile) {
      throw new Error("ANCHOR_WALLET environment variable not set");
    }
    const secretKeyString = fs.readFileSync(keypairFile, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const authorityKeypair = Keypair.fromSecretKey(secretKey);

    await program.methods
      .initializeMarketplace(5) // 5% fee
      .accounts({
        authority: authorityKeypair.publicKey,
        marketplace: marketplacePda,
        systemProgram: SystemProgram.programId,
      })
      .signers([authorityKeypair])
      .rpc();

    const marketplaceAccount = await program.account.marketplace.fetch(marketplacePda);
    assert.equal(marketplaceAccount.authority.toBase58(), authorityKeypair.publicKey.toBase58());
    assert.equal(marketplaceAccount.feePercentage, 5);
  });
  
  it("Lists a service", async () => {
    const vendorKeypair = Keypair.generate();
    const nftMintKeypair = Keypair.generate();
    nftMint = nftMintKeypair.publicKey;
  
    const [serviceListingPda] = await PublicKey.findProgramAddress(
      [Buffer.from("service_listing"), nftMint.toBuffer()],
      program.programId
    );
    serviceListing = serviceListingPda;
  
    const metaplexProgramId = new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID.toString());
    const [metadataPda] = await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        metaplexProgramId.toBuffer(),
        nftMint.toBuffer(),
      ],
      metaplexProgramId
    );
  
    // Fund the vendor's account
    const vendorInitialBalance = 10000000000; // 10 SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(vendorKeypair.publicKey, vendorInitialBalance),
      "confirmed"
    );
  
    console.log("Public Keys:");
    console.log("Vendor:", vendorKeypair.publicKey.toBase58());
    console.log("NFT Mint:", nftMint.toBase58());
    console.log("Service Listing:", serviceListing.toBase58());
    console.log("Metadata:", metadataPda.toBase58());
    console.log("Token Program:", TOKEN_PROGRAM_ID.toBase58());
    console.log("Token Metadata Program:", metaplexProgramId.toBase58());
    console.log("System Program:", SystemProgram.programId.toBase58());
    console.log("Rent:", SYSVAR_RENT_PUBKEY.toBase58());
    console.log("Missing Signature Public Key:", "GPKMuz8QAd2Zef7hRjqPBxoiTDWfza7mvhSktJRW2rKw");
  
    const allKeypairs = [vendorKeypair, nftMintKeypair, Keypair.generate()]; // Add an extra keypair
  
    for (const keypair of allKeypairs) {
      try {
        const tx = await program.methods
          .listService("Test Service", "A test service", new anchor.BN(1000000), false)
          .accounts({
            vendor: vendorKeypair.publicKey,
            serviceListing: serviceListing,
            nftMint: nftMint,
            metadata: metadataPda,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenMetadataProgram: metaplexProgramId,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
          })
          .signers([keypair])
          .rpc();
  
        console.log(`Transaction successful with keypair: ${keypair.publicKey.toBase58()}`);
        console.log("Transaction signature:", tx);
  
        // Wait for the transaction to be confirmed
        await provider.connection.confirmTransaction(tx, "confirmed");
        
        console.log("Transaction confirmed");
  
        // Fetch the account data
        const serviceListingAccount = await program.account.serviceListing.fetch(serviceListing);
        console.log("Service Listing Account:", serviceListingAccount);
        assert.equal(serviceListingAccount.vendor.toBase58(), vendorKeypair.publicKey.toBase58());
        assert.equal(serviceListingAccount.name, "Test Service");
        assert.equal(serviceListingAccount.price.toNumber(), 1000000);
        assert.equal(serviceListingAccount.isSoulbound, false);
  
        break; // If successful, exit the loop
      } catch (error) {
        console.error(`Error with keypair ${keypair.publicKey.toBase58()}:`, error);
      }
    }
  });  
});