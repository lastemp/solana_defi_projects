import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SyntheticAssets } from "../target/types/synthetic_assets";
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
  ValidDepthSizePair,
  createAllocTreeIx,
} from "@solana/spl-account-compression";
import { MPL_BUBBLEGUM_PROGRAM_ID } from "@metaplex-foundation/mpl-bubblegum";
import {
  Metaplex,
  keypairIdentity,
  CreateCompressedNftOutput,
} from "@metaplex-foundation/js";
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";

describe("synthetic_assets", () => {
  // Configure the client to use the devnet cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.SyntheticAssets as Program<SyntheticAssets>;

  const payer = wallet.payer;
  console.log("payer address: " + payer.publicKey.toBase58());

  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const metaplex = Metaplex.make(connection).use(keypairIdentity(payer));

  // pda "tree creator", allows our program to update the tree
  let [pda, pdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("auth")],
    program.programId
  );
  console.log("Pda address: " + pda.toBase58());

  // keypair for tree
  const merkleTree = Keypair.generate();
  console.log("merkleTree address: " + merkleTree.publicKey.toBase58());

  const MPL_BUBBLEGUM_PROGRAM_ID_KEY = new anchor.web3.PublicKey(
    MPL_BUBBLEGUM_PROGRAM_ID
  );

  // tree authority
  const [treeAuthority] = PublicKey.findProgramAddressSync(
    [merkleTree.publicKey.toBuffer()],
    MPL_BUBBLEGUM_PROGRAM_ID_KEY
  );

  const [bubblegumSigner] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("collection-cpi")],
    MPL_BUBBLEGUM_PROGRAM_ID_KEY
  );

  console.log("bubblegumSigner address: " + bubblegumSigner.toBase58());

  const maxDepthSizePair: ValidDepthSizePair = {
    maxDepth: 14,
    maxBufferSize: 64,
  };
  const canopyDepth = maxDepthSizePair.maxDepth - 5;

  const metadata = {
    uri: "https://arweave.net/xxx", // max length of 200
    name: "PROPERTY", // max length of 20
    symbol: "PRT", // max length of 10
  };

  let collectionNft: CreateCompressedNftOutput;

  before(async () => {
    // Create collection nft
    collectionNft = await metaplex.nfts().create({
      uri: metadata.uri,
      name: metadata.name,
      sellerFeeBasisPoints: 0,
      isCollection: true,
    });

    // transfer collection nft metadata update authority to pda
    await metaplex.nfts().update({
      nftOrSft: collectionNft.nft,
      updateAuthority: wallet.payer,
      newUpdateAuthority: pda,
    });

    // instruction to create new account with required space for tree
    const allocTreeIx = await createAllocTreeIx(
      connection,
      merkleTree.publicKey,
      wallet.publicKey,
      maxDepthSizePair,
      canopyDepth
    );

    const tx = new Transaction().add(allocTreeIx);

    const txSig = await sendAndConfirmTransaction(
      connection,
      tx,
      [wallet.payer, merkleTree],
      {
        commitment: "confirmed",
      }
    );
    console.log(`https://explorer.solana.com/tx/${txSig}?cluster=devnet`);
  });

  it("Is create tree!", async () => {
    let initParams = {
      maxDepth: 14,
      maxBufferSize: 64,
      public: false,
    };

    // CreateTree instruction: Create merkle tree config
    const createTreeInstruction = await program.methods
      .createTree(initParams)
      .accounts({
        payer: payer.publicKey,
        pda: pda,
        treeAuthority: treeAuthority,
        merkleTree: merkleTree.publicKey,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    // Array of instructions
    const instructions: anchor.web3.TransactionInstruction[] = [
      createTreeInstruction,
    ];

    await createAndSendV0Tx(instructions);
  });

  it("Is mint compressed nft!", async () => {
    let initParams = {
      name: metadata.name,
      symbol: metadata.symbol,
      uri: metadata.uri,
    };

    // MintCnft instruction: Mint compressed nft
    const mintCnftInstruction = await program.methods
      .mintCnft(initParams)
      .accounts({
        payer: payer.publicKey,
        pda: pda,
        treeAuthority: treeAuthority,
        merkleTree: merkleTree.publicKey,
        bubblegumSigner: bubblegumSigner,
        collectionMint: collectionNft.mintAddress,
        collectionMetadata: collectionNft.metadataAddress,
        editionAccount: collectionNft.masterEditionAddress,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    // Array of instructions
    const instructions: anchor.web3.TransactionInstruction[] = [
      mintCnftInstruction,
    ];

    await createAndSendV0Tx(instructions);
  });

  async function createAndSendV0Tx(
    txInstructions: anchor.web3.TransactionInstruction[]
  ) {
    // Step 1 - Fetch Latest Blockhash
    let latestBlockhash = await connection.getLatestBlockhash("confirmed");
    console.log(
      "   ✅ - Fetched latest blockhash. Last Valid Height:",
      latestBlockhash.lastValidBlockHeight
    );

    // Step 2 - Generate Transaction Message
    const messageV0 = new anchor.web3.TransactionMessage({
      payerKey: payer.publicKey,
      recentBlockhash: latestBlockhash.blockhash,
      instructions: txInstructions,
    }).compileToV0Message();
    console.log("   ✅ - Compiled Transaction Message");
    const transaction = new anchor.web3.VersionedTransaction(messageV0);

    // Step 3 - Sign your transaction with the required `Signers`
    transaction.sign([payer]);
    console.log("   ✅ - Transaction Signed");

    // Step 4 - Send our v0 transaction to the cluster
    const txid = await connection.sendTransaction(transaction, {
      maxRetries: 5,
    });
    console.log("   ✅ - Transaction sent to network");

    // Step 5 - Confirm Transaction
    const confirmation = await connection.confirmTransaction({
      signature: txid,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    if (confirmation.value.err) {
      throw new Error("   ❌ - Transaction not confirmed.");
    }
    console.log(
      "🎉 Transaction Succesfully Confirmed!",
      "\n",
      `https://explorer.solana.com/tx/${txid}?cluster=devnet`
    );
  }
});
