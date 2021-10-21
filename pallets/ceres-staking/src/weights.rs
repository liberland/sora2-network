//! Autogenerated weights for ceres_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-10-21, STEPS: [], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("main-coded"), DB CACHE: 128

// Executed Command:
// target\release\framenode.exe
// benchmark
// --chain
// main-coded
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// ceres_staking
// --extrinsic
// *
// --repeat
// 10
// --raw
// --output
// ./

#![allow(unused_parens)]
#![allow(unused_imports)]

use common::weights::constants::EXTRINSIC_FIXED_WEIGHT;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

/// Weight functions for ceres_staking.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    fn deposit() -> Weight {
        (196_600_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn withdraw() -> Weight {
        (205_100_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
}

impl crate::WeightInfo for () {
    fn deposit() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn withdraw() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
