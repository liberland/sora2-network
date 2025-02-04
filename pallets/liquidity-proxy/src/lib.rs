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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;

use codec::{Decode, Encode};

use assets::AssetIdOf;
use common::prelude::fixnum::ops::{Bounded, Zero as _};
use common::prelude::{Balance, FixedWrapper, QuoteAmount, SwapAmount, SwapOutcome, SwapVariant};
use common::{
    balance, fixed_wrapper, AccountIdOf, DEXInfo, DexIdOf, FilterMode, Fixed, GetMarketInfo,
    GetPoolReserves, LiquidityProxyTrait, LiquidityRegistry, LiquiditySource,
    LiquiditySourceFilter, LiquiditySourceId, LiquiditySourceType, RewardReason, TradingPair,
    VestedRewardsPallet, XSTUSD,
};
use fallible_iterator::FallibleIterator as _;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::{ensure, fail, RuntimeDebug};
use frame_system::ensure_signed;
use itertools::Itertools as _;
pub use pallet::*;
use sp_runtime::traits::{CheckedSub, Zero};
use sp_runtime::DispatchError;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::prelude::*;
use sp_std::{cmp::Ord, cmp::Ordering, vec};

type LiquiditySourceIdOf<T> = LiquiditySourceId<<T as common::Config>::DEXId, LiquiditySourceType>;

type Rewards<AssetId> = Vec<(Balance, AssetId, RewardReason)>;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_utils;

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"liquidity-proxy";
pub const TECH_ACCOUNT_MAIN: &[u8] = b"main";
pub const ADAR_COMMISSION_RATIO: Balance = balance!(0.0075);

/// Possible exchange paths for two assets.
struct ExchangePath<T: Config>(Vec<T::AssetId>);

#[derive(Debug)]
enum AssetType {
    Base,
    SyntheticBase,
    Basic,
    Synthetic,
}

impl AssetType {
    fn determine<T: Config>(
        dex_info: &DEXInfo<T::AssetId>,
        synthetic_assets: &BTreeSet<T::AssetId>,
        asset_id: T::AssetId,
    ) -> Self {
        if asset_id == dex_info.base_asset_id {
            AssetType::Base
        } else if asset_id == dex_info.synthetic_base_asset_id {
            AssetType::SyntheticBase
        } else if synthetic_assets.contains(&asset_id) {
            AssetType::Synthetic
        } else {
            AssetType::Basic
        }
    }
}

macro_rules! forward_or_backward {
    ($ex1:tt, $ex2:tt) => {
        ($ex1, $ex2) | ($ex2, $ex1)
    };
}

impl<T: Config> ExchangePath<T> {
    pub fn new_trivial(
        dex_info: &DEXInfo<T::AssetId>,
        input_asset_id: T::AssetId,
        output_asset_id: T::AssetId,
    ) -> Option<Vec<Self>> {
        use AssetType::*;

        let synthetic_assets = T::PrimaryMarketXST::enabled_target_assets();
        let input_type = AssetType::determine::<T>(dex_info, &synthetic_assets, input_asset_id);
        let output_type = AssetType::determine::<T>(dex_info, &synthetic_assets, output_asset_id);

        match (input_type, output_type) {
            forward_or_backward!(Base, Basic) | forward_or_backward!(Base, SyntheticBase) => {
                Some(vec![Self(vec![input_asset_id, output_asset_id])])
            }
            forward_or_backward!(SyntheticBase, Synthetic) => Some(vec![
                Self(vec![input_asset_id, output_asset_id]),
                Self(vec![
                    input_asset_id,
                    dex_info.base_asset_id,
                    output_asset_id,
                ]),
            ]),
            (Basic, Basic) | forward_or_backward!(SyntheticBase, Basic) => Some(vec![Self(vec![
                input_asset_id,
                dex_info.base_asset_id,
                output_asset_id,
            ])]),
            (Synthetic, Synthetic) => Some(vec![
                Self(vec![
                    input_asset_id,
                    dex_info.synthetic_base_asset_id,
                    output_asset_id,
                ]),
                Self(vec![
                    input_asset_id,
                    dex_info.base_asset_id,
                    output_asset_id,
                ]),
            ]),
            forward_or_backward!(Base, Synthetic) => Some(vec![
                Self(vec![input_asset_id, output_asset_id]),
                Self(vec![
                    input_asset_id,
                    dex_info.synthetic_base_asset_id,
                    output_asset_id,
                ]),
            ]),
            (Basic, Synthetic) => Some(vec![
                Self(vec![
                    input_asset_id,
                    dex_info.base_asset_id,
                    dex_info.synthetic_base_asset_id,
                    output_asset_id,
                ]),
                Self(vec![
                    input_asset_id,
                    dex_info.base_asset_id,
                    output_asset_id,
                ]),
            ]),
            (Synthetic, Basic) => Some(vec![
                Self(vec![
                    input_asset_id,
                    dex_info.synthetic_base_asset_id,
                    dex_info.base_asset_id,
                    output_asset_id,
                ]),
                Self(vec![
                    input_asset_id,
                    dex_info.base_asset_id,
                    output_asset_id,
                ]),
            ]),
            (Base, Base) | (SyntheticBase, SyntheticBase) => None,
        }
    }
}

/// Output of the aggregated LiquidityProxy::quote() price.
#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct AggregatedSwapOutcome<LiquiditySourceType, AmountType> {
    /// A distribution of amounts each liquidity sources gets to swap in the entire trade
    pub distribution: Vec<(LiquiditySourceType, QuoteAmount<AmountType>)>,
    /// The best possible output/input amount for a given trade and a set of liquidity sources
    pub amount: AmountType,
    /// Total fee amount, nominated in XOR
    pub fee: AmountType,
}

impl<LiquiditySourceIdType, AmountType> AggregatedSwapOutcome<LiquiditySourceIdType, AmountType> {
    pub fn new(
        distribution: Vec<(LiquiditySourceIdType, QuoteAmount<AmountType>)>,
        amount: AmountType,
        fee: AmountType,
    ) -> Self {
        Self {
            distribution,
            amount,
            fee,
        }
    }
}

#[derive(Eq, PartialEq, Encode, Decode)]
pub struct QuoteInfo<AssetId, LiquiditySource> {
    pub outcome: SwapOutcome<Balance>,
    pub amount_without_impact: Option<Balance>,
    pub rewards: Rewards<AssetId>,
    pub liquidity_sources: Vec<LiquiditySource>,
    pub path: Vec<AssetId>,
}

fn merge_two_vectors_unique<T: PartialEq>(vec_1: &mut Vec<T>, vec_2: Vec<T>) {
    for el in vec_2 {
        if !vec_1.contains(&el) {
            vec_1.push(el);
        }
    }
}

pub trait WeightInfo {
    fn swap(variant: SwapVariant) -> Weight;
    fn enable_liquidity_source() -> Weight;
    fn disable_liquidity_source() -> Weight;
    fn swap_transfer_batch(n: u32, m: u32) -> Weight;
}

impl<T: Config> Pallet<T> {
    /// Temporary workaround to prevent tbc oracle exploit with xyk-only filter.
    pub fn is_forbidden_filter(
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        selected_source_types: &Vec<LiquiditySourceType>,
        filter_mode: &FilterMode,
    ) -> bool {
        let tbc_reserve_assets = T::PrimaryMarketTBC::enabled_target_assets();
        // check if user has selected only xyk either explicitly or by excluding other types
        // FIXME: such detection approach is unreliable, come up with better way
        let is_xyk_only = selected_source_types.contains(&LiquiditySourceType::XYKPool)
            && !selected_source_types
                .contains(&LiquiditySourceType::MulticollateralBondingCurvePool)
            && !selected_source_types.contains(&LiquiditySourceType::XSTPool)
            && filter_mode == &FilterMode::AllowSelected
            || selected_source_types
                .contains(&LiquiditySourceType::MulticollateralBondingCurvePool)
                && selected_source_types.contains(&LiquiditySourceType::XSTPool)
                && !selected_source_types.contains(&LiquiditySourceType::XYKPool)
                && filter_mode == &FilterMode::ForbidSelected;
        // check if either of tbc reserve assets is present
        let reserve_asset_present = tbc_reserve_assets.contains(input_asset_id)
            || tbc_reserve_assets.contains(output_asset_id);

        is_xyk_only && reserve_asset_present
    }

