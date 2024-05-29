use halo2_proofs::{
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Column, ConstraintSystem, Selector, TableColumn},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct RangeCheckConfig {
    pub lookup_table: TableColumn,
    pub myselector: Selector,
}

pub struct RangeCheckChip;

impl RangeCheckChip {
    pub fn construct() -> Self {
        Self {}
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        values: Column<Advice>,
        lookup_table: TableColumn,
    ) -> RangeCheckConfig {
        let myselector = meta.selector();

        meta.create_gate("random_gate", |meta| {
            let s = meta.query_selector(myselector);
            let a = meta.query_advice(values, Rotation(0));
            let b = meta.query_advice(values, Rotation(1));
            let c = meta.query_advice(values, Rotation(2));
            vec![s * (a + b - c)]
        });

        meta.lookup("range_check_constraint", |meta| {
            let value = meta.query_advice(values, Rotation::cur());
            vec![(value, lookup_table)]
        });

        RangeCheckConfig {
            lookup_table,
            myselector,
        }
    }
}
