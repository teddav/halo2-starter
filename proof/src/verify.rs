use anyhow::Result;
use halo2_proofs::{halo2curves::bn256::Fr as Fp, plonk::Circuit};
use halo2_solidity_verifier::{BatchOpenScheme::Bdfg21, SolidityGenerator};

use crate::{prove::generate_params, save_to_file};

pub fn generate_verifier_solidity(k: u32, circuit: &impl Circuit<Fp>) -> Result<()> {
    let (params, _, vk) = generate_params(k, circuit, None).unwrap();

    let generator = SolidityGenerator::new(&params, &vk, Bdfg21, 1);
    let (verifier_solidity, vk_solidity) = generator.render_separately().unwrap();

    save_to_file(verifier_solidity.as_bytes(), "Verifier.sol")?;
    save_to_file(vk_solidity.as_bytes(), "VerifyingKey.sol")?;

    Ok(())
}
