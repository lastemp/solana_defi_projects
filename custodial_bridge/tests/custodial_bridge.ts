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
} from "@solana/spl-token";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("custodial_bridge", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.CustodialBridge as Program<CustodialBridge>;
  const payer = wallet.payer;

  let mint: anchor.web3.PublicKey;
  let wrappedMint: anchor.web3.PublicKey;
  let userTokenAccount: anchor.web3.PublicKey;
  let custodianTokenAccount: anchor.web3.PublicKey;
  let userWrappedTokenAccount: anchor.web3.PublicKey;

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
      payer.publicKey,
      null,
      6 // Decimals
    );

    // Create associated token accounts
    /*
    userTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        payer.publicKey
      )
    ).address;
    */

    custodianTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        payer.publicKey
      )
    ).address;

    userWrappedTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        wrappedMint,
        payer.publicKey
      )
    ).address;
    //console.log("payer.publicKey: ", payer.publicKey);
    console.log("Connection:", provider.connection);
    console.log("Payer:", payer);
    console.log("Mint Public Key:", mint.toString());
    //console.log("User Token Account:", userTokenAccount.toString());
    console.log("Payer Public Key:", payer.publicKey.toString());

    // Create user's token account
    const userAccountInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      provider.wallet.publicKey
    );
    userTokenAccount = userAccountInfo.address;

    // Debug user token account details
    console.log("User Token Account:", userTokenAccount.toString());

    // Mint some tokens to the user's token account for testing
    try {
      const tx = await mintTo(
        provider.connection,
        payer,
        mint,
        userTokenAccount,
        payer.publicKey,
        1000 // Mint 1000 tokens
      );
      console.log("mintTo was successful");
    } catch (error) {
      console.log("mintTo: ", error);
    }
  });

  it("Deposits tokens and mints wrapped tokens", async () => {
    const depositAmount = 100;
    let requestParams = {
      amount: new anchor.BN(depositAmount),
    };
    /*
    try {
      const tx = await program.methods
        .depositTokens(requestParams)
        .accounts({
          user: payer.publicKey,
          userTokenAccount: userTokenAccount,
          custodianTokenAccount: custodianTokenAccount,
          wrappedMint: wrappedMint,
          userWrappedTokenAccount: userWrappedTokenAccount,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
    */

    /*
    // Verify balances after deposit
    const userTokenAccountInfo = await getAccount(
      provider.connection,
      userTokenAccount
    );
    const custodianTokenAccountInfo = await getAccount(
      provider.connection,
      custodianTokenAccount
    );
    const userWrappedTokenAccountInfo = await getAccount(
      provider.connection,
      userWrappedTokenAccount
    );

    expect(Number(userTokenAccountInfo.amount)).toBe(900); // 100 tokens deducted
    expect(Number(custodianTokenAccountInfo.amount)).toBe(100); // 100 tokens in custodian account
    expect(Number(userWrappedTokenAccountInfo.amount)).toBe(100); // 100 wrapped tokens minted
    */
  });

  /*
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
          custodianTokenAccount: custodianTokenAccount,
          wrappedMint: wrappedMint,
          userWrappedTokenAccount: userWrappedTokenAccount,
          userTokenAccount: userTokenAccount,
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
      userTokenAccount
    );
    const custodianTokenAccountInfo = await getAccount(
      provider.connection,
      custodianTokenAccount
    );
    const userWrappedTokenAccountInfo = await getAccount(
      provider.connection,
      userWrappedTokenAccount
    );

    expect(Number(userTokenAccountInfo.amount)).toBe(1000); // Tokens returned to user
    expect(Number(custodianTokenAccountInfo.amount)).toBe(0); // Custodian account emptied
    expect(Number(userWrappedTokenAccountInfo.amount)).toBe(0); // Wrapped tokens burned
  });
  */
});
