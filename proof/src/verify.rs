use std::{fs::File, io::Write};

use halo2_proofs::{halo2curves::bn256::Fr as Fp, plonk::Circuit};
use halo2_solidity_verifier::{BatchOpenScheme::Bdfg21, SolidityGenerator};

use crate::prove::generate_params;

pub fn generate_verifier_solidity(k: u32, circuit: &impl Circuit<Fp>) {
    let (params, _, vk) = generate_params(k, circuit).unwrap();

    let generator = SolidityGenerator::new(&params, &vk, Bdfg21, 1);
    let (verifier_solidity, vk_solidity) = generator.render_separately().unwrap();

    File::create("./Verifier.sol")
        .unwrap()
        .write_all(verifier_solidity.as_bytes())
        .unwrap();
    File::create("./VerifyingKey.sol")
        .unwrap()
        .write_all(vk_solidity.as_bytes())
        .unwrap();
}
