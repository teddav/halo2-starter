use anyhow::Result;
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr as Fp},
    plonk::verify_proof,
    poly::kzg::{
        commitment::KZGCommitmentScheme, multiopen::VerifierSHPLONK, strategy::SingleStrategy,
    },
};
use halo2_solidity_verifier::Keccak256Transcript;
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::verify::read_proof_from_file;

fn main() -> Result<()> {
    let public_value = Fp::from(7);
    let (proof, pk, params) = read_proof_from_file::<DummyCircuit>("proof.json")?;

    let strategy = SingleStrategy::new(&params);
    let mut transcript = Keccak256Transcript::new(proof.as_slice());
    verify_proof::<KZGCommitmentScheme<Bn256>, VerifierSHPLONK<Bn256>, _, _, _>(
        &params,
        pk.get_vk(),
        strategy,
        &[[[public_value].as_ref()].as_ref()],
        &mut transcript,
    )?;

    Ok(())
}
