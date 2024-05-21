use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;

#[test]
fn prove_mock() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let public_value = Fp::from(7);

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![public_value]]).unwrap();
    assert!(prover.verify().is_ok());
}
