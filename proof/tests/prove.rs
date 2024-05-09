use halo2_proofs::{circuit::Value, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::{
    prove::{generate_params, generate_proof},
    verify::generate_verifier_solidity,
};

#[test]
fn prove_with_key_generation() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };

    let k = 4;
    let public_value = Fp::from(7);

    let (params, pk, _) = generate_params(k, &circuit).unwrap();
    let proof = generate_proof(&params, &pk, circuit, [public_value].as_ref()).unwrap();
    println!("{proof:?}");
}

#[test]
fn generate_verifier() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let k = 4;
    generate_verifier_solidity(k, &circuit);
}
