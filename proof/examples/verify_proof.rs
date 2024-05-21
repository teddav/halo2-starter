use std::{fs::File, io::BufReader};

use anyhow::{anyhow, Result};
use halo2_proofs::{
    circuit::Value,
    halo2curves::bn256::{Bn256, Fr as Fp},
    plonk::verify_proof,
    poly::kzg::{
        commitment::KZGCommitmentScheme, multiopen::VerifierSHPLONK, strategy::SingleStrategy,
    },
};
use halo2_solidity_verifier::Keccak256Transcript;
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::{
    output_path,
    prove::{generate_params, ProofFile},
};

fn main() -> Result<()> {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };

    let k = 4;
    let public_value = Fp::from(7);

    let (params, _, vk) = generate_params(k, &circuit, Some("./proof/ptau/hermez-raw-15"))?;

    let proof_file = File::open(output_path("proof.json")?)?;
    let reader = BufReader::new(proof_file);
    let proof_json: ProofFile = serde_json::from_reader(reader)?;
    let proof = proof_json
        .proof
        .strip_prefix("0x")
        .ok_or(anyhow!("couldn't read proof from json"))?;
    let proof = hex::decode(proof)?;

    let strategy = SingleStrategy::new(&params);
    let mut transcript = Keccak256Transcript::new(proof.as_slice());
    verify_proof::<KZGCommitmentScheme<Bn256>, VerifierSHPLONK<Bn256>, _, _, _>(
        &params,
        &vk,
        strategy,
        &[[[public_value].as_ref()].as_ref()],
        &mut transcript,
    )?;

    Ok(())
}
