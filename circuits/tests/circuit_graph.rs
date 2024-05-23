use halo2_proofs::{circuit::Value, dev::CircuitLayout, halo2curves::bn256::Fr as Fp};
use halo2_starter_circuits::circuits::dummy_circuit::DummyCircuit;

#[test]
fn generate_graph() {
    use plotters::prelude::*;

    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };

    let k = 4;

    let root = BitMapBackend::new("layout.png", (2000, 2000)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root
        .titled("Example Circuit Layout", ("sans-serif", 60))
        .unwrap();

    CircuitLayout::default().render(k, &circuit, &root).unwrap();
}
