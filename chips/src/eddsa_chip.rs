use std::marker::PhantomData;


use ark_ed25519::{EdwardsAffine as G1Affine, EdwardsConfig, EdwardsProjective as G1Projective}; 
use ark_ed25519::Fr;
use halo2_base::halo2_proofs::halo2curves::ed25519::{Ed25519, Ed25519Affine};
use halo2_base::halo2_proofs::plonk::Assigned;
use halo2_base::utils::{biguint_to_fe, CurveAffineExt};
use halo2_base::{AssignedValue, Context};
use halo2_ecc::bigint::{CRTInteger, ProperCrtUint};
use halo2_ecc::ecc::{EcPoint, EccChip};
use halo2_ecc::halo2_base::utils::{BigPrimeField, ScalarField};
use halo2_proofs::pasta::group::cofactor;
use halo2_proofs::pasta::group::ff::PrimeField;
use halo2_proofs::pasta::pallas::Base;
use halo2_proofs::plonk::{Advice, Column, ConstraintSystem, Error, Instance, Selector};
use halo2_proofs::arithmetic::{CurveAffine, Field};
use halo2_proofs::circuit::{Chip, Value};
use halo2_ecc::fields::{fp::FpChip, FieldChip};
use halo2_base::gates::range::RangeChip;
use halo2_base::halo2_proofs::halo2curves::ed25519::TwistedEdwardsCurveAffineExt;
use num_bigint::BigUint;
/*
@note
•   EdDSA hip needs to check the following:
    •   public key (A), base point (B), signature point (R) are on curve 
    •   S = r + H(R||A||M)s is inside scalar field
    •   all hashed values lie inside Ed25519 the scalar field

•   Note that even though eddsa uses edwards curves this implementation should
    be curve agnostic
•   Implementation inspired by https://github.com/shuklaayush/halo2-lib-eddsa
•   TODO:
    •   Optimize w/ projective coordinate operations
    •   Verify correctness of hash

*/
#[derive(Clone, Debug)]
pub struct EddsaChipConfig<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField> {
    ecc_chip: &'chip EccChip<'chip, F, FpChip<'chip, F, BF>>,
    scalar_chip: &'chip FpChip<'chip, F, SF>,
}

impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField> EddsaChipConfig<'chip, F, BF, SF> {
    fn new(
        ecc_chip: &'chip EccChip<'chip, F, FpChip<'chip, F, BF>>,
        scalar_chip: &'chip FpChip<'chip, F, SF>,
    ) -> EddsaChipConfig<'chip, F, BF, SF>{
        EddsaChipConfig{ 
            ecc_chip: ecc_chip,
            scalar_chip: scalar_chip,
        }
    }
}

//@note deserialization of signature is handled outside of chip
pub struct EddsaChip<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C>
    where
    C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    pub chip_config: &'chip EddsaChipConfig<'chip, F, BF, SF>,
    _marker: PhantomData<C>
}

impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> EddsaChip<'chip, F, BF, SF, C>
where
C: CurveAffineExt<Base = BF, ScalarExt = SF>{
    fn new(chip_config: &'chip EddsaChipConfig<F, BF, SF>) -> EddsaChip<'chip, F, BF, SF, C>{
        EddsaChip{
            chip_config: chip_config,
            _marker: PhantomData::<C>,
        }
    }

    fn verify_sig(
        &self, 
        ctx: &mut Context<F>,
        M: ProperCrtUint<F>, 
        B: EcPoint<F, <FpChip<F, BF> as FieldChip<F>>::FieldPoint>, 
        R: EcPoint<F, <FpChip<F, BF> as FieldChip<F>>::FieldPoint>, 
        S: ProperCrtUint<F>,
        A: EcPoint<F, <FpChip<F, BF> as FieldChip<F>>::FieldPoint>, 
        hash_RAM: ProperCrtUint<F>){

            let eccc = self.chip_config.ecc_chip; 
            let sc = self.chip_config.scalar_chip;
            let bc = eccc.field_chip;

            sc.enforce_less_than_p(ctx, S.clone());


            eccc.assert_is_on_curve::<C>(ctx, &R);
            eccc.assert_is_on_curve::<C>(ctx, &B);
            eccc.assert_is_on_curve::<C>(ctx, &A);


            let lhs = eccc.scalar_mult::<C>(ctx, B.clone(), S.limbs().to_vec(), bc.limb_bits, bc.limb_bits);

            let hram_a = eccc.scalar_mult::<C>(ctx, B, hash_RAM.limbs().to_vec(), bc.limb_bits, bc.limb_bits);

            let rhs = eccc.add_unequal(ctx, R, hram_a, false);

            let diff = eccc.sub_unequal(ctx, lhs, rhs, false);
            let cofac = sc.load_constant_uint(ctx, BigUint::from(8 as usize));
            let diff_mul_cofac = eccc.scalar_mult::<C>(ctx, diff, cofac.limbs().to_vec(), bc.limb_bits, bc.limb_bits); 

            //there is only one point with a zero x coordinate (the additive identity)
            bc.is_zero(ctx, diff_mul_cofac.x);
            


    }

}



impl<'chip, F: BigPrimeField, BF: BigPrimeField, SF: BigPrimeField, C> Chip<F> for EddsaChip<'chip, F, BF, SF, C>
where
C: CurveAffineExt<Base = BF, ScalarExt = SF>{
   type Config = EddsaChipConfig<'chip, F, BF, SF>;
   type Loaded = ();

   fn config(&self) -> &Self::Config {
       self.chip_config
   }

   fn loaded(&self) -> &Self::Loaded {
      &() 
   }
}