# synthetic_assets

This is a Synthetic Assets Rust Smart Contract(Solana Blockchain).
We will use compressed NFTs to represent real estate assets on-chain.

Below are some features contained in the program:

- Create tree
- Mint compressed nfts

## Getting started

In order to run this example program you will need to install Rust and
Solana. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

The smart contract needs to be deployed on Devnet as there is the dependency on the Bubblegum contract.

## Install typescript client dependencies

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../sky_trade_compressed_nft
```

1. JavaScript client for Mpl Bubblegum:

   ```
   npm install @metaplex-foundation/mpl-bubblegum
   ```

1. Metaplex JavaScript SDK:

   ```
   npm install @metaplex-foundation/js @solana/web3.js
   ```
   
1. A TypeScript library for interacting with SPL Account Compression and SPL NoOp:

   ```
   npm install --save @solana/spl-account-compression @solana/web3.js
   ```
   
   or
   
   ```
   yarn add @solana/spl-account-compression @solana/web3.js
   ```
   
1. JavaScript client for Mpl Token Metadata:

   ```
   npm i @metaplex-foundation/mpl-token-metadata
   ```
   
Once you've completed the installations run the following
commands to configure your machine for deployment:

```
solana config set --url devnet
solana-keygen new
```

These two commands create Solana config files in `~/.config/solana/`
which solana command line tools will read in to determine what cluster
to connect to and what keypair to use.

Having done that run a local Solana validator by executing:

```
solana-test-validator
```

This program must be left running in a separate terminal window.   

## Deploying the Solana program

To deploy the Solana program in this repository to the Solana cluster
that you have configured run:

```
anchor build
```

```
anchor deploy
```

## Running the test program

To run the test program you must have already deployed the Solana
program. The test program sends a transaction to the Solana
blockchain asking it to execute the deployed program and reports the
results.

```
anchor test --skip-local-validator
```
