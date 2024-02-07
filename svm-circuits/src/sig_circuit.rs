use halo2_base::utils::CurveAffineExt;
use halo2_ecc::{halo2_base::{utils::BigPrimeField}, ecc::EccChip};
use halo2_ecc::fields::fp::FpChip;
use halo2_proofs::{
    arithmetic::Field,
    circuit::{
        SimpleFloorPlanner,
    }
    plonk::{
        Circuit,
        Column, 
        Advice,
        Instance,
        Selector,
        ConstraintSystem,
    }
};

use chips::eddsa_chip::{self, EddsaChip};
/*
@note

•   this circuit verifies that the list of signatures in a transaction
    correctly correspond to the accounts passed.

•   A solana transaction has a list of signatures and a list of
    non-signing accounts. Here, we make sure that 

•   pub struct Transaction {
        pub signatures: Vec,
        pub message: Message,
    }

•   pub struct Message {
        pub header: MessageHeader,
        pub account_keys: Vec,
        pub recent_blockhash: Hash,
        pub instructions: Vec,
    }

*/
pub struct SigCircuitConfig<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> 
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    pub eddsa_chip: EddsaChip<'chip, F, BF, SF, C>,
    pub sig_advice: Column<Advice>,
    pub signer_advice: Column<Advice>,
    pub instance: Column<Instance>,
    pub selector: Selector,

}
pub struct SigCircuit<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> 
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    //nothing yet tx circuit for solana VM
    pub sc_config: SigCircuitConfig<'chip, F, BF, SF, C>,
    pub sigs: Vec<Vec<u8>>,
    pub signers: Vec<Vec<u8>>, 
}

impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> SubCircuit<F> for SigCircuit<'chip, F, BF, SF, C>
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
}
