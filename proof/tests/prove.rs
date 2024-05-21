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
use halo2_starter_proof::prove::{generate_params, generate_proof};

#[test]
fn generate_verify_proof() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };

    let k = 4;
    let public_value = Fp::from(7);

    let (params, pk, vk) = generate_params(k, &circuit, None).unwrap();
    let proof = generate_proof(&params, &pk, circuit, [public_value].as_ref()).unwrap();

    let strategy = SingleStrategy::new(&params);
    let mut transcript = Keccak256Transcript::new(proof.as_slice());
    let verif = verify_proof::<KZGCommitmentScheme<Bn256>, VerifierSHPLONK<Bn256>, _, _, _>(
        &params,
        &vk,
        strategy,
        &[[[public_value].as_ref()].as_ref()],
        &mut transcript,
    );

    assert!(verif.is_ok());
}