    pub fn inner_swap(
        sender: T::AccountId,
        receiver: T::AccountId,
        dex_id: T::DEXId,
        input_asset_id: T::AssetId,
        output_asset_id: T::AssetId,
        swap_amount: SwapAmount<Balance>,
        selected_source_types: Vec<LiquiditySourceType>,
        filter_mode: FilterMode,
    ) -> Result<(), DispatchError> {
        ensure!(
            assets::AssetInfos::<T>::get(input_asset_id).2 != 0
                && assets::AssetInfos::<T>::get(output_asset_id).2 != 0,
            Error::<T>::UnableToSwapIndivisibleAssets
        );

        if Self::is_forbidden_filter(
            &input_asset_id,
            &output_asset_id,
            &selected_source_types,
            &filter_mode,
        ) {
            fail!(Error::<T>::ForbiddenFilter);
        }

        let (outcome, sources) = Self::inner_exchange(
            dex_id,
            &sender,
            &receiver,
            &input_asset_id,
            &output_asset_id,
            swap_amount,
            LiquiditySourceFilter::with_mode(dex_id, filter_mode, selected_source_types),
        )?;

        let (input_amount, output_amount, fee_amount) = match swap_amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in, ..
            } => (desired_amount_in, outcome.amount, outcome.fee),
            SwapAmount::WithDesiredOutput {
                desired_amount_out, ..
            } => (outcome.amount, desired_amount_out, outcome.fee),
        };
        Self::deposit_event(Event::<T>::Exchange(
            sender,
            dex_id,
            input_asset_id,
            output_asset_id,
            input_amount,
            output_amount,
            fee_amount,
            sources,
        ));

        Ok(().into())
    }

    /// Applies trivial routing (via Base Asset), resulting in a poly-swap which may contain several individual swaps.
    /// Those individual swaps are subject to liquidity aggregation algorithm.
    ///
    /// This a wrapper for `exchange_single`.
    pub fn inner_exchange(
        dex_id: T::DEXId,
        sender: &T::AccountId,
        receiver: &T::AccountId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: SwapAmount<Balance>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<(SwapOutcome<Balance>, Vec<LiquiditySourceIdOf<T>>), DispatchError> {
        ensure!(
            input_asset_id != output_asset_id,
            Error::<T>::UnavailableExchangePath
        );

        common::with_transaction(|| {
            let dex_info = dex_manager::Pallet::<T>::get_dex_info(&dex_id)?;
            let maybe_path =
                ExchangePath::<T>::new_trivial(&dex_info, *input_asset_id, *output_asset_id);
            maybe_path.map_or(Err(Error::<T>::UnavailableExchangePath.into()), |paths| {
                Self::exchange_sequence(&dex_info, sender, receiver, paths, amount, &filter)
            })
        })
    }

    /// Exchange sequence of assets, where each pair is a direct exchange.
    /// The swaps path is selected via `select_best_path`
    fn exchange_sequence(
        dex_info: &DEXInfo<T::AssetId>,
        sender: &T::AccountId,
        receiver: &T::AccountId,
        asset_paths: Vec<ExchangePath<T>>,
        amount: SwapAmount<Balance>,
        filter: &LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<(SwapOutcome<Balance>, Vec<LiquiditySourceIdOf<T>>), DispatchError> {
        match amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => {
                let best_path = Self::select_best_path(
                    dex_info,
                    asset_paths,
                    Ordering::Greater,
                    desired_amount_in,
                    filter,
                    true,
                    true,
                )
                .map(|info| info.path)?;
                Self::exchange_sequence_with_input_amount(
                    dex_info,
                    sender,
                    receiver,
                    &best_path,
                    desired_amount_in,
                    filter,
                )
                .and_then(|(swap, sources)| {
                    ensure!(
                        swap.amount >= min_amount_out,
                        Error::<T>::SlippageNotTolerated
                    );
                    Ok((swap, sources))
                })
            }
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => {
                let best_path = Self::select_best_path(
                    dex_info,
                    asset_paths,
                    Ordering::Less,
                    desired_amount_out,
                    filter,
                    true,
                    true,
                )
                .map(|info| info.path)?;
                let input_amount =
                    Self::calculate_input_amount(dex_info, &best_path, desired_amount_out, filter)?;
                ensure!(
                    input_amount <= max_amount_in,
                    Error::<T>::SlippageNotTolerated
                );

                Self::exchange_sequence_with_input_amount(
                    dex_info,
                    sender,
                    receiver,
                    &best_path,
                    input_amount,
                    filter,
                )
                .and_then(|(mut swap, sources)| {
                    swap.amount = input_amount;
                    Ok((swap, sources))
                })
            }
        }
    }

    /// Exchange sequence of assets using input amount.
    ///
    /// Performs [`Self::exchange_single()`] for each pair of assets and aggregates the results.
    fn exchange_sequence_with_input_amount(
        dex_info: &DEXInfo<T::AssetId>,
        sender: &T::AccountId,
        receiver: &T::AccountId,
        assets: &[T::AssetId],
        input_amount: Balance,
        filter: &LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<(SwapOutcome<Balance>, Vec<LiquiditySourceIdOf<T>>), DispatchError> {
        use itertools::EitherOrBoth::*;

        let transit_account = T::GetTechnicalAccountId::get();
        let exchange_count = assets.len() - 1;

        let sender_iter = sp_std::iter::once(sender)
            .chain(sp_std::iter::repeat(&transit_account).take(exchange_count - 1));
        let receiver_iter = sp_std::iter::repeat(&transit_account)
            .take(exchange_count - 1)
            .chain(sp_std::iter::once(receiver));
        let mut current_amount = input_amount;

        fallible_iterator::convert(
            assets
                .iter()
                .tuple_windows()
                .zip_longest(sender_iter)
                .zip_longest(receiver_iter)
                .map(|zip| match zip {
                    Both(Both((from, to), cur_sender), cur_receiver) => {
                        (from, to, cur_sender, cur_receiver)
                    }
                    // Sanity check. Should never happen
                    _ => panic!(
                        "Exchanging failed, iterator invariants are broken - \
                         this is a programmer error"
                    ),
                })
                // Exchange
                .map(
                    |(from, to, cur_sender, cur_receiver)| -> Result<_, DispatchError> {
                        let swap_amount =
                            SwapAmount::with_desired_input(current_amount, Balance::zero());

                        let (swap_outcome, sources) = Self::exchange_single(
                            cur_sender,
                            cur_receiver,
                            &dex_info.base_asset_id,
                            from,
                            to,
                            swap_amount,
                            filter.clone(),
                        )?;

                        current_amount = swap_outcome.amount;
                        Ok((swap_outcome, sources))
                    },
                ),
        )
        // Exchange aggregation
        .fold(
            (SwapOutcome::new(balance!(0), balance!(0)), Vec::new()),
            |(mut outcome, mut sources), (swap_outcome, swap_sources)| {
                outcome.amount = swap_outcome.amount;
                outcome.fee = swap_outcome
                    .fee
                    .checked_add(swap_outcome.fee)
                    .ok_or(Error::<T>::CalculationError)?;
                merge_two_vectors_unique(&mut sources, swap_sources);
                Ok((outcome, sources))
            },
        )
    }

    /// Calculate the input amount for a given `output_amount` for a sequence of direct swaps.
    fn calculate_input_amount(
        dex_info: &DEXInfo<T::AssetId>,
        assets: &[T::AssetId],
        output_amount: Balance,
        filter: &LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<Balance, DispatchError> {
        let mut amount = output_amount;

        assets
            .iter()
            .rev()
            .tuple_windows()
            .map(|(to, from)| (from, to)) // Need to reverse pairs as well
            .map(|(from, to)| -> Result<_, DispatchError> {
                let (quote, _, _) = Self::quote_single(
                    &dex_info.base_asset_id,
                    &from,
                    &to,
                    QuoteAmount::with_desired_output(amount),
                    filter.clone(),
                    true,
                    true,
                )?;
                amount = quote.amount;
                Ok(())
            })
            .for_each(drop);
        Ok(amount)
    }

    /// Performs a swap given a number of liquidity sources and a distribution of the swap amount across the sources.
    fn exchange_single(
        sender: &T::AccountId,
        receiver: &T::AccountId,
        base_asset_id: &T::AssetId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: SwapAmount<Balance>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<(SwapOutcome<Balance>, Vec<LiquiditySourceIdOf<T>>), DispatchError> {
        common::with_transaction(|| {
            let (outcome, _, sources) = Self::quote_single(
                base_asset_id,
                input_asset_id,
                output_asset_id,
                amount.into(),
                filter,
                true,
                true,
            )?;

            let res = outcome
                .distribution
                .into_iter()
                .filter(|(_src, part_amount)| part_amount.amount() > balance!(0))
                .map(|(src, part_amount)| {
                    let part_amount = part_amount.amount();
                    let part_limit = (FixedWrapper::from(part_amount) / amount.amount()
                        * amount.limit())
                    .try_into_balance()
                    .map_err(|_| Error::CalculationError::<T>)?;
                    T::LiquidityRegistry::exchange(
                        sender,
                        receiver,
                        &src,
                        input_asset_id,
                        output_asset_id,
                        amount.copy_direction(part_amount, part_limit),
                    )
                })
                .collect::<Result<Vec<SwapOutcome<Balance>>, DispatchError>>()?;

            let (amount, fee): (FixedWrapper, FixedWrapper) = res.into_iter().fold(
                (fixed_wrapper!(0), fixed_wrapper!(0)),
                |(amount_acc, fee_acc), x| {
                    (
                        amount_acc + FixedWrapper::from(x.amount),
                        fee_acc + FixedWrapper::from(x.fee),
                    )
                },
            );
            let amount = amount
                .try_into_balance()
                .map_err(|_| Error::CalculationError::<T>)?;
            let fee = fee
                .try_into_balance()
                .map_err(|_| Error::CalculationError::<T>)?;

            Ok((SwapOutcome::new(amount, fee), sources))
        })
    }

    /// Applies trivial routing (via Base Asset), resulting in a poly-swap which may contain several individual swaps.
    /// Those individual swaps are subject to liquidity aggregation algorithm.
    ///
    /// This a wrapper for `quote_single`.
    pub fn inner_quote(
        dex_id: T::DEXId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
        skip_info: bool,
        deduce_fee: bool,
    ) -> Result<QuoteInfo<T::AssetId, LiquiditySourceIdOf<T>>, DispatchError> {
        ensure!(
            input_asset_id != output_asset_id,
            Error::<T>::UnavailableExchangePath
        );
        let dex_info = dex_manager::Pallet::<T>::get_dex_info(&dex_id)?;
        let maybe_path =
            ExchangePath::<T>::new_trivial(&dex_info, *input_asset_id, *output_asset_id);
        maybe_path.map_or_else(
            || Err(Error::<T>::UnavailableExchangePath.into()),
            |paths| Self::quote_sequence(&dex_info, paths, amount, &filter, skip_info, deduce_fee),
        )
    }

    /// Quote sequence of assets, where each pair is a direct exchange.
    /// Selects swaps path via `select_best_path`
    fn quote_sequence(
        dex_info: &DEXInfo<T::AssetId>,
        asset_paths: Vec<ExchangePath<T>>,
        amount: QuoteAmount<Balance>,
        filter: &LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
        skip_info: bool,
        deduce_fee: bool,
    ) -> Result<QuoteInfo<T::AssetId, LiquiditySourceIdOf<T>>, DispatchError> {
        match amount {
            QuoteAmount::WithDesiredInput { desired_amount_in } => Self::select_best_path(
                dex_info,
                asset_paths,
                Ordering::Greater,
                desired_amount_in,
                filter,
                skip_info,
                deduce_fee,
            ),
            QuoteAmount::WithDesiredOutput { desired_amount_out } => Self::select_best_path(
                dex_info,
                asset_paths,
                Ordering::Less,
                desired_amount_out,
                filter,
                skip_info,
                deduce_fee,
            ),
        }
    }

    /// Selects the best path between two swap paths
    /// `ord` parameter influences the preprocessing before
    /// calling `quote_pairs_with_flexible_amount`. The Ordering:Greater variant
    /// is related to `QuoteAmount::WithDesiredInput` and other ordering variants are related to
    /// `QuoteAmount::WithDesiredOutput`
    ///
    /// Returns Result containing a quote result and the selected path
    fn select_best_path(
        dex_info: &DEXInfo<T::AssetId>,
        asset_paths: Vec<ExchangePath<T>>,
        ord: Ordering,
        amount: Balance,
        filter: &LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
        skip_info: bool,
        deduce_fee: bool,
    ) -> Result<QuoteInfo<T::AssetId, LiquiditySourceIdOf<T>>, DispatchError> {
        let mut path_quote_iter = asset_paths.into_iter().map(|ExchangePath(atomic_path)| {
            let quote = match ord {
                Ordering::Greater => Self::quote_pairs_with_flexible_amount(
                    dex_info,
                    atomic_path.iter().tuple_windows(),
                    QuoteAmount::with_desired_input,
                    amount,
                    filter,
                    skip_info,
                    deduce_fee,
                ),
                _ => Self::quote_pairs_with_flexible_amount(
                    dex_info,
                    atomic_path
                        .iter()
                        .rev()
                        .tuple_windows()
                        .map(|(to, from)| (from, to)),
                    QuoteAmount::with_desired_output,
                    amount,
                    filter,
                    skip_info,
                    deduce_fee,
                ),
            };
            quote.map(|x| QuoteInfo {
                outcome: x.0,
                amount_without_impact: x.1,
                rewards: x.2,
                liquidity_sources: x.3,
                path: atomic_path,
            })
        });

        let primary_path = path_quote_iter
            .next()
            .ok_or(Error::<T>::UnavailableExchangePath)?;

        path_quote_iter.fold(primary_path, |acc, path| match (&acc, &path) {
            (Ok(_), Err(_)) => acc,
            (Err(_), Ok(_)) => path,
            (Ok(acc_quote_info), Ok(quote_info)) => {
                match (ord, acc_quote_info.outcome.cmp(&quote_info.outcome)) {
                    (Ordering::Greater, Ordering::Less) => path,
                    (Ordering::Greater, _) => acc,
                    (_, Ordering::Less) => acc,
                    _ => path,
                }
            }
            _ => acc,
        })
    }

    /// Quote given pairs of assets using `amount_ctr` to construct [`QuoteAmount`] for each pair.
    ///
    /// Performs [`Self::quote_single()`] for each pair and aggregates the results.
    fn quote_pairs_with_flexible_amount<'asset, F: Fn(Balance) -> QuoteAmount<Balance>>(
        dex_info: &DEXInfo<T::AssetId>,
        asset_pairs: impl Iterator<Item = (&'asset T::AssetId, &'asset T::AssetId)>,
        amount_ctr: F,
        amount: Balance,
        filter: &LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
        skip_info: bool,
        deduce_fee: bool,
    ) -> Result<
        (
            SwapOutcome<Balance>,
            Option<Balance>,
            Rewards<T::AssetId>,
            Vec<LiquiditySourceIdOf<T>>,
        ),
        DispatchError,
    > {
        let mut current_amount = amount;
        let init_outcome_without_impact = (!skip_info).then(|| balance!(0));
        fallible_iterator::convert(asset_pairs.map(|(from_asset_id, to_asset_id)| {
            let (quote, rewards, liquidity_sources) = Self::quote_single(
                &dex_info.base_asset_id,
                from_asset_id,
                to_asset_id,
                amount_ctr(current_amount),
                filter.clone(),
                skip_info,
                deduce_fee,
            )?;
            current_amount = quote.amount;
            Ok((quote, rewards, liquidity_sources, (from_asset_id, to_asset_id)))
        }))
        .fold(
            (
                SwapOutcome::new(balance!(0), balance!(0)),
                init_outcome_without_impact,
                Rewards::new(),
                Vec::new(),
            ),
            |(
                mut outcome,
                mut outcome_without_impact,
                mut rewards,
                mut liquidity_sources,
            ),
             (quote, mut quote_rewards, quote_liquidity_sources, (from_asset, to_asset))| {
                outcome_without_impact = outcome_without_impact.map(|without_impact| {
                    Self::calculate_amount_without_impact(
                        from_asset,
                        to_asset,
                        &quote.distribution,
                        outcome.amount,
                        without_impact,
                        deduce_fee,
                    )
                })
                    .transpose()?;
                outcome.amount = quote.amount;
                outcome.fee = outcome
                    .fee
                    .checked_add(quote.fee)
                    .ok_or(Error::<T>::CalculationError)?;
                rewards.append(&mut quote_rewards);
                merge_two_vectors_unique(&mut liquidity_sources, quote_liquidity_sources);
                Ok((
                    outcome,
                    outcome_without_impact,
                    rewards,
                    liquidity_sources,
                ))
            },
        )
    }

    // Would likely to fail if operating near the limits,
    // because it uses i128 for fixed-point arithmetics.
    // TODO: switch to unsigned internal representation
    fn calculate_amount_without_impact(
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        distribution: &Vec<(
            LiquiditySourceId<T::DEXId, LiquiditySourceType>,
            QuoteAmount<Balance>,
        )>,
        outcome_amount: u128,
        outcome_without_impact: u128,
        deduce_fee: bool,
    ) -> Result<Balance, DispatchError> {
        use common::fixnum;
        use fixnum::ops::{One, RoundMode, RoundingDiv, RoundingMul};

        let ratio_to_actual = if outcome_amount != 0 {
            // TODO: switch to unsigned internal representation (`FixedPoint<u128, U18>`)
            // for now lib `fixnum` doesn't implement operations for such types, so
            // we just use `i128` repr
            let outcome_without_impact = Fixed::from_bits(
                outcome_without_impact
                    .try_into()
                    .map_err(|_| Error::<T>::FailedToCalculatePriceWithoutImpact)?,
            );
            let outcome_amount = Fixed::from_bits(
                outcome_amount
                    .try_into()
                    .map_err(|_| Error::<T>::FailedToCalculatePriceWithoutImpact)?,
            );
            // Same RoundMode as was used in frontend
            outcome_without_impact
                .rdiv(outcome_amount, RoundMode::Floor)
                .unwrap_or(Fixed::ONE)
        } else {
            <Fixed as One>::ONE
        };

        // multiply all amounts in distribution to adjust prev quote without impact:
        let distribution = distribution
            .into_iter()
            .filter(|(_, part_amount)| part_amount.amount() > balance!(0))
            .map(|(market, amount)| {
                // Should not overflow unless the amounts are comparable to 10^38 .
                // For reference, a trillion is 10^12.
                //
                // same as mul by ratioToActual, just without floating point ops
                let adjusted_amount: u128 = Fixed::from_bits(
                    amount
                        .amount()
                        .try_into()
                        .map_err(|_| Error::<T>::FailedToCalculatePriceWithoutImpact)?,
                )
                .rmul(ratio_to_actual, RoundMode::Floor)
                .map_err(|_| Error::<T>::FailedToCalculatePriceWithoutImpact)?
                .into_bits()
                .try_into()
                .map_err(|_| Error::<T>::FailedToCalculatePriceWithoutImpact)?;
                Ok::<_, Error<T>>((market, amount.copy_direction(adjusted_amount)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut accumulated_without_impact: Balance = 0;
        for (src, part_amount) in distribution.into_iter() {
            let part_outcome = T::LiquidityRegistry::quote_without_impact(
                src,
                input_asset_id,
                output_asset_id,
                part_amount,
                deduce_fee,
            )?;
            accumulated_without_impact = accumulated_without_impact
                .checked_add(part_outcome.amount)
                .ok_or(Error::<T>::FailedToCalculatePriceWithoutImpact)?;
        }
        Ok(accumulated_without_impact)
    }

    /// Computes the optimal distribution across available liquidity sources to execute the requested trade
    /// given the input and output assets, the trade amount and a liquidity sources filter.
    ///
    /// - `input_asset_id` - ID of the asset to sell,
    /// - `output_asset_id` - ID of the asset to buy,
    /// - `amount` - the amount with "direction" (sell or buy) together with the maximum price impact (slippage),
    /// - `filter` - a filter composed of a list of liquidity sources IDs to accept or ban for this trade.
    /// - `skip_info` - flag that indicates that additional info should not be shown, that is needed when actual exchange is performed.
    ///
    fn quote_single(
        base_asset_id: &T::AssetId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
        skip_info: bool,
        deduce_fee: bool,
    ) -> Result<
        (
            AggregatedSwapOutcome<LiquiditySourceIdOf<T>, Balance>,
            Rewards<T::AssetId>,
            Vec<LiquiditySourceIdOf<T>>,
        ),
        DispatchError,
    > {
        let mut sources =
            T::LiquidityRegistry::list_liquidity_sources(input_asset_id, output_asset_id, filter)?;
        let locked = trading_pair::LockedLiquiditySources::<T>::get();
        sources.retain(|x| !locked.contains(&x.liquidity_source_index));
        ensure!(!sources.is_empty(), Error::<T>::UnavailableExchangePath);

        // Check if we have exactly one source => no split required
        if sources.len() == 1 {
            let src = sources.first().unwrap();
            let outcome = T::LiquidityRegistry::quote(
                src,
                input_asset_id,
                output_asset_id,
                amount.into(),
                deduce_fee,
            )?;
            let rewards = if skip_info {
                Vec::new()
            } else {
                let (input_amount, output_amount) = amount.place_input_and_output(outcome.clone());
                T::LiquidityRegistry::check_rewards(
                    src,
                    input_asset_id,
                    output_asset_id,
                    input_amount,
                    output_amount,
                )
                .unwrap_or(Vec::new())
            };
            return Ok((
                AggregatedSwapOutcome::new(
                    vec![(src.clone(), amount)],
                    outcome.amount,
                    outcome.fee,
                ),
                rewards,
                sources,
            ));
        }

        // Check if we have exactly two sources: the primary market and the secondary market
        // Do the "smart" swap split (with fallback)
        // NOTE: we assume here that XST tokens are not added to TBC reserves. If they are in the future, this
        // logic should be redone!
        if sources.len() == 2 {
            let mut primary_market: Option<LiquiditySourceIdOf<T>> = None;
            let mut secondary_market: Option<LiquiditySourceIdOf<T>> = None;

            for src in &sources {
                match src.liquidity_source_index {
                    // We can't use XST as primary market for smart split, because it use XST asset as base
                    // and does not support DEXes except Polkaswap
                    LiquiditySourceType::MulticollateralBondingCurvePool => {
                        primary_market = Some(src.clone())
                    }
                    LiquiditySourceType::XYKPool | LiquiditySourceType::MockPool => {
                        secondary_market = Some(src.clone())
                    }
                    _ => (),
                }
            }

            if let (Some(primary_mkt), Some(xyk)) = (primary_market, secondary_market) {
                let outcome = Self::smart_split(
                    &primary_mkt,
                    &xyk,
                    base_asset_id,
                    input_asset_id,
                    output_asset_id,
                    amount.clone(),
                    skip_info,
                    deduce_fee,
                )?;
                return Ok((outcome.0, outcome.1, sources));
            }
        }

        fail!(Error::<T>::UnavailableExchangePath);
    }

    /// Check if given two arbitrary tokens can be used to perform an exchange via any available sources.
    pub fn is_path_available(
        dex_id: T::DEXId,
        input_asset_id: T::AssetId,
        output_asset_id: T::AssetId,
    ) -> Result<bool, DispatchError> {
        let dex_info = dex_manager::Pallet::<T>::get_dex_info(&dex_id)?;
        let maybe_path = ExchangePath::<T>::new_trivial(&dex_info, input_asset_id, output_asset_id);
        maybe_path.map_or(Ok(false), |paths| {
            let paths_flag = paths
                .into_iter()
                .map(|ExchangePath(atomic_path)| {
                    Self::check_asset_path(&dex_id, &dex_info, &atomic_path)
                })
                .any(|x| x);
            Ok(paths_flag)
        })
    }

    /// Checks if the path, consisting of sequential swaps of assets in `path`, is
    /// available and if it is, then returns Ok(true)
    pub fn check_asset_path(
        dex_id: &T::DEXId,
        dex_info: &DEXInfo<T::AssetId>,
        path: &[T::AssetId],
    ) -> bool {
        path.iter()
            .tuple_windows()
            .filter_map(|(from, to)| {
                let pair = Self::weak_sort_pair(&dex_info, *from, *to);
                trading_pair::Pallet::<T>::list_enabled_sources_for_trading_pair(
                    dex_id,
                    &pair.base_asset_id,
                    &pair.target_asset_id,
                )
                .ok()
            })
            .all(|sources| !sources.is_empty())
    }

    /// Returns a BTreeSet with all LiquiditySourceTypes, which will be used for swap
    pub fn get_asset_path_sources(
        dex_id: &T::DEXId,
        dex_info: &DEXInfo<T::AssetId>,
        path: &[T::AssetId],
    ) -> Result<BTreeSet<LiquiditySourceType>, DispatchError> {
        let sources_set = fallible_iterator::convert(path.to_vec().iter().tuple_windows().map(
            |(from, to)| -> Result<_, DispatchError> {
                let pair = Self::weak_sort_pair(&dex_info, *from, *to);
                let sources = trading_pair::Pallet::<T>::list_enabled_sources_for_trading_pair(
                    &dex_id,
                    &pair.base_asset_id,
                    &pair.target_asset_id,
                )?;
                ensure!(!sources.is_empty(), Error::<T>::UnavailableExchangePath);
                Ok(sources)
            },
        ))
        .fold(None, |acc: Option<BTreeSet<_>>, sources| match acc {
            Some(mut set) => {
                set.retain(|x| sources.contains(x));
                Ok(Some(set))
            }
            None => Ok(Some(sources)),
        })?
        .unwrap_or_default();
        Ok(sources_set)
    }

    /// Given two arbitrary tokens return sources that can be used to cover full path.
    /// If there are two possible swap paths, then returns a union of used liquidity sources
    pub fn list_enabled_sources_for_path(
        dex_id: T::DEXId,
        input_asset_id: T::AssetId,
        output_asset_id: T::AssetId,
    ) -> Result<Vec<LiquiditySourceType>, DispatchError> {
        let dex_info = dex_manager::Pallet::<T>::get_dex_info(&dex_id)?;
        let maybe_path = ExchangePath::<T>::new_trivial(&dex_info, input_asset_id, output_asset_id);
        maybe_path.map_or_else(
            || Err(Error::<T>::UnavailableExchangePath.into()),
            |paths| {
                let mut paths_sources_iter = paths.into_iter().map(|ExchangePath(atomic_path)| {
                    Self::get_asset_path_sources(&dex_id, &dex_info, &atomic_path)
                });

                let primary_set: Result<BTreeSet<LiquiditySourceType>, DispatchError> =
                    paths_sources_iter
                        .next()
                        .ok_or(Error::<T>::UnavailableExchangePath)?;

                paths_sources_iter
                    .fold(primary_set, |acc: Result<_, DispatchError>, set| {
                        match (acc, set) {
                            (Ok(acc_unwrapped), Err(_)) => Ok(acc_unwrapped),
                            (Err(_), Ok(set_unwrapped)) => Ok(set_unwrapped),
                            (Ok(mut acc_unwrapped), Ok(mut set_unwrapped)) => {
                                acc_unwrapped.append(&mut set_unwrapped);
                                Ok(acc_unwrapped)
                            }
                            (Err(e), _) => Err(e),
                        }
                    })
                    .map(|set| Vec::from_iter(set.into_iter()))
            },
        )
    }

    pub fn list_enabled_sources_for_path_with_xyk_forbidden(
        dex_id: T::DEXId,
        input_asset_id: T::AssetId,
        output_asset_id: T::AssetId,
    ) -> Result<Vec<LiquiditySourceType>, DispatchError> {
        let tbc_reserve_assets = T::PrimaryMarketTBC::enabled_target_assets();
        let mut initial_result =
            Self::list_enabled_sources_for_path(dex_id, input_asset_id, output_asset_id)?;
        if tbc_reserve_assets.contains(&input_asset_id)
            || tbc_reserve_assets.contains(&output_asset_id)
        {
            initial_result.retain(|&lst| lst != LiquiditySourceType::XYKPool);
        }
        Ok(initial_result)
    }

    // Not full sort, just ensure that if there is base asset then it's sorted, otherwise order is unchanged.
    fn weak_sort_pair(
        dex_info: &DEXInfo<T::AssetId>,
        asset_a: T::AssetId,
        asset_b: T::AssetId,
    ) -> TradingPair<T::AssetId> {
        use AssetType::*;

        let synthetic_assets = T::PrimaryMarketXST::enabled_target_assets();
        let a_type = AssetType::determine::<T>(dex_info, &synthetic_assets, asset_a);
        let b_type = AssetType::determine::<T>(dex_info, &synthetic_assets, asset_b);

        match (a_type, b_type) {
            (Base, _) => TradingPair {
                base_asset_id: asset_a,
                target_asset_id: asset_b,
            },
            (_, Base) => TradingPair {
                base_asset_id: asset_b,
                target_asset_id: asset_a,
            },
            (SyntheticBase, _) => TradingPair {
                base_asset_id: asset_a,
                target_asset_id: asset_b,
            },
            (_, SyntheticBase) => TradingPair {
                base_asset_id: asset_b,
                target_asset_id: asset_a,
            },
            (_, _) => TradingPair {
                base_asset_id: asset_a,
                target_asset_id: asset_b,
            },
        }
    }

    /// Implements the "smart" split algorithm.
    ///
    /// - `primary_source_id` - ID of the primary market liquidity source,
    /// - `secondary_source_id` - ID of the secondary market liquidity source,
    /// - `input_asset_id` - ID of the asset to sell,
    /// - `output_asset_id` - ID of the asset to buy,
    /// - `amount` - the amount with "direction" (sell or buy) together with the maximum price impact (slippage).
    /// - `skip_info` - flag that indicates that additional info should not be shown, that is needed when actual exchange is performed.
    ///
    fn smart_split(
        primary_source_id: &LiquiditySourceIdOf<T>,
        secondary_source_id: &LiquiditySourceIdOf<T>,
        base_asset_id: &T::AssetId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        skip_info: bool,
        deduce_fee: bool,
    ) -> Result<
        (
            AggregatedSwapOutcome<LiquiditySourceIdOf<T>, Balance>,
            Rewards<T::AssetId>,
        ),
        DispatchError,
    > {
        // The "smart" split algo is based on the following reasoning.
        // First, we try to calculate the spot price of the `input_asset_id` in both
        // the primary and secondary markets. If the price in the secondary market is
        // better than that in the primary market, we allocate as much of the `amount` to
        // be swapped in the secondary market as we can until the prices level up.
        // The rest will be swapped in the primary market.
        //
        // In case the default partitioning between sources returns an error, it can
        // only be due to the MCBC pool not being available or initialized.
        // In this case the primary market weight is zeroed out and the entire `amount`
        // is sent to the secondary market (regardless whether the latter has enough
        // reserves to actually execute such swap).
        //
        // In case the "smart" procedure has returned some weights (a, b), such that
        // a > 0, b > 0, a + b == 1.0, and neither of the arms fails due to insufficient
        // reserves, we must still account for the fact that the algorithm tends to overweigh
        // the MCBC share which can lead to substantially non-optimal results
        // (especially when selling XOR to the MCBC).
        // To limit the impact of this imbalance we want to always compare the result of
        // the "smart" split with the purely secondary market one.
        // Comparing the result with the purely MCBC swap doesn't make sense in this case
        // because the "smart" swap is always at least as good as the 100% MCBC one.

        ensure!(
            input_asset_id == base_asset_id || output_asset_id == base_asset_id,
            Error::<T>::UnavailableExchangePath
        );
        let other_asset = if base_asset_id == input_asset_id {
            output_asset_id
        } else {
            input_asset_id
        };

        let (reserves_base, reserves_other) =
            T::SecondaryMarket::reserves(base_asset_id, other_asset);

        let amount_primary = if output_asset_id == base_asset_id {
            // XOR is being bought
            Self::decide_primary_market_amount_buying_base_asset(
                base_asset_id,
                other_asset,
                amount.clone(),
                (reserves_base, reserves_other),
            )
            .unwrap_or(
                // Error can only be due to MCBC or XST pool, hence zeroing it out
                amount.copy_direction(balance!(0)),
            )
        } else {
            // XOR is being sold
            Self::decide_primary_market_amount_selling_base_asset(
                base_asset_id,
                other_asset,
                amount.clone(),
                (reserves_base, reserves_other),
            )
            .unwrap_or(amount.copy_direction(balance!(0)))
        };

        let (is_better, extremum): (fn(a: Balance, b: Balance) -> bool, Balance) = match amount {
            QuoteAmount::WithDesiredInput { .. } => (|a, b| a > b, Balance::zero()),
            _ => (|a, b| a < b, Balance::MAX),
        };

        let mut best: Balance = extremum;
        let mut total_fee: Balance = 0;
        let mut rewards = Vec::new();
        let mut distr = Vec::new();
        let mut maybe_error: Option<DispatchError> = None;

        if amount_primary.amount() > Balance::zero() {
            // Attempting to quote according to the default sources weights
            let intermediary_result = T::LiquidityRegistry::quote(
                primary_source_id,
                input_asset_id,
                output_asset_id,
                amount_primary.clone(),
                deduce_fee,
            )
            .and_then(|outcome_primary| {
                if amount_primary.amount() < amount.amount() {
                    let amount_secondary = amount
                        .checked_sub(&amount_primary)
                        .ok_or(Error::<T>::CalculationError)?;
                    T::LiquidityRegistry::quote(
                        secondary_source_id,
                        input_asset_id,
                        output_asset_id,
                        amount_secondary.clone(),
                        deduce_fee,
                    )
                    .and_then(|outcome_secondary| {
                        if !skip_info {
                            for info in vec![
                                (primary_source_id, amount_primary, outcome_primary.clone()),
                                (
                                    secondary_source_id,
                                    amount_secondary,
                                    outcome_secondary.clone(),
                                ),
                            ] {
                                let (input_amount, output_amount) =
                                    info.1.place_input_and_output(info.2);
                                rewards.append(
                                    &mut T::LiquidityRegistry::check_rewards(
                                        info.0,
                                        input_asset_id,
                                        output_asset_id,
                                        input_amount,
                                        output_amount,
                                    )
                                    .unwrap_or(Vec::new()),
                                );
                            }
                        };
                        best = outcome_primary.amount + outcome_secondary.amount;
                        total_fee = outcome_primary.fee + outcome_secondary.fee;
                        distr = vec![
                            (primary_source_id.clone(), amount_primary),
                            (secondary_source_id.clone(), amount_secondary),
                        ];
                        Ok(())
                    })
                } else {
                    best = outcome_primary.amount;
                    total_fee = outcome_primary.fee;
                    distr = vec![(primary_source_id.clone(), amount_primary)];
                    Ok(())
                }
            });
            if let Err(e) = intermediary_result {
                maybe_error = Some(e);
            }
        }

        // Regardless whether we have got any result so far, we still must do
        // calculations for the secondary market alone
        let xyk_result = T::LiquidityRegistry::quote(
            secondary_source_id,
            input_asset_id,
            output_asset_id,
            amount.clone(),
            deduce_fee,
        )
        .and_then(|outcome| {
            if is_better(outcome.amount, best) {
                best = outcome.amount;
                total_fee = outcome.fee;
                distr = vec![(secondary_source_id.clone(), amount.clone())];
                if !skip_info {
                    let (input_amount, output_amount) =
                        amount.place_input_and_output(outcome.clone());
                    rewards = T::LiquidityRegistry::check_rewards(
                        secondary_source_id,
                        input_asset_id,
                        output_asset_id,
                        input_amount,
                        output_amount,
                    )
                    .unwrap_or(Vec::new());
                };
            };
            Ok(())
        });

        // Check if we have got a result at either of the steps
        if let Err(err) = xyk_result {
            // If both attempts to get the price failed, return the first error
            if let Some(e) = maybe_error {
                // Quote at the first step was attempted and failed
                return Err(e);
            }
            if best == extremum {
                // The quote at first step was never attempted, returning the current error
                return Err(err);
            }
        }

        Ok((AggregatedSwapOutcome::new(distr, best, total_fee), rewards))
    }

    /// Determines the share of a swap that should be exchanged in the primary market
    /// (i.e., the multi-collateral bonding curve pool) based on the current reserves of
    /// the base asset and the collateral asset in the secondary market (e.g., an XYK pool)
    /// provided the base asset is being bought.
    ///
    /// - `base_asset_id` - ID of the base asset,
    /// - `collateral_asset_id` - ID of the collateral asset,
    /// - `amount` - the swap amount with "direction" (fixed input vs fixed output),
    /// - `secondary_market_reserves` - a pair (base_reserve, collateral_reserve) in the secondary market
    ///
    fn decide_primary_market_amount_buying_base_asset(
        base_asset_id: &T::AssetId,
        collateral_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        secondary_market_reserves: (Balance, Balance),
    ) -> Result<QuoteAmount<Balance>, DispatchError> {
        let (reserves_base, reserves_other) = secondary_market_reserves;
        let x: FixedWrapper = reserves_base.into();
        let y: FixedWrapper = reserves_other.into();
        let k: FixedWrapper = x.clone() * y.clone();
        let secondary_price: FixedWrapper = if x > fixed_wrapper!(0) {
            y.clone() / x.clone()
        } else {
            Fixed::MAX.into()
        };

        macro_rules! match_buy_price {
            ($source_type:ident) => {
                T::$source_type::buy_price(base_asset_id, collateral_asset_id)
                    .map_err(|_| Error::<T>::CalculationError)?
                    .into()
            };
        }
        let primary_buy_price: FixedWrapper = if collateral_asset_id == &XSTUSD.into() {
            match_buy_price!(PrimaryMarketXST)
        } else {
            match_buy_price!(PrimaryMarketTBC)
        };

        match amount {
            QuoteAmount::WithDesiredInput { desired_amount_in } => {
                let wrapped_amount: FixedWrapper = desired_amount_in.into();
                // checking that secondary price is better than primary initially
                let amount_primary = if secondary_price < primary_buy_price {
                    // find intercept between secondary and primary market curves:
                    // 1) (x - x1) * (y + y1) = k // xyk equation
                    // 2) (y + y1) / (x - x1) = p // desired price `p` equation
                    // composing 1 and 2: (y + y1) * (y + y1) = k * p
                    // √(k * p) - y = y1
                    // √(k) * √(p) - y = y1 // to prevent overflow
                    // where
                    // * x is base reserve, x1 is base amount, y is target reserve, y1 is target amount
                    // * p is desired price i.e. target/base
                    let k_sqrt = k.sqrt_accurate();
                    let primary_buy_price_sqrt = primary_buy_price.sqrt_accurate();
                    let amount_secondary = k_sqrt * primary_buy_price_sqrt - y; // always > 0
                    if amount_secondary >= wrapped_amount {
                        balance!(0)
                    } else if amount_secondary <= fixed_wrapper!(0) {
                        desired_amount_in
                    } else {
                        (wrapped_amount - amount_secondary)
                            .try_into_balance()
                            .unwrap()
                    }
                } else {
                    desired_amount_in
                };
                Ok(QuoteAmount::with_desired_input(amount_primary))
            }
            QuoteAmount::WithDesiredOutput { desired_amount_out } => {
                let wrapped_amount: FixedWrapper = desired_amount_out.into();
                // checking that secondary price is better than primary initially
                let amount_primary = if secondary_price < primary_buy_price {
                    // find intercept between secondary and primary market curves:
                    // 1) (x - x1) * (y + y1) = k // xyk equation
                    // 2) (y + y1) / (x - x1) = p // desired price `p` equation
                    // composing 1 and 2: (x - x1) * (x - x1) * p = k
                    // x - √(k / p) = x1
                    // where
                    // * x is base reserve, x1 is base amount, y is target reserve, y1 is target amount
                    // * p is desired price i.e. target/base
                    let amount_secondary = x - (k / primary_buy_price).sqrt_accurate(); // always > 0
                    if amount_secondary >= wrapped_amount {
                        balance!(0)
                    } else if amount_secondary <= fixed_wrapper!(0) {
                        desired_amount_out
                    } else {
                        (wrapped_amount - amount_secondary)
                            .try_into_balance()
                            .unwrap()
                    }
                } else {
                    desired_amount_out
                };
                Ok(QuoteAmount::with_desired_output(amount_primary))
            }
        }
    }

    /// Determines the share of a swap that should be exchanged in the primary market
    /// (i.e. the multi-collateral bonding curve pool) based on the current reserves of
    /// the base asset and the collateral asset in the secondary market (e.g. an XYK pool)
    /// provided the base asset is being sold.
    ///
    /// - `base_asset_id` - ID of the base asset,
    /// - `collateral_asset_id` - ID of the collateral asset,
    /// - `amount` - the swap amount with "direction" (fixed input vs fixed output),
    /// - `secondary_market_reserves` - a pair (base_reserve, collateral_reserve) in the secondary market
    ///
    fn decide_primary_market_amount_selling_base_asset(
        base_asset_id: &T::AssetId,
        collateral_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        secondary_market_reserves: (Balance, Balance),
    ) -> Result<QuoteAmount<Balance>, DispatchError> {
        let (reserves_base, reserves_other) = secondary_market_reserves;
        let x: FixedWrapper = reserves_base.into();
        let y: FixedWrapper = reserves_other.into();
        let k: FixedWrapper = x.clone() * y.clone();
        let secondary_price: FixedWrapper = if x > fixed_wrapper!(0) {
            y.clone() / x.clone()
        } else {
            Fixed::ZERO.into()
        };

        macro_rules! match_sell_price {
            ($source_type:ident) => {
                T::$source_type::sell_price(base_asset_id, collateral_asset_id)
                    .map_err(|_| Error::<T>::CalculationError)?
                    .into()
            };
        }
        let primary_sell_price: FixedWrapper = if collateral_asset_id == &XSTUSD.into() {
            match_sell_price!(PrimaryMarketXST)
        } else {
            match_sell_price!(PrimaryMarketTBC)
        };

        match amount {
            QuoteAmount::WithDesiredInput { desired_amount_in } => {
                let wrapped_amount: FixedWrapper = desired_amount_in.into();
                // checking that secondary price is better than primary initially
                let amount_primary = if secondary_price > primary_sell_price {
                    // find intercept between secondary and primary market curves:
                    // 1) (x + x1) * (y - y1) = k // xyk equation
                    // 2) (y - y1) / (x + x1) = p // desired price `p` equation
                    // composing 1 and 2: (x + x1) * (x + x1) * p = k
                    // √(k / p) - x = x1
                    // where
                    // * x is base reserve, x1 is base amount, y is target reserve, y1 is target amount
                    // * p is desired price i.e. target/base
                    let amount_secondary = (k / primary_sell_price).sqrt_accurate() - x; // always > 0
                    if amount_secondary >= wrapped_amount {
                        balance!(0)
                    } else if amount_secondary <= fixed_wrapper!(0) {
                        desired_amount_in
                    } else {
                        (wrapped_amount - amount_secondary)
                            .try_into_balance()
                            .unwrap()
                    }
                } else {
                    desired_amount_in
                };
                Ok(QuoteAmount::with_desired_input(amount_primary))
            }
            QuoteAmount::WithDesiredOutput { desired_amount_out } => {
                let wrapped_amount: FixedWrapper = desired_amount_out.into();
                // checking that secondary price is better than primary initially
                let amount_primary = if secondary_price > primary_sell_price {
                    // find intercept between secondary and primary market curves:
                    // 1) (x + x1) * (y - y1) = k // xyk equation
                    // 2) (y - y1) / (x + x1) = p // desired price `p` equation
                    // composing 1 and 2: (y - y1) * (y - y1) = k * p
                    // y - √(k * p) = y1
                    // where
                    // * x is base reserve, x1 is base amount, y is target reserve, y1 is target amount
                    // * p is desired price i.e. target/base
                    let amount_secondary = y - (k * primary_sell_price).sqrt_accurate();
                    if amount_secondary >= wrapped_amount {
                        balance!(0)
                    } else if amount_secondary <= fixed_wrapper!(0) {
                        desired_amount_out
                    } else {
                        (wrapped_amount - amount_secondary)
                            .try_into_balance()
                            .unwrap()
                    }
                } else {
                    desired_amount_out
                };
                Ok(QuoteAmount::with_desired_output(amount_primary))
            }
        }
    }

    /// Swaps tokens for the following batch distribution and calculates a remainder.
    /// Remainder is used due to inaccuracy of the quote calculation.
    fn exchange_batch_tokens(
        sender: &T::AccountId,
        num_of_receivers: u128,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        max_input_amount: Balance,
        selected_source_types: &Vec<LiquiditySourceType>,
        dex_id: T::DEXId,
        filter_mode: &FilterMode,
        out_amount: Balance,
    ) -> Result<(Balance, Balance), DispatchError> {
        let filter = LiquiditySourceFilter::with_mode(
            dex_id,
            filter_mode.clone(),
            selected_source_types.clone(),
        );

        if Self::is_forbidden_filter(
            &input_asset_id,
            &output_asset_id,
            &selected_source_types,
            &filter_mode,
        ) {
            fail!(Error::<T>::ForbiddenFilter);
        }

        ensure!(
            assets::AssetInfos::<T>::get(input_asset_id).2 != 0
                && assets::AssetInfos::<T>::get(output_asset_id).2 != 0,
            Error::<T>::UnableToSwapIndivisibleAssets
        );

        let (
            SwapOutcome {
                amount: executed_input_amount,
                fee: fee_amount,
            },
            sources,
        ) = Self::inner_exchange(
            dex_id,
            &sender,
            &sender,
            &input_asset_id,
            &output_asset_id,
            SwapAmount::WithDesiredOutput {
                desired_amount_out: out_amount,
                max_amount_in: max_input_amount,
            },
            filter.clone(),
        )?;

        Self::deposit_event(Event::<T>::Exchange(
            sender.clone(),
            dex_id,
            input_asset_id.clone(),
            output_asset_id.clone(),
            executed_input_amount,
            out_amount,
            fee_amount,
            sources,
        ));

        let caller_output_asset_balance =
            assets::Pallet::<T>::total_balance(&output_asset_id, &sender)?;
        let remainder_per_receiver: Balance = if caller_output_asset_balance < out_amount {
            let remainder = out_amount.saturating_sub(caller_output_asset_balance);
            remainder / num_of_receivers + remainder % num_of_receivers
        } else {
            0
        };
        Ok((executed_input_amount, remainder_per_receiver))
    }

    fn transfer_batch_tokens_unchecked(
        sender: &T::AccountId,
        output_asset_id: &T::AssetId,
        receivers: Vec<BatchReceiverInfo<T::AccountId>>,
        remainder_per_receiver: Balance,
    ) -> Result<(), DispatchError> {
        fallible_iterator::convert(receivers.into_iter().map(|val| Ok(val))).for_each(|receiver| {
            assets::Pallet::<T>::transfer_from(
                &output_asset_id,
                &sender,
                &receiver.account_id,
                receiver.target_amount - remainder_per_receiver,
            )
        })
    }

    fn calculate_adar_commission(amount: Balance) -> Result<Balance, DispatchError> {
        let adar_commission_ratio = FixedWrapper::from(ADAR_COMMISSION_RATIO);

        let adar_commission = (FixedWrapper::from(amount) * adar_commission_ratio)
            .try_into_balance()
            .map_err(|_| Error::<T>::CalculationError)?;

        Ok(adar_commission)
    }

    fn inner_swap_batch_transfer(
        sender: &T::AccountId,
        input_asset_id: &T::AssetId,
        swap_batches: Vec<SwapBatchInfo<T::AssetId, T::DEXId, T::AccountId>>,
        mut max_input_amount: Balance,
        selected_source_types: &Vec<LiquiditySourceType>,
        filter_mode: &FilterMode,
    ) -> Result<Balance, DispatchError> {
        let mut unique_asset_ids: BTreeSet<T::AssetId> = BTreeSet::new();

        let mut executed_batch_input_amount = balance!(0);

        fallible_iterator::convert(swap_batches.into_iter().map(|val| Ok(val))).for_each(
            |swap_batch_info| {
                let SwapBatchInfo {
                    outcome_asset_id: asset_id,
                    dex_id,
                    receivers,
                } = swap_batch_info;

                // extrinsic fails if there are duplicate output asset ids
                if !unique_asset_ids.insert(asset_id.clone()) {
                    fail!(Error::<T>::AggregationError);
                }

                if receivers.len() == 0 {
                    fail!(Error::<T>::InvalidReceiversInfo);
                }

                let out_amount = receivers.iter().map(|recv| recv.target_amount).sum();

                let (executed_input_amount, remainder_per_receiver): (Balance, Balance) =
                    if &asset_id != input_asset_id {
                        Self::exchange_batch_tokens(
                            &sender,
                            receivers.len() as u128,
                            &input_asset_id,
                            &asset_id,
                            max_input_amount,
                            &selected_source_types,
                            dex_id,
                            &filter_mode,
                            out_amount,
                        )?
                    } else {
                        (out_amount, 0)
                    };

                executed_batch_input_amount = executed_batch_input_amount
                    .checked_add(executed_input_amount)
                    .ok_or(Error::<T>::CalculationError)?;

                max_input_amount = max_input_amount
                    .checked_sub(executed_input_amount)
                    .ok_or(Error::<T>::SlippageNotTolerated)?;

                Self::transfer_batch_tokens_unchecked(
                    &sender,
                    &asset_id,
                    receivers,
                    remainder_per_receiver,
                )
            },
        )?;
        let adar_commission = Self::calculate_adar_commission(executed_batch_input_amount)?;
        max_input_amount
            .checked_sub(adar_commission)
            .ok_or(Error::<T>::SlippageNotTolerated)?;
        Ok(adar_commission)
    }
}

impl<T: Config> LiquidityProxyTrait<T::DEXId, T::AccountId, T::AssetId> for Pallet<T> {
    /// Applies trivial routing (via Base Asset), resulting in a poly-swap which may contain several individual swaps.
    /// Those individual swaps are subject to liquidity aggregation algorithm.
    ///
    /// This is a wrapper for `quote_single`.
    fn quote(
        dex_id: T::DEXId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
        deduce_fee: bool,
    ) -> Result<SwapOutcome<Balance>, DispatchError> {
        Pallet::<T>::inner_quote(
            dex_id,
            input_asset_id,
            output_asset_id,
            amount,
            filter,
            true,
            deduce_fee,
        )
        .map(|quote_info| quote_info.outcome)
    }

    /// Applies trivial routing (via Base Asset), resulting in a poly-swap which may contain several individual swaps.
    /// Those individual swaps are subject to liquidity aggregation algorithm.
    ///
    /// This is a wrapper for `exchange_single`.
    fn exchange(
        dex_id: T::DEXId,
        sender: &T::AccountId,
        receiver: &T::AccountId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: SwapAmount<Balance>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<SwapOutcome<Balance>, DispatchError> {
        let (outcome, _) = Pallet::<T>::inner_exchange(
            dex_id,
            sender,
            receiver,
            input_asset_id,
            output_asset_id,
            amount,
            filter,
        )?;
        Ok(outcome)
    }
}

#[derive(
    Encode, Decode, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug, scale_info::TypeInfo,
)]
#[scale_info(skip_type_params(T))]
pub struct BatchReceiverInfo<AccountId> {
    pub account_id: AccountId,
    pub target_amount: Balance,
}

impl<AccountId> BatchReceiverInfo<AccountId> {
    pub fn new(account_id: AccountId, amount: Balance) -> Self {
        BatchReceiverInfo {
            account_id,
            target_amount: amount,
        }
    }
}

#[derive(
    Encode, Decode, Clone, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug, scale_info::TypeInfo,
)]
#[scale_info(skip_type_params(T))]
pub struct SwapBatchInfo<AssetId, DEXId, AccountId> {
    pub outcome_asset_id: AssetId,
    pub dex_id: DEXId,
    pub receivers: Vec<BatchReceiverInfo<AccountId>>,
}

impl<AssetId, DEXId, AccountId> SwapBatchInfo<AssetId, DEXId, AccountId> {
    pub fn len(&self) -> usize {
        self.receivers.len()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::{traits::StorageVersion, transactional};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + common::Config + assets::Config + trading_pair::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type LiquidityRegistry: LiquidityRegistry<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            LiquiditySourceType,
            Balance,
            DispatchError,
        >;
        type GetNumSamples: Get<usize>;
        type GetTechnicalAccountId: Get<Self::AccountId>;
        type PrimaryMarketTBC: GetMarketInfo<Self::AssetId>;
        type PrimaryMarketXST: GetMarketInfo<Self::AssetId>;
        type SecondaryMarket: GetPoolReserves<Self::AssetId>;
        type VestedRewardsPallet: VestedRewardsPallet<Self::AccountId, Self::AssetId>;
        type GetADARAccountId: Get<Self::AccountId>;
        /// Weight information for the extrinsics in this Pallet.
        type WeightInfo: WeightInfo;
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Perform swap of tokens (input/output defined via SwapAmount direction).
        ///
        /// - `origin`: the account on whose behalf the transaction is being executed,
        /// - `dex_id`: DEX ID for which liquidity sources aggregation is being done,
        /// - `input_asset_id`: ID of the asset being sold,
        /// - `output_asset_id`: ID of the asset being bought,
        /// - `swap_amount`: the exact amount to be sold (either in input_asset_id or output_asset_id units with corresponding slippage tolerance absolute bound),
        /// - `selected_source_types`: list of selected LiquiditySource types, selection effect is determined by filter_mode,
        /// - `filter_mode`: indicate either to allow or forbid selected types only, or disable filtering.
        #[pallet::weight(<T as Config>::WeightInfo::swap((*swap_amount).into()))]
        pub fn swap(
            origin: OriginFor<T>,
            dex_id: T::DEXId,
            input_asset_id: T::AssetId,
            output_asset_id: T::AssetId,
            swap_amount: SwapAmount<Balance>,
            selected_source_types: Vec<LiquiditySourceType>,
            filter_mode: FilterMode,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::inner_swap(
                who.clone(),
                who,
                dex_id,
                input_asset_id,
                output_asset_id,
                swap_amount,
                selected_source_types,
                filter_mode,
            )?;
            Ok(().into())
        }

        /// Perform swap of tokens (input/output defined via SwapAmount direction).
        ///
        /// - `origin`: the account on whose behalf the transaction is being executed,
        /// - `receiver`: the account that receives the output,
        /// - `dex_id`: DEX ID for which liquidity sources aggregation is being done,
        /// - `input_asset_id`: ID of the asset being sold,
        /// - `output_asset_id`: ID of the asset being bought,
        /// - `swap_amount`: the exact amount to be sold (either in input_asset_id or output_asset_id units with corresponding slippage tolerance absolute bound),
        /// - `selected_source_types`: list of selected LiquiditySource types, selection effect is determined by filter_mode,
        /// - `filter_mode`: indicate either to allow or forbid selected types only, or disable filtering.
        #[pallet::weight(<T as Config>::WeightInfo::swap((*swap_amount).into()))]
        pub fn swap_transfer(
            origin: OriginFor<T>,
            receiver: T::AccountId,
            dex_id: T::DEXId,
            input_asset_id: T::AssetId,
            output_asset_id: T::AssetId,
            swap_amount: SwapAmount<Balance>,
            selected_source_types: Vec<LiquiditySourceType>,
            filter_mode: FilterMode,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::inner_swap(
                who,
                receiver,
                dex_id,
                input_asset_id,
                output_asset_id,
                swap_amount,
                selected_source_types,
                filter_mode,
            )?;
            Ok(().into())
        }

        /// Dispatches multiple swap & transfer operations. `swap_batches` contains vector of
        /// SwapBatchInfo structs, where each batch specifies which asset ID and DEX ID should
        /// be used for swapping, receiver accounts and their desired outcome amount in asset,
        /// specified for the current batch.
        ///
        /// - `origin`: the account on whose behalf the transaction is being executed,
        /// - `swap_batches`: the vector containing the SwapBatchInfo structs,
        /// - `input_asset_id`: ID of the asset being sold,
        /// - `max_input_amount`: the maximum amount to be sold in input_asset_id,
        /// - `selected_source_types`: list of selected LiquiditySource types, selection effect is
        ///                            determined by filter_mode,
        /// - `filter_mode`: indicate either to allow or forbid selected types only, or disable filtering.
        #[transactional]
        #[pallet::weight(<T as Config>::WeightInfo::swap_transfer_batch(
                swap_batches.len() as u32,
                swap_batches.iter()
                    .map(|batch| batch.len() as u32)
                    .sum()
            )
        )]
        pub fn swap_transfer_batch(
            origin: OriginFor<T>,
            swap_batches: Vec<SwapBatchInfo<T::AssetId, T::DEXId, T::AccountId>>,
            input_asset_id: T::AssetId,
            max_input_amount: Balance,
            selected_source_types: Vec<LiquiditySourceType>,
            filter_mode: FilterMode,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let adar_commission = Self::inner_swap_batch_transfer(
                &who,
                &input_asset_id,
                swap_batches,
                max_input_amount,
                &selected_source_types,
                &filter_mode,
            )?;

            assets::Pallet::<T>::transfer_from(
                &input_asset_id,
                &who,
                &T::GetADARAccountId::get(),
                adar_commission,
            )
            .map_err(|_| Error::<T>::FailedToTransferAdarCommission)?;

            Ok(().into())
        }

        /// Enables XST or TBC liquidity source.
        ///
        /// - `liquidity_source`: the liquidity source to be enabled.
        #[pallet::weight(<T as Config>::WeightInfo::enable_liquidity_source())]
        pub fn enable_liquidity_source(
            origin: OriginFor<T>,
            liquidity_source: LiquiditySourceType,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(
                liquidity_source == LiquiditySourceType::XSTPool
                    || liquidity_source == LiquiditySourceType::MulticollateralBondingCurvePool,
                Error::<T>::UnableToEnableLiquiditySource
            );

            let mut locked = trading_pair::LockedLiquiditySources::<T>::get();

            ensure!(
                locked.contains(&liquidity_source),
                Error::<T>::LiquiditySourceAlreadyEnabled
            );

            locked.retain(|x| *x != liquidity_source);
            trading_pair::LockedLiquiditySources::<T>::set(locked);
            Self::deposit_event(Event::<T>::LiquiditySourceEnabled(liquidity_source));
            Ok(().into())
        }

        /// Disables XST or TBC liquidity source. The liquidity source becomes unavailable for swap.
        ///
        /// - `liquidity_source`: the liquidity source to be disabled.
        #[pallet::weight(<T as Config>::WeightInfo::disable_liquidity_source())]
        pub fn disable_liquidity_source(
            origin: OriginFor<T>,
            liquidity_source: LiquiditySourceType,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(
                liquidity_source == LiquiditySourceType::XSTPool
                    || liquidity_source == LiquiditySourceType::MulticollateralBondingCurvePool,
                Error::<T>::UnableToDisableLiquiditySource
            );
            ensure!(
                !trading_pair::LockedLiquiditySources::<T>::get().contains(&liquidity_source),
                Error::<T>::LiquiditySourceAlreadyDisabled
            );
            trading_pair::LockedLiquiditySources::<T>::append(liquidity_source);
            Self::deposit_event(Event::<T>::LiquiditySourceDisabled(liquidity_source));
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Exchange of tokens has been performed
        /// [Caller Account, DEX Id, Input Asset Id, Output Asset Id, Input Amount, Output Amount, Fee Amount]
        Exchange(
            AccountIdOf<T>,
            DexIdOf<T>,
            AssetIdOf<T>,
            AssetIdOf<T>,
            Balance,
            Balance,
            Balance,
            Vec<LiquiditySourceIdOf<T>>,
        ),
        /// Liquidity source was enabled
        LiquiditySourceEnabled(LiquiditySourceType),
        /// Liquidity source was disabled
        LiquiditySourceDisabled(LiquiditySourceType),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// No route exists in a given DEX for given parameters to carry out the swap
        UnavailableExchangePath,
        /// Max fee exceeded
        MaxFeeExceeded,
        /// Fee value outside of the basis points range [0..10000]
        InvalidFeeValue,
        /// None of the sources has enough reserves to execute a trade
        InsufficientLiquidity,
        /// Path exists but it's not possible to perform exchange with currently available liquidity on pools.
        AggregationError,
        /// Specified parameters lead to arithmetic error
        CalculationError,
        /// Slippage either exceeds minimum tolerated output or maximum tolerated input.
        SlippageNotTolerated,
        /// Selected filtering request is not allowed.
        ForbiddenFilter,
        /// Failure while calculating price ignoring non-linearity of liquidity source.
        FailedToCalculatePriceWithoutImpact,
        /// Unable to swap indivisible assets
        UnableToSwapIndivisibleAssets,
        /// Unable to enable liquidity source
        UnableToEnableLiquiditySource,
        /// Liquidity source is already enabled
        LiquiditySourceAlreadyEnabled,
        /// Unable to disable liquidity source
        UnableToDisableLiquiditySource,
        /// Liquidity source is already disabled
        LiquiditySourceAlreadyDisabled,
        // Information about swap batch receivers is invalid
        InvalidReceiversInfo,
        // Failure while transferring commission to ADAR account
        FailedToTransferAdarCommission,
    }
}
