use anyhow::Result;
use halo2_proofs::{circuit::Value, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::{
    prove::{generate_params, generate_proof, save_proof_to_file},
    verify::generate_verifier_solidity,
};

fn main() -> Result<()> {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };

    let k = 4;
    let public_value = Fp::from(7);

    let (params, pk, vk) = generate_params(k, &circuit, Some("./proof/ptau/hermez-raw-15"))?;
    let proof = generate_proof(&params, &pk, circuit, vec![vec![public_value]])?;
    let out = save_proof_to_file(&proof, &pk, &params, "proof.json")?;
    println!("Proof successfully saved to {out}");

    let (out1, out2) = generate_verifier_solidity(&params, &vk)?;
    println!("Verifier successfully saved to {out1}");
    println!("VerifierKey successfully saved to {out2}");

    Ok(())
}
