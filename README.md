# Halo2 Starter

This repo will help you start building ZK circuits with [Halo2](https://zcash.github.io/halo2/).

We'll be using the [fork from PSE](https://github.com/privacy-scaling-explorations/halo2/).

If you need a tutorial on building circuits, checkout how to rebuild [Tornado Cash with Halo2](https://dev.to/teddav/tornado-cash-with-halo2-62b).

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
You can write your circuits in the `circuits` directory

# MockProver
Halo2 provides a `MockProver` to quickly test our circuits. You can find a simple test in `circuits/tests/prove.rs`.  
Run with
```bash
cargo test prove_mock -- --show-output
```

# Real prover
Running a real prover/verifier is a bit more complex. You will find examples on how to do that in `proof/examples`.  
When calling `generate_params()` you can specify a Powers of Tau file, or just generate a proof with random SRS (only for testing!).

## Run proof examples
```bash
cargo run --example generate_proof
cargo run --example verify_proof
cargo run --example generate_verifier_solidity
```
