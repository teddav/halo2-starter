use std::{fs::File, io::BufReader};

use anyhow::{anyhow, Result};
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr as Fp, G1Affine},
    plonk::{Circuit, ProvingKey},
    poly::{commitment::Params, kzg::commitment::ParamsKZG},
    SerdeFormat,
};
use halo2_solidity_verifier::{BatchOpenScheme::Bdfg21, SolidityGenerator};

use crate::{
    output_path,
    prove::{generate_params, ProofFile},
    save_to_file,
};

pub fn generate_verifier_solidity(k: u32, circuit: &impl Circuit<Fp>) -> Result<()> {
    let (params, _, vk) = generate_params(k, circuit, None).unwrap();

    let generator = SolidityGenerator::new(&params, &vk, Bdfg21, 1);
    let (verifier_solidity, vk_solidity) = generator.render_separately().unwrap();

    save_to_file(verifier_solidity.as_bytes(), "Verifier.sol")?;
    save_to_file(vk_solidity.as_bytes(), "VerifyingKey.sol")?;

    Ok(())
}

pub fn read_proof_from_file<C: Circuit<Fp>>(
    filename: &str,
) -> Result<(Vec<u8>, ProvingKey<G1Affine>, ParamsKZG<Bn256>)> {
    let proof_file = File::open(output_path(filename)?)?;

    let reader = BufReader::new(proof_file);
    let proof_json: ProofFile = serde_json::from_reader(reader)?;

    let proof = proof_json
        .proof
        .strip_prefix("0x")
        .ok_or(anyhow!("couldn't read proof from json file {filename}"))?;
    let proof = hex::decode(proof)?;

    let pk = proof_json
        .key
        .strip_prefix("0x")
        .ok_or(anyhow!("couldn't read pk from json file {filename}"))?;
    let pk = hex::decode(pk)?;
    let pk = ProvingKey::from_bytes::<C>(&pk, SerdeFormat::RawBytes)?;

    let params = proof_json
        .params
        .strip_prefix("0x")
        .ok_or(anyhow!("couldn't read params from json file {filename}"))?;
    let params = hex::decode(params)?;
    let mut reader = BufReader::new(&params[..]);
    let params = ParamsKZG::read(&mut reader)?;

    Ok((proof, pk, params))
}
