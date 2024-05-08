use halo2_proofs::{
    circuit::Value,
    dev::MockProver,
    halo2curves::bn256::{Bn256, Fr as Fp, G1Affine},
    plonk::{create_proof, keygen_pk, keygen_vk, Circuit, Error, ProvingKey, VerifyingKey},
    poly::kzg::{
        commitment::{KZGCommitmentScheme, ParamsKZG},
        multiopen::ProverSHPLONK,
    },
    transcript::TranscriptWriterBuffer,
};
use halo2_solidity_verifier::Keccak256Transcript;
use rand::rngs::OsRng;

pub fn generate_params(
    k: u32,
    circuit: &impl Circuit<Fp>,
) -> Result<
    (
        ParamsKZG<Bn256>,
        ProvingKey<G1Affine>,
        VerifyingKey<G1Affine>,
    ),
    Error,
> {
    let params = ParamsKZG::<Bn256>::setup(k, OsRng);
    let vk = keygen_vk(&params, circuit)?;
    let pk = keygen_pk(&params, vk.clone(), circuit)?;
    Ok((params, pk, vk))
}

pub fn full_proof(k: u32, circuit: impl Circuit<Fp>) {
    let public_value = Fp::from(7);
    let (params, pk, vk) = generate_params(k, &circuit).unwrap();
    let mut transcript = Keccak256Transcript::new(vec![]);
    create_proof::<
        KZGCommitmentScheme<Bn256>,
        ProverSHPLONK<Bn256>,
        _,
        _,
        Keccak256Transcript<G1Affine, Vec<u8>>,
        _,
    >(
        &params,
        &pk,
        &[circuit],
        [[[public_value].as_ref()].as_ref()].as_ref(),
        OsRng,
        &mut transcript,
    )
    .unwrap();
    let proof = transcript.finalize();
}

fn main() {
    println!("hello");
}
