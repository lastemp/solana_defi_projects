# custodial_bridge

This is a Custodial Bridge Rust Smart Contract(Solana Blockchain).
A Custodial Bridge is a trusted, centralized/decentralized system for moving assets between blockchains by locking assets on one blockchain and minting their representation on another.

Below are some features contained in the program:

- Initialise custodial bridge
- Deposit tokens
- Withdraw tokens
- Mint wBTC token (*for test purposes)
- Burn wBTC token

## Getting started

In order to run this example program you will need to install Rust and
Solana. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

Once you've completed the Solana installation run the following
commands to configure your machine for local deployment:

```
solana config set --url localhost
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
