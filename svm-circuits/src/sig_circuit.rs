use halo2_base::utils::CurveAffineExt;
use halo2_ecc::{halo2_base::{utils::BigPrimeField}, ecc::EccChip};
use halo2_ecc::fields::fp::FpChip;
use halo2_proofs::{
    arithmetic::Field,
    plonk::{
        Column, 
        Advice,
        Instance,
        Selector,
        ConstraintSystem,
    }
};

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
//pub struct SigCircuit<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> 
//    where
//    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
//    //nothing yet tx circuit for solana VM
//    pub sigs: Vec<Vec<u8>>,
//    pub signers: Vec<Vec<u8>>, 
//}
//
//impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> SigCircuit<'chip, F, BF, SF, C>
//    where
//    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
//    pub fn configure(cs: &mut ConstraintSystem<F>){
//        //assign each
//    }
//}