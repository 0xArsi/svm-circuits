//structure inspired by scroll zkevm circuits
use halo2_proofs::plonk::{
    Advice, Instance, Selector, Column, Error
};
use halo2_proofs::circuit::Layouter;


/* 
•   sub-circuit trait which every circuit that comprises the super circuit
•   mock prover can still load this circuit even though it does not implement
    the Circuit trait given out-of-the-box by halo2
*/
pub trait SubCircuitConfig<F>{
    type ConfigArgs;

    fn new(
        cs: &mut ConstraintSystem<F>,
        args: Self::ConfigArgs,
    ) -> Self;
}

pub trait SubCircuit<F>{
    type Config: SubCircuitConfig<F>;

    fn instance(&self) -> Vec<Vec<F>>{
        vec![]
    }

    fn synthesize_sub(
        &self,
        config: &SubCircuitConfig,
        layouter: &mut impl Layouter<F>
    ) -> Result<(), Error>;

}