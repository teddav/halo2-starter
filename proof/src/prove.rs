use std::fs::File;

use anyhow::{anyhow, Result};
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr as Fp, G1Affine},
    plonk::{create_proof, keygen_pk, keygen_vk, Circuit, ProvingKey, VerifyingKey},
    poly::{
        commitment::Params,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::ProverSHPLONK,
        },
    },
    transcript::TranscriptWriterBuffer,
};
use halo2_solidity_verifier::Keccak256Transcript;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::save_to_file;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofFile {
    pub proof: String,
}

pub fn generate_params(
    k: u32,
    circuit: &impl Circuit<Fp>,
    ptau_path: Option<&str>,
) -> Result<(
    ParamsKZG<Bn256>,
    ProvingKey<G1Affine>,
    VerifyingKey<G1Affine>,
)> {
    let params = match ptau_path {
        Some(ptau_path) => {
            let mut ptau = File::open(ptau_path)?;
            let mut p = ParamsKZG::<Bn256>::read(&mut ptau)?;
            if p.k() < k {
                return Err(anyhow!("ptau error: k is too large. max k: {}", p.k()));
            }
            if p.k() > k {
                p.downsize(k);
            }
            p
        }
        None => ParamsKZG::<Bn256>::setup(k, OsRng),
    };
    let vk = keygen_vk(&params, circuit)?;
    let pk = keygen_pk(&params, vk.clone(), circuit)?;
    Ok((params, pk, vk))
}

pub fn generate_proof(
    params: &ParamsKZG<Bn256>,
    pk: &ProvingKey<G1Affine>,
    circuit: impl Circuit<Fp>,
    public_inputs: &[Fp],
) -> Result<Vec<u8>> {
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
        [[public_inputs].as_ref()].as_ref(),
        OsRng,
        &mut transcript,
    )?;

    let proof = transcript.finalize();
    Ok(proof)
}

pub fn save_proof_to_file(proof: &Vec<u8>, filename: &str) -> Result<()> {
    let serialized_proof = format!("0x{}", hex::encode(proof));
    let to_json = serde_json::to_string(&ProofFile {
        proof: serialized_proof,
    })?;
    save_to_file(to_json.to_string().as_bytes(), filename)?;
    Ok(())
}
