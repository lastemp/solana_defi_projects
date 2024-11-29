import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DexExchange } from "../target/types/dex_exchange";
import {
  Account,
  createAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("dex_exchange", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.DexExchange as Program<DexExchange>;
  const adminOwner = anchor.web3.Keypair.generate();
  const depositAccount = anchor.web3.Keypair.generate();
  /* const usdcMint = new anchor.web3.PublicKey(
    "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
  ); // USDC devnet */

  const payer = wallet.payer;
  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  const mintTokenA = anchor.web3.Keypair.generate(); // dummy usdc token created for test purposes
  const tokenAccountA = anchor.utils.token.associatedAddress({
    mint: mintTokenA.publicKey,
    owner: payer.publicKey,
  });

  let firstLiquidityProviderOwner = anchor.web3.Keypair.generate();
  let firstLiquidityProviderOwnerATA = anchor.web3.Keypair.generate();

  let secondLiquidityProviderOwner = anchor.web3.Keypair.generate();
  let secondLiquidityProviderOwnerATA = anchor.web3.Keypair.generate();

  let firstTraderOwner = anchor.web3.Keypair.generate();
  let firstTraderOwnerATA = anchor.web3.Keypair.generate();

  let secondTraderOwner = anchor.web3.Keypair.generate();
  let secondTraderOwnerATA = anchor.web3.Keypair.generate();

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

  let [liquidityPool] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("liquidity-pool"),
      adminOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [firstLiquidityProvider] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("liquidity-provider"),
      firstLiquidityProviderOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [secondLiquidityProvider] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("liquidity-provider"),
      secondLiquidityProviderOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [firstTrader] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("trader"),
      firstTraderOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [secondTrader] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("trader"),
      secondTraderOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

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

  // first liquidity provider owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      firstLiquidityProviderOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // second liquidity provider owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      secondLiquidityProviderOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // first trader owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      firstTraderOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // second trader owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      secondTraderOwner.publicKey,
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
        tokenA: mintTokenA.publicKey, // token A
        tokenB: mintTokenA.publicKey, // token B
        decimals: 9, // token mint in smallest unit i.e 9 decimals
      };

      const tx = await program.methods
        .init(requestParams)
        .accounts({
          owner: adminOwner.publicKey,
          liquidityPool: liquidityPool,
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
  });

  it("Is create token!", async () => {
    console.log("mint token: ", mintTokenA.publicKey.toBase58());
    console.log("token account: ", tokenAccountA.toBase58());

    try {
      let requestParams = {
        amount: new anchor.BN(200),
      };

      const tx = await program.methods
        .createToken(requestParams)
        .accounts({
          owner: payer.publicKey,
          liquidityPool: liquidityPool,
          mintToken: mintTokenA.publicKey,
          tokenAccount: tokenAccountA,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintTokenA])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - first liquidity provider", async () => {
    console.log(
      "liquidity provider owner token account: ",
      firstLiquidityProviderOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        firstLiquidityProviderOwner,
        mintTokenA.publicKey,
        firstLiquidityProviderOwner.publicKey,
        firstLiquidityProviderOwnerATA
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
          liquidityPool: liquidityPool,
          mintToken: mintTokenA.publicKey,
          fromAccount: tokenAccountA,
          toAccount: firstLiquidityProviderOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintTokenA])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - second liquidity provider", async () => {
    console.log(
      "liquidity provider owner token account: ",
      secondLiquidityProviderOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        secondLiquidityProviderOwner,
        mintTokenA.publicKey,
        secondLiquidityProviderOwner.publicKey,
        secondLiquidityProviderOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let requestParams = {
        amount: new anchor.BN(100),
      };
      const tx = await program.methods
        .transferToken(requestParams)
        .accounts({
          owner: payer.publicKey,
          liquidityPool: liquidityPool,
          mintToken: mintTokenA.publicKey,
          fromAccount: tokenAccountA,
          toAccount: secondLiquidityProviderOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintTokenA])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - first trader", async () => {
    console.log(
      "trader owner token account: ",
      firstTraderOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        firstTraderOwner,
        mintTokenA.publicKey,
        firstTraderOwner.publicKey,
        firstTraderOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let requestParams = {
        amount: new anchor.BN(10),
      };
      const tx = await program.methods
        .transferToken(requestParams)
        .accounts({
          owner: payer.publicKey,
          liquidityPool: liquidityPool,
          mintToken: mintTokenA.publicKey,
          fromAccount: tokenAccountA,
          toAccount: firstTraderOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintTokenA])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - second trader", async () => {
    console.log(
      "trader owner token account: ",
      secondTraderOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        secondTraderOwner,
        mintTokenA.publicKey,
        secondTraderOwner.publicKey,
        secondTraderOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let requestParams = {
        amount: new anchor.BN(20),
      };
      const tx = await program.methods
        .transferToken(requestParams)
        .accounts({
          owner: payer.publicKey,
          liquidityPool: liquidityPool,
          mintToken: mintTokenA.publicKey,
          fromAccount: tokenAccountA,
          toAccount: secondTraderOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintTokenA])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register first liquidity provider!", async () => {
    try {
      const tx = await program.methods
        .registerLiquidityProvider()
        .accounts({
          owner: firstLiquidityProviderOwner.publicKey,
          liquidityProvider: firstLiquidityProvider,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstLiquidityProviderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register second liquidity provider!", async () => {
    try {
      const tx = await program.methods
        .registerLiquidityProvider()
        .accounts({
          owner: secondLiquidityProviderOwner.publicKey,
          liquidityProvider: secondLiquidityProvider,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondLiquidityProviderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register first trader!", async () => {
    try {
      const tx = await program.methods
        .registerTrader()
        .accounts({
          owner: firstTraderOwner.publicKey,
          trader: firstTrader,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstTraderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register second trader!", async () => {
    try {
      const tx = await program.methods
        .registerTrader()
        .accounts({
          owner: secondTraderOwner.publicKey,
          trader: secondTrader,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondTraderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is add liquidity - first liquidity provider!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintTokenA.publicKey,
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
        amountA: new anchor.BN(10),
        amountB: new anchor.BN(10),
      };

      const tx = await program.methods
        .addLiquidity(requestParams)
        .accounts({
          owner: firstLiquidityProviderOwner.publicKey,
          liquidityPool: liquidityPool,
          liquidityProvider: firstLiquidityProvider,
          senderTokens: firstLiquidityProviderOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintTokenA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstLiquidityProviderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.liquidityProvider.fetch(
        firstLiquidityProvider
      );
      console.log("liquidity provider: ", result);

      let result2 = await program.account.pool.fetch(liquidityPool);
      console.log("liquidity pool: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is add liquidity - second liquidity provider!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintTokenA.publicKey,
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
        amountA: new anchor.BN(20),
        amountB: new anchor.BN(20),
      };

      const tx = await program.methods
        .addLiquidity(requestParams)
        .accounts({
          owner: secondLiquidityProviderOwner.publicKey,
          liquidityPool: liquidityPool,
          liquidityProvider: secondLiquidityProvider,
          senderTokens: secondLiquidityProviderOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintTokenA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondLiquidityProviderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.liquidityProvider.fetch(
        secondLiquidityProvider
      );
      console.log("liquidity provider: ", result);

      let result2 = await program.account.pool.fetch(liquidityPool);
      console.log("liquidity pool: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is swap token - first trader!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintTokenA.publicKey,
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
        amountIn: new anchor.BN(2),
        tokenIn: mintTokenA.publicKey,
      };

      const tx = await program.methods
        .swap(requestParams)
        .accounts({
          owner: firstTraderOwner.publicKey,
          liquidityPool: liquidityPool,
          trader: firstTrader,
          senderTokens: firstTraderOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintTokenA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstTraderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.trader.fetch(firstTrader);
      console.log("trader: ", result);

      let result2 = await program.account.pool.fetch(liquidityPool);
      console.log("liquidity pool: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is swap token - second trader!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintTokenA.publicKey,
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
        amountIn: new anchor.BN(3),
        tokenIn: mintTokenA.publicKey,
      };

      const tx = await program.methods
        .swap(requestParams)
        .accounts({
          owner: secondTraderOwner.publicKey,
          liquidityPool: liquidityPool,
          trader: secondTrader,
          senderTokens: secondTraderOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintTokenA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondTraderOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.trader.fetch(secondTrader);
      console.log("trader: ", result);

      let result2 = await program.account.pool.fetch(liquidityPool);
      console.log("liquidity pool: ", result2);
    } catch (error) {
      console.log(error);
    }
  });
});
