use halo2_proofs::{
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Column, ConstraintSystem, Fixed},
    poly::Rotation,
};

#[derive(Clone, Debug)]
pub struct RangeCheckConfig {
    pub lookup_table1: Column<Fixed>,
    pub lookup_table2: Column<Fixed>,
}

pub struct RangeCheckChip;

impl RangeCheckChip {
    pub fn construct() -> Self {
        Self {}
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        values: Column<Advice>,
        lookup_table1: Column<Fixed>,
        lookup_table2: Column<Fixed>,
    ) -> RangeCheckConfig {
        meta.lookup_any("range check constraint", |meta| {
            let value1 = meta.query_advice(values, Rotation::cur());
            let range1 = meta.query_fixed(lookup_table1, Rotation::cur());

            // let value2 = meta.query_advice(values, Rotation::cur());
            // let range2 = meta.query_fixed(lookup_table2, Rotation::cur());
            // vec![(value1, range1), (value2, range2)]
            vec![(value1, range1)]
        });

        RangeCheckConfig {
            lookup_table1,
            lookup_table2,
        }
    }
}
