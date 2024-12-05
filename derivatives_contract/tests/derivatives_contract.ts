import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DerivativesContract } from "../target/types/derivatives_contract";
import {
  Account,
  createAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("derivatives_contract", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace
    .DerivativesContract as Program<DerivativesContract>;
  const adminOwner = anchor.web3.Keypair.generate();
  const depositAccount = anchor.web3.Keypair.generate();

  const payer = wallet.payer;
  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  const mintToken = anchor.web3.Keypair.generate(); // dummy usdc token created for test purposes
  const tokenAccount = anchor.utils.token.associatedAddress({
    mint: mintToken.publicKey,
    owner: payer.publicKey,
  });

  let buyerOwner = anchor.web3.Keypair.generate();
  let buyerOwnerATA = anchor.web3.Keypair.generate();
  let sellerOwner = anchor.web3.Keypair.generate();
  let sellerOwnerATA = anchor.web3.Keypair.generate();

  let treasuryVaultATA: Account;

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

  let [derivativeContract] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("derivative-contract"),
      adminOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  /*
  let [buyer] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("buyer"), buyerOwner.publicKey.toBuffer()],
    program.programId
  );

  let [seller] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("seller"),
      sellerOwner.publicKey.toBuffer(),
    ],
    program.programId
  );
  */

  // admin owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // buyer owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      buyerOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // seller owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      sellerOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    try {
      let requestParams = {
        decimals: 9, // token mint in smallest unit i.e 9 decimals
      };

      const tx = await program.methods
        .init(requestParams)
        .accounts({
          owner: adminOwner.publicKey,
          derivativeContract: derivativeContract,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([adminOwner, depositAccount])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.derivativeContract.fetch(
        derivativeContract
      );
      console.log("derivative contract: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is create token!", async () => {
    console.log("mint token: ", mintToken.publicKey.toBase58());
    console.log("token account: ", tokenAccount.toBase58());

    try {
      let requestParams = {
        amount: new anchor.BN(200),
      };

      const tx = await program.methods
        .createToken(requestParams)
        .accounts({
          owner: payer.publicKey,
          derivativeContract: derivativeContract,
          mintToken: mintToken.publicKey,
          tokenAccount: tokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - seller", async () => {
    console.log(
      "seller owner token account: ",
      sellerOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        sellerOwner,
        mintToken.publicKey,
        sellerOwner.publicKey,
        sellerOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let requestParams = {
        amount: new anchor.BN(70),
      };
      const tx = await program.methods
        .transferToken(requestParams)
        .accounts({
          owner: payer.publicKey,
          derivativeContract: derivativeContract,
          mintToken: mintToken.publicKey,
          fromAccount: tokenAccount,
          toAccount: sellerOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is create futures contract!", async () => {
    try {
      let requestParams = {
        expiryDate: new anchor.BN(12),
        underlyingAsset: mintToken.publicKey,
        price: new anchor.BN(3),
        buyer: buyerOwner.publicKey,
        seller: sellerOwner.publicKey,
      };

      const tx = await program.methods
        .createFuturesContract(requestParams)
        .accounts({
          owner: adminOwner.publicKey,
          derivativeContract: derivativeContract,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([adminOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is deposit asset - seller!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintToken.publicKey,
        treasuryVault,
        true
      );
      console.log(
        "treasuryVaultATA address: " + treasuryVaultATA.address.toBase58()
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let requestParams = {
        // 1 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(20),
      };

      const tx = await program.methods
        .depositAsset(requestParams)
        .accounts({
          owner: sellerOwner.publicKey,
          derivativeContract: derivativeContract,
          senderTokens: sellerOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintToken.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([sellerOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.derivativeContract.fetch(
        derivativeContract
      );
      console.log("derivative contract: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is deposit funds - buyer!", async () => {
    try {
      let requestParams = {
        // amount of sol to transfer
        amount: new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL),
      };

      const tx = await program.methods
        .depositFunds(requestParams)
        .accounts({
          owner: buyerOwner.publicKey,
          derivativeContract: derivativeContract,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([buyerOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.derivativeContract.fetch(
        derivativeContract
      );
      console.log("derivative contract: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is settle futures contract!", async () => {
    console.log(
      "buyer owner token account: ",
      buyerOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        buyerOwner,
        mintToken.publicKey,
        buyerOwner.publicKey,
        buyerOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let requestParams = {
        // 1 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(2),
        fundsAmount: new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL),
        buyer: buyerOwner.publicKey,
      };
      const tx = await program.methods
        .settleFuturesContract(requestParams)
        .accounts({
          owner: adminOwner.publicKey,
          seller: sellerOwner,
          derivativeContract: derivativeContract,
          senderTokens: treasuryVaultATA.address,
          recipientTokens: buyerOwnerATA.publicKey,
          mintToken: mintToken.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([adminOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("deposit account: ", result);

      let result2 = await program.account.derivativeContract.fetch(
        derivativeContract
      );
      console.log("derivative contract: ", result2);
    } catch (error) {
      console.log(error);
    }
  });
});
