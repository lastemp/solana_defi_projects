import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StableCoinIssuer } from "../target/types/stable_coin_issuer";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  burn,
  getAccount,
  Account,
} from "@solana/spl-token";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("stable_coin_issuer", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace
    .StableCoinIssuer as Program<StableCoinIssuer>;

  const depositAccount = anchor.web3.Keypair.generate();
  const payer = wallet.payer;

  let wrappedMint: anchor.web3.PublicKey;
  const userOwner = anchor.web3.Keypair.generate();
  let userWrappedTokenAccount: Account;

  // pdaAuth
  let [pdaAuth, adminPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      depositAccount.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [treasuryVault, adminTreasuryBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("treasury-vault"), pdaAuth.toBuffer()],
      program.programId
    );

  // user owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      userOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  before(async () => {
    // Create wrapped token mint
    wrappedMint = await createMint(
      provider.connection,
      payer,
      treasuryVault,
      null,
      6 // Decimals
    );

    // Create associated token accounts
    userWrappedTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      wrappedMint,
      userOwner.publicKey
    );
  });

  it("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .init()
        .accounts({
          owner: payer.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer, depositAccount])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Deposits funds and mints wrapped tokens", async () => {
    // Lets assume 1 sol is equal to 100 usdc
    let requestParams = {
      depositAmount: new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL),
      stableCoinAmount: new anchor.BN(100),
    };

    try {
      const tx = await program.methods
        .deposit(requestParams)
        .accounts({
          user: userOwner.publicKey,
          wrappedMint: wrappedMint,
          userWrappedTokenAccount: userWrappedTokenAccount.address,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([userOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    // Verify balances after deposit
    const userWrappedTokenAccountInfo = await getAccount(
      provider.connection,
      userWrappedTokenAccount.address
    );

    console.log(
      "UserWrappedTokenAccount amount",
      Number(userWrappedTokenAccountInfo.amount)
    );

    console.log("userOwner address: " + userOwner.publicKey.toBase58());
    console.log("treasuryVault address: " + treasuryVault.toBase58());

    assert.strictEqual(Number(userWrappedTokenAccountInfo.amount), Number(100)); // 100 wrapped tokens minted
  });

  it("Withdraw funds and burns wrapped tokens", async () => {
    // Lets assume 1 sol is equal to 100 usdc
    let requestParams = {
      withdrawalAmount: new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL),
      stableCoinAmount: new anchor.BN(100),
    };

    try {
      const tx = await program.methods
        .withdraw(requestParams)
        .accounts({
          user: userOwner.publicKey,
          wrappedMint: wrappedMint,
          userWrappedTokenAccount: userWrappedTokenAccount.address,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([userOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    // Verify balances after withdrawal

    const userWrappedTokenAccountInfo = await getAccount(
      provider.connection,
      userWrappedTokenAccount.address
    );

    console.log(
      "userWrappedTokenAccount amount",
      Number(userWrappedTokenAccountInfo.amount)
    );

    assert.strictEqual(Number(userWrappedTokenAccountInfo.amount), Number(0)); // Wrapped tokens burned
  });
});
