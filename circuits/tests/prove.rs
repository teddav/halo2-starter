use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;
use halo2_starter_proof::prove::full_proof;

#[test]
fn prove_mock() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![Fp::from(7)]]).unwrap();
    println!("{prover:#?}");
    assert!(prover.verify().is_ok());
}

#[test]
fn prove_with_key_generation() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let k = 4;
    full_proof(k, circuit);
}
