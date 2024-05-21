# Halo2 Starter

This repo will help you start building ZK circuits with [Halo2](https://zcash.github.io/halo2/).

We'll be using the [fork from PSE](https://github.com/privacy-scaling-explorations/halo2/).

If you need a tutorial on building circuits, checkout how to rebuild [Tornado Cash with Halo2](https://dev.to/teddav/tornado-cash-with-halo2-62b).

# Setup
## Prerequisite
- You need to have [Rust installed](https://rustup.rs/)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## KZG SRS
You're going to need the result of a "Powers of Tau" ceremony. You can download such a file from https://github.com/han0110/halo2-kzg-srs  
Copy the file to `proof/ptau` directory (I already pre-downloaded the `hermez-raw-15` for you).  
Example:
```bash
wget -P ./proof/ptau https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-15
```

## Run
```bash
cargo run --example generate_proof
cargo run --example verify_proof
cargo run --example generate_verifier_solidity
```
