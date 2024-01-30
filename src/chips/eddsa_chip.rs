use std::marker::PhantomData;


use halo2_ecc::ecc::EccChip;
use halo2_ecc::halo2_base::utils::{BigPrimeField, ScalarField};
use halo2_proofs::pasta::group::ff::PrimeField;
use halo2_proofs::plonk::{Advice, Instance, Selector, Column, ConstraintSystem};
use halo2_proofs::arithmetic::Field;
use halo2_proofs::circuit::Chip;
use halo2_ecc::fields::{fp::FpChip, FieldChip};
use halo2_base::gates::range::RangeChip;
/*
@note
•   EdDSA field chip needs to check the following:
    •   signature is inside Ed25519 scalar field
    •   signer address is inside Ed25519 scalar field
    •   all hashed values lie inside Ed25519 the scalar field
•   We need an FpChip for all of these things
*/
#[derive(Clone, Debug)]
pub struct EddsaChipConfig<'chip, F: BigPrimeField, Fp: BigPrimeField>{
    limbs: usize,
    bits_per_limb: usize,
    ecc_chip: &'chip EccChip<'chip, F, FpChip<'chip, F, Fp>>,
}

impl<'chip, F: BigPrimeField, Fp: BigPrimeField> EddsaChipConfig<'chip, F, Fp> {
    fn new(
        ecc_chip: &'chip EccChip<'chip, F, FpChip<'chip, F, Fp>>,
        limbs: usize,
        bits_per_limb: usize,
    ) -> EddsaChipConfig<'chip, F, Fp>{

        EddsaChipConfig { 
            limbs: limbs,
            bits_per_limb: bits_per_limb,
            ecc_chip: ecc_chip,
        }
    }
}

pub struct EddsaChip<'chip, F: BigPrimeField, Fp: BigPrimeField>{
    chip_config: EddsaChipConfig<'chip, F, Fp>,
}

impl<'chip, F: BigPrimeField, Fp: BigPrimeField> EddsaChip<'chip, F, Fp>{
    fn new(limbs: usize, bits_per_limb: usize, ecc_chip: &'chip EccChip<'chip, F, FpChip<'chip, F, Fp>>) -> EddsaChip<'chip, F, Fp>{
        let cf = EddsaChipConfig::new(ecc_chip, limbs, bits_per_limb);
        EddsaChip {
            chip_config: cf,
        }
    }
}
impl<'chip, F: BigPrimeField, Fp: BigPrimeField> Chip<F> for EddsaChip<'chip, F, Fp>{
    type Config = EddsaChipConfig<'chip, F, Fp>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.chip_config
    }

    fn loaded(&self) -> &Self::Loaded {
       &() 
    }
}