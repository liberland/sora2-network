// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Autogenerated weights for `assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Amaterasu.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/framenode
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// assets
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use common::weights::constants::EXTRINSIC_FIXED_WEIGHT;
use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    // Storage: System Account (r:1 w:1)
    // Storage: Assets AssetOwners (r:1 w:1)
    // Storage: Permissions Owners (r:2 w:2)
    // Storage: Permissions Permissions (r:2 w:1)
    // Storage: Assets AssetInfos (r:0 w:1)
    fn register() -> Weight {
        (116_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn transfer() -> Weight {
        (22_000_000 as Weight)
    }
    // Storage: Assets AssetInfos (r:1 w:0)
    // Storage: Permissions Permissions (r:1 w:0)
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn mint() -> Weight {
        (71_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn force_mint() -> Weight {
        (49_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn burn() -> Weight {
        (48_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Assets AssetOwners (r:1 w:0)
    // Storage: Assets AssetInfos (r:1 w:1)
    fn set_non_mintable() -> Weight {
        (35_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn update_balance() -> Weight {
        (41_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}

impl crate::WeightInfo for () {
    fn register() -> Weight {
        10 * EXTRINSIC_FIXED_WEIGHT
    }
    fn transfer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn force_mint() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn mint() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn burn() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn set_non_mintable() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn update_balance() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
