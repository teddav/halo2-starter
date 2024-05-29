use crate::chips::range_check::{RangeCheckChip, RangeCheckConfig};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr as Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct DummyCircuit {
    pub a: Value<Fp>,
    pub b: Value<Fp>,
}

#[derive(Clone, Debug)]
pub struct DummyCircuitConfig {
    advice: Column<Advice>,
    instance: Column<Instance>,
    rangecheck_config: RangeCheckConfig,
}

impl Circuit<Fp> for DummyCircuit {
    type Config = DummyCircuitConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let advice = meta.advice_column();
        let instance = meta.instance_column();
        meta.enable_equality(advice);
        meta.enable_equality(instance);

        let lookup_table = meta.lookup_table_column();

        let rangecheck_config = RangeCheckChip::configure(meta, advice, lookup_table);

        DummyCircuitConfig {
            advice,
            instance,
            rangecheck_config,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_table(
            || "assign lookup table",
            |mut table| {
                for (i, v) in (0..9).into_iter().enumerate() {
                    table.assign_cell(
                        || "assign cell in lookup table",
                        config.rangecheck_config.lookup_table,
                        i,
                        || Value::known(Fp::from(v as u64)),
                    )?;
                }
                Ok(())
            },
        )?;

        let out = layouter.assign_region(
            || "main region",
            |mut region| {
                config.rangecheck_config.myselector.enable(&mut region, 0)?;

                let a = region.assign_advice(|| "a", config.advice, 0, || self.a)?;
                let b = region.assign_advice(|| "b", config.advice, 1, || self.b)?;
                region.assign_advice(|| "out", config.advice, 2, || a.value() + b.value())
            },
        )?;

        layouter.constrain_instance(out.cell(), config.instance, 0)?;

        Ok(())
    }
}
