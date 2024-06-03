# Halo2 Starter

This repo will help you start building ZK circuits with [Halo2](https://zcash.github.io/halo2/) (we'll be using the [PSE fork](https://github.com/privacy-scaling-explorations/halo2/)).

> [!TIP]
> If you’re a beginner and trying to learn the basics of developing with Halo2, check out my beginner tutorial where I’ll teach you [how to rewrite TornadoCash with Halo2](https://dev.to/teddav/tornado-cash-with-halo2-62b) > https://github.com/teddav/tornado-halo2

# Prerequisite

You need to have [Rust installed](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## KZG SRS

You're going to need the result of a "Powers of Tau" ceremony. You can download such a file from https://github.com/han0110/halo2-kzg-srs  
Copy the file to `proof/ptau` directory (I already pre-downloaded the `hermez-raw-15` for you).

```bash
wget -P ./proof/ptau https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-15
```

# Write circuits

You can write your circuits in the `circuits` directory. You'll find there my "DummyCircuit" example.

## MockProver

Halo2 provides a `MockProver` to quickly test our circuits. You can find a simple test in `circuits/tests/prove.rs`.  
Run with

```bash
cargo test prove_mock
```

## Real prover

Running a real prover/verifier is a bit more complex. You will find examples on how to do that in `proof/examples`.  
When calling `generate_params()` you can specify a Powers of Tau file, or just generate a proof with random SRS (only for testing!).

You can find the entire flow (generate + verify proof) in `proof/tests/prove.rs`.
Run it:

```bash
cargo test generate_verify_proof
```

Notice how a proof generated this way would be worthless: because the KZG parameters are generated during the test and not through a trusted setup.

### Generate proof

```bash
cargo run --example generate_proof
cargo run --example verify_proof
cargo run --example generate_verifier_solidity
```

The generated proof (and parameters) will be saved in the `output` directory

# Contracts

In `contracts` you can find everything you need to run your on-chain tests.  
Once you've generated the Verifier contracts, copy them to `contracts/src` and then run the `test_Verify` test (in `contracts/test/MyContract.t.sol`)

```bash
forge test --mt test_Verify
```

# Extra features

## Graph

https://zcash.github.io/halo2/user/dev-tools.html

```
Instance columns have a white background.
Advice columns have a red background.
Fixed columns have a blue background.
```

You can generate the graph for your circuit by running

```bash
cargo test generate_graph
```

## Cheat the verifier

I added a special feature just for you: the ability to modify the proof manually and check if verification still passes. This can be useful to check if your circuit is not under-constrained.

You'll find an example on how to do that in `circuits/testts/cheater.rs`, which you can run with

```bash
cargo test cheater
```
