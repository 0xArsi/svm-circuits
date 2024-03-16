use halo2_base::gates::GateInstructions;
use halo2_base::{
    halo2_proofs::dev::metadata::Constraint,
     utils::CurveAffineExt,
    Context,};
use halo2_ecc::{ecc::EccChip, fields::FieldChip, halo2_base::utils::BigPrimeField};
use halo2_ecc::fields::fp::FpChip;
use halo2_proofs::{
    arithmetic::{CurveAffine, Field},
    circuit::{
        Layouter, SimpleFloorPlanner, Value,
    },
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector
    }
};

use crate::util::{SubCircuitConfig, SubCircuit};
use crate::table::SigTable;
use chips::eddsa_chip::EddsaChip;
/*
@note

•   this circuit verifies that the list of signatures in a transaction
    correctly correspond to the accounts passed.G

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
pub struct SigCircuit<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> 
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    pub sc_config: SigCircuitConfig<'chip, F, BF, SF, C>,
}

impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> SubCircuit<F> for SigCircuit<'chip, F, BF, SF, C>
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{

    type Config = SigCircuitConfig<'chip, F, BF, SF, C>;
    
    fn synthesize_sub(
        &self,
        config: impl SubCircuitConfig<F>,
        layouter: &mut impl Layouter<F>,
    ) -> Result<(), Error>{
        /*
        @note
        •   should decompose data for assigning via the assign helper functions?
        */
        self.assign(layouter)?;
        Ok(())
    }
}

impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> SigCircuit<'chip, F, BF, SF, C>
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{

    fn assign(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error>{
        Ok(())
    }
    fn assign_sig_valid(&self, ctx: &mut Context<F>, bp: C, pk: C, hash_ram: SF, sig_r: C, sig_s: SF, eddsa_chip: EddsaChip<'chip, F, BF, SF, C>) -> Result<(), Error>{

        //load ed25519 base point
        
        //load values 
        let base_point = eddsa_chip.chip_config.ecc_chip.load_private::<C>(ctx, bp.into_coordinates());
        let public_key = eddsa_chip.chip_config.ecc_chip.load_private::<C>(ctx, pk.into_coordinates());
        let signature_r = eddsa_chip.chip_config.ecc_chip.load_private::<C>(ctx, sig_r.into_coordinates());

        //let message = eddsa_chip.chip_config.scalar_chip.load_private(ctx, msg);

        let signature_s = eddsa_chip.chip_config.scalar_chip.load_private(ctx, sig_s);

        let hash_of_ram = eddsa_chip.chip_config.scalar_chip.load_private(ctx, hash_ram);

        let sig_is_valid = match eddsa_chip.verify_sig(ctx, base_point, signature_r, signature_s, public_key, hash_of_ram) {
            Ok(()) => 1,
            Error => 0,
        };
        assert!(sig_is_valid == 0 || sig_is_valid == 1, "invalid signature validity bit");

        let validity_bit = eddsa_chip.chip_config.ecc_chip.field_chip().load_private(ctx, BF::from_u128(sig_is_valid as u128));
        let one = eddsa_chip.chip_config.ecc_chip.field_chip().load_private(ctx, BF::ONE); 
        eddsa_chip.chip_config.ecc_chip.field_chip.assert_equal(ctx, validity_bit, one);

        Ok(())
    }
}
