use std::mem::discriminant;

use halo2_proofs::{
    circuit::Value,
    dev::{CellValue, InstanceValue, MockProver},
    halo2curves::bn256::Fr as Fp,
    plonk::Any,
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
    // println!("prover {prover:?}");

    // println!("advice before {:?}", prover.advice());
    // println!("fixed before {:?}", prover.fixed());
    // println!("instance before {:?}", prover.instance());

    let advice0 = prover.advice_mut(0);
    advice0[0] = CellValue::Assigned(Fp::from(1));
    advice0[1] = CellValue::Assigned(Fp::from(2));
    advice0[2] = CellValue::Assigned(Fp::from(3));
    advice0[3] = CellValue::Assigned(Fp::from(5));
    advice0[4] = CellValue::Assigned(Fp::from(5));

    let instance0 = prover.instance_mut(0);
    instance0[0] = InstanceValue::Assigned(Fp::from(3));

    prover.add_to_region(discriminant(&Any::advice()), 0, 3);

    // println!("after {prover:?}");

    println!("verify {:?}", prover.verify());
    // assert!(prover.verify().is_ok());

    println!("ok");
}
