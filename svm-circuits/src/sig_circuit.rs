use halo2_base::{halo2_proofs::dev::metadata::Constraint, utils::CurveAffineExt};
use halo2_ecc::{halo2_base::{utils::BigPrimeField}, ecc::EccChip};
use halo2_ecc::fields::fp::FpChip;
use halo2_proofs::{
    arithmetic::Field,
    circuit::{
        SimpleFloorPlanner,
    },
    plonk::{
        Circuit,
        Column, 
        Advice,
        Instance,
        Selector,
        ConstraintSystem,
    }
};

use crate::util::{SubCircuitConfig, SubCircuit};
use crate::table::SigTable;
use chips::eddsa_chip::EddsaChip;
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
pub struct SigCircuitConfigArgs<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C>
    where C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    pub eddsa_chip: EddsaChip<'chip, F, BF, SF, C>,
    pub sig_table: SigTable
}
pub struct SigCircuitConfig<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> 
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    pub eddsa_chip: EddsaChip<'chip, F, BF, SF, C>,
    pub sig_table: SigTable
        
}

impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> SubCircuitConfig<F> for SigCircuitConfig<'chip, F, BF, SF, C>
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
        type ConfigArgs = SigCircuitConfigArgs<'chip, F, BF, SF, C>;
    
    fn new(
        cs: &mut ConstraintSystem<F>,
        Self::ConfigArgs{
            eddsa_chip,
            sig_table

        }: Self::ConfigArgs,
    ) -> SigCircuitConfig<'chip, F, BF, SF, C>{

        cs.enable_equality(sig_table.sig_R);
        cs.enable_equality(sig_table.sig_S);
        cs.enable_equality(sig_table.hash_ram);
        cs.enable_equality(sig_table.signer);
        cs.enable_equality(sig_table.is_valid);
        let config = SigCircuitConfig{
            eddsa_chip: eddsa_chip,
            sig_table: sig_table,
         };

         config
    }

}
// pub struct SigCircuit<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> 
//     where
//     C: CurveAffineExt<Base = BF, ScalarExt = SF>{
//     pub sc_config: SigCircuitConfig<'chip, F, BF, SF, C>,
//     pub sigs: Vec<Vec<u8>>,pub uuuuuuuuuubbbbbbbbbb          
//     pub signers: Vec<Vec<u8>>, 
// }

// impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> SubCircuit<F> for SigCircuit<'chip, F, BF, SF, C>
//     where
//     C: CurveAffineExt<Base = BF, ScalarExt = SF>{
// }
