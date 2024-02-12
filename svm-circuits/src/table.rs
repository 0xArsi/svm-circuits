use halo2_proofs::plonk::{
    Column, 
    Advice,
    Instance, 
    Selector, 
    ConstraintSystem,
};
use halo2_proofs::arithmetic::Field;

#[derive(Clone, Debug)]
pub struct SigTable {

    pub enable: Selector,

    pub base_point: Column<Advice>,

    //(R, S) in signature
    pub sig_R: Column<Advice>,
    pub sig_S: Column<Advice>,

    //message hash
    pub hash_ram: Column<Advice>,

    //table of public keys 
    pub signer: Column<Advice>,

    //contains 0 or 1
    pub is_valid: Column<Advice>,
}

impl SigTable{
    fn construct<F: Field>(cs: &mut ConstraintSystem<F>) -> Self{
        Self {
            enable: cs.selector(), 
            base_point: cs.advice_column(), 
            sig_R: cs.advice_column(), 
            sig_S: cs.advice_column(), 
            hash_ram: cs.advice_column(), 
            signer: cs.advice_column(),
            is_valid: cs.advice_column(), 
        }
    }
}