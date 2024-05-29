use anyhow::Result;
use halo2_proofs::{circuit::Value, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::{prove::generate_params, verify::generate_verifier_solidity};

fn main() -> Result<()> {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let k = 4;

    let (params, _, vk) = generate_params(k, &circuit, Some("./proof/ptau/hermez-raw-15"))?;

    let (out1, out2) = generate_verifier_solidity(&params, &vk)?;
    println!("Verifier successfully saved to {out1}");
    println!("VerifierKey successfully saved to {out2}");

    Ok(())
}
