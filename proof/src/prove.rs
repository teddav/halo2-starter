use std::{fs::File, io::BufWriter};

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
    SerdeFormat,
};
use halo2_solidity_verifier::Keccak256Transcript;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::save_to_file;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofFile {
    pub proof: String,
    pub key: String,
    pub params: String,
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
    public_inputs: Vec<Vec<Fp>>,
) -> Result<Vec<u8>> {
    let mut transcript = Keccak256Transcript::new(vec![]);

    let instances = &(public_inputs
        .iter()
        .map(|instance| instance.as_slice())
        .collect::<Vec<&[Fp]>>());

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
        &[instances],
        OsRng,
        &mut transcript,
    )?;

    let proof = transcript.finalize();
    Ok(proof)
}

fn params_kzg_to_bytes(params: &ParamsKZG<Bn256>) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(10000);
    {
        let mut stream = BufWriter::new(&mut buf);
        params.write(&mut stream)?;
    }
    Ok(buf)
}

pub fn save_proof_to_file(
    proof: &Vec<u8>,
    pk: &ProvingKey<G1Affine>,
    params: &ParamsKZG<Bn256>,
    filename: &str,
) -> Result<String> {
    let pk = pk.to_bytes(SerdeFormat::RawBytes);
    let params = params_kzg_to_bytes(params)?;

    let to_json = serde_json::to_string(&ProofFile {
        proof: format!("0x{}", hex::encode(proof)),
        key: format!("0x{}", hex::encode(pk)),
        params: format!("0x{}", hex::encode(params)),
    })?;
    let out = save_to_file(to_json.to_string().as_bytes(), filename)?;
    Ok(out)
}
