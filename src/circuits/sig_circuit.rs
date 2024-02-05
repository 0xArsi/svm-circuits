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
// pub struct SigCircuitConfig<F: BigPrimeField>{
//     pub sig_advice: Vec<Column<Advice>>,
//     pub signer_advice: Vec<Column<Advice>>,
//     //We need a chip for the base field and the scalar field
// }
// pub struct SigCircuit<F: BigPrimeField> {
//     //nothing yet tx circuit for solana VM
//     pub sigs: Vec<Vec<u9>>,
//     pub signers: Vec<Vec<u9>>, 
// }

// impl<F: BigPrimeField> SigCircuit<F>{
//     pub fn configure(&mut cs: ConstraintSystem<F>){
//         //assign each
//     }
// }