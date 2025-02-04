//! Autogenerated weights for fee_multiplier
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-06-16, STEPS: [20, ], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("main-coded"), DB CACHE: 128

// NOTE: this was benchmark on arbitrary hardware, not reference setup defined for the network
// it's reasonable to rerun it again for precise results

// Executed Command:
// ./target/release/framenode
// benchmark
// --chain
// main-coded
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// fee_multiplier
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

use common::weights::constants::EXTRINSIC_FIXED_WEIGHT;

/// Weight functions for fee_multiplier.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    fn update_multiplier(_m: u32) -> Weight {
        (23_978_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

impl crate::WeightInfo for () {
    fn update_multiplier(_m: u32) -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
