use halo2_proofs::{
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Column, ConstraintSystem, Fixed},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct RangeCheckConfig {
    pub lookup_table: Column<Fixed>,
}

pub struct RangeCheckChip;

impl RangeCheckChip {
    pub fn construct() -> Self {
        Self {}
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        values: Column<Advice>,
        lookup_table: Column<Fixed>,
    ) -> RangeCheckConfig {
        meta.lookup_any("range check constraint", |meta| {
            let value = meta.query_advice(values, Rotation::cur());
            let range = meta.query_fixed(lookup_table, Rotation::cur());
            vec![(value, range)]
        });
        RangeCheckConfig { lookup_table }
    }
}
