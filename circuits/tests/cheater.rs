use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, MockProver},
    halo2curves::bn256::Fr as Fp,
};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;

#[test]
fn cheater() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let public_value = Fp::from(7);

    let k = 4;
    let mut prover = MockProver::run(k, &circuit, vec![vec![public_value]]).unwrap();

    prover.modify_advice(Fp::from(2), 0, 0);

    let advice0 = prover.advice_mut(0);
    advice0[1] = CellValue::Assigned(Fp::from(4));
    advice0[2] = CellValue::Assigned(Fp::from(6));

    prover.modify_instance(Fp::from(6), 0, 0);

    assert!(prover.verify().is_ok());
}
