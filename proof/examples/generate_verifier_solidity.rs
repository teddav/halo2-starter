use anyhow::Result;
use halo2_proofs::{circuit::Value, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::verify::generate_verifier_solidity;

fn main() -> Result<()> {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let k = 4;
    generate_verifier_solidity(k, &circuit)?;

    Ok(())
}
