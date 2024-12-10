import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CustodialBridge } from "../target/types/custodial_bridge";
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

describe("custodial_bridge", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.CustodialBridge as Program<CustodialBridge>;
  const depositAccount = anchor.web3.Keypair.generate();
  const payer = wallet.payer;

  let mint: anchor.web3.PublicKey;
  let wrappedMint: anchor.web3.PublicKey;
  const userTokenOwner = anchor.web3.Keypair.generate();
  let userTokenAccount: Account;
  let custodianTokenAccount: Account;
  let userWrappedTokenAccount: Account;

  // pdaAuth
  let [pdaAuth, adminPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      depositAccount.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [mintAuthority, adminTreasuryBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("mint-authority"), pdaAuth.toBuffer()],
      program.programId
    );

  before(async () => {
    // Create original token mint
    mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      6 // Decimals
    );

    // Create wrapped token mint
    wrappedMint = await createMint(
      provider.connection,
      payer,
      mintAuthority,
      null,
      6 // Decimals
    );

    // Create associated token accounts
    userTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      userTokenOwner.publicKey
    );

    custodianTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      payer.publicKey
    );

    userWrappedTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      wrappedMint,
      payer.publicKey
    );

    // Mint some tokens to the user's token account for testing
    const tx = await mintTo(
      provider.connection,
      payer,
      mint,
      userTokenAccount.address,
      payer,
      1000 // Mint 1000 tokens
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
          mintAuthority: mintAuthority,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer, depositAccount])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Deposits tokens and mints wrapped tokens", async () => {
    const depositAmount = 100;
    let requestParams = {
      amount: new anchor.BN(depositAmount),
    };

    try {
      const tx = await program.methods
        .depositTokens(requestParams)
        .accounts({
          user: userTokenOwner.publicKey,
          userTokenAccount: userTokenAccount.address,
          custodianTokenAccount: custodianTokenAccount.address,
          wrappedMint: wrappedMint,
          userWrappedTokenAccount: userWrappedTokenAccount.address,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          mintAuthority: mintAuthority,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([userTokenOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    // Verify balances after deposit
    const userTokenAccountInfo = await getAccount(
      provider.connection,
      userTokenAccount.address
    );
    const custodianTokenAccountInfo = await getAccount(
      provider.connection,
      custodianTokenAccount.address
    );
    const userWrappedTokenAccountInfo = await getAccount(
      provider.connection,
      userWrappedTokenAccount.address
    );

    console.log("UserTokenAccount amount", Number(userTokenAccountInfo.amount));

    console.log(
      "CustodianTokenAccount amount",
      Number(custodianTokenAccountInfo.amount)
    );

    console.log(
      "UserWrappedTokenAccount amount",
      Number(userWrappedTokenAccountInfo.amount)
    );

    assert.strictEqual(Number(userTokenAccountInfo.amount), Number(900)); // 100 tokens deducted
    assert.strictEqual(Number(custodianTokenAccountInfo.amount), Number(100)); // 100 tokens in custodian account
    assert.strictEqual(Number(userWrappedTokenAccountInfo.amount), Number(100)); // 100 wrapped tokens minted
  });

  it("Withdraws tokens and burns wrapped tokens", async () => {
    const withdrawAmount = 100;
    let requestParams = {
      amount: new anchor.BN(withdrawAmount),
    };

    try {
      const tx = await program.methods
        .withdrawTokens(requestParams)
        .accounts({
          user: payer.publicKey,
          custodianTokenAccount: custodianTokenAccount.address,
          wrappedMint: wrappedMint,
          userWrappedTokenAccount: userWrappedTokenAccount.address,
          userTokenAccount: userTokenAccount.address,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    // Verify balances after withdrawal
    const userTokenAccountInfo = await getAccount(
      provider.connection,
      userTokenAccount.address
    );

    const custodianTokenAccountInfo = await getAccount(
      provider.connection,
      custodianTokenAccount.address
    );

    const userWrappedTokenAccountInfo = await getAccount(
      provider.connection,
      userWrappedTokenAccount.address
    );

    console.log("userTokenAccount amount", Number(userTokenAccountInfo.amount));

    console.log(
      "custodianTokenAccount amount",
      Number(custodianTokenAccountInfo.amount)
    );

    console.log(
      "userWrappedTokenAccount amount",
      Number(userWrappedTokenAccountInfo.amount)
    );

    assert.strictEqual(Number(userTokenAccountInfo.amount), Number(1000)); // Tokens returned to user
    assert.strictEqual(Number(custodianTokenAccountInfo.amount), Number(0)); // Custodian account emptied
    assert.strictEqual(Number(userWrappedTokenAccountInfo.amount), Number(0)); // Wrapped tokens burned
  });
});
