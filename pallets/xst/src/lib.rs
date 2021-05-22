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

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;

use core::convert::TryInto;

use assets::AssetIdOf;
use codec::{Decode, Encode};
use common::fixnum::ops::Zero as _;
use common::prelude::{
    Balance, EnsureDEXManager, EnsureTradingPairExists, Fixed, FixedWrapper, QuoteAmount,
    SwapAmount, SwapOutcome,
};
use common::{
    balance, fixed, fixed_wrapper, DEXId, DexIdOf, GetMarketInfo, LiquiditySource,
    LiquiditySourceFilter, LiquiditySourceType, ManagementMode, PSWAP, USDT, VAL, //TODO: rename USDT to DAI. It is NOT USDT
};
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::{ensure, fail};
use frame_system::ensure_signed;
use liquidity_proxy::LiquidityProxyTrait;
use permissions::{Scope, BURN, MINT, TRANSFER};
use pswap_distribution::{OnPswapBurned, PswapRemintInfo};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::Zero;
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::vec::Vec;

pub trait WeightInfo {
    fn on_initialize(_elems: u32) -> Weight;
    fn initialize_pool() -> Weight;
    fn set_reference_asset() -> Weight;
}

type Assets<T> = assets::Module<T>;
type Technical<T> = technical::Module<T>;

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"xst-pool";

pub const RETRY_DISTRIBUTION_FREQUENCY: u32 = 1000;

pub use pallet::*;

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DistributionAccount<AccountId, TechAccountId> {
    Account(AccountId),
    TechAccount(TechAccountId),
}

impl<AccountId, TechAccountId: Default> Default for DistributionAccount<AccountId, TechAccountId> {
    fn default() -> Self {
        Self::TechAccount(TechAccountId::default())
    }
}

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DistributionAccountData<DistributionAccount> {
    pub account: DistributionAccount,
    pub coefficient: Fixed,
}

impl<DistributionAccount: Default> Default for DistributionAccountData<DistributionAccount> {
    fn default() -> Self {
        Self {
            account: Default::default(),
            coefficient: Default::default(),
        }
    }
}

impl<DistributionAccount> DistributionAccountData<DistributionAccount> {
    pub fn new(account: DistributionAccount, coefficient: Fixed) -> Self {
        DistributionAccountData {
            account,
            coefficient,
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + common::Config
        + assets::Config
        + technical::Config
        + trading_pair::Config
        + pool_xyk::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type LiquidityProxy: LiquidityProxyTrait<Self::DEXId, Self::AccountId, Self::AssetId>;
        type EnsureDEXManager: EnsureDEXManager<Self::DEXId, Self::AccountId, DispatchError>;
        type EnsureTradingPairExists: EnsureTradingPairExists<
            Self::DEXId,
            Self::AssetId,
            DispatchError,
        >;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Enable exchange path on the pool for pair BaseAsset-SyntheticAsset.
        #[pallet::weight(<T as Config>::WeightInfo::initialize_pool())]
        pub fn initialize_pool(
            origin: OriginFor<T>,
            collateral_asset_id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            let _who = <T as Config>::EnsureDEXManager::ensure_can_manage(
                &DEXId::Polkaswap.into(),
                origin,
                ManagementMode::Private,
            )?;
            Self::initialize_pool_unchecked(collateral_asset_id, true)?;
            Ok(().into())
        }

        /// Change reference asset which is used to determine collateral assets value. Inteded to be e.g. stablecoin DAI.
        #[pallet::weight(<T as Config>::WeightInfo::set_reference_asset())]
        pub fn set_reference_asset(
            origin: OriginFor<T>,
            reference_asset_id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            let _who = <T as Config>::EnsureDEXManager::ensure_can_manage(
                &DEXId::Polkaswap.into(),
                origin,
                ManagementMode::Private,
            )?;
            ReferenceAssetId::<T>::put(reference_asset_id.clone());
            Self::deposit_event(Event::ReferenceAssetChanged(reference_asset_id));
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::metadata(DexIdOf<T> = "DEXId", AssetIdOf<T> = "AssetId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Pool is initialized for pair. [DEX Id, Synthetic Asset Id]
        PoolInitialized(DexIdOf<T>, AssetIdOf<T>),
        /// Reference Asset has been changed for pool. [New Reference Asset Id]
        ReferenceAssetChanged(AssetIdOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// An error occurred while calculating the price.
        PriceCalculationFailed,
        /// The pool can't perform exchange on itself.
        CannotExchangeWithSelf,
        /// Attempt to initialize pool for pair that already exists.
        PoolAlreadyInitializedForPair,
        /// Attempt to get info for uninitialized pool.
        PoolNotInitialized,
        /// Indicated limits for slippage has not been met during transaction execution.
        SlippageLimitExceeded,
        /// Indicated collateral asset is not enabled for pool.
        UnsupportedCollateralAssetId,
        /// Could not calculate fee including sell penalty.
        FeeCalculationFailed,
        /// Liquidity source can't exchange assets with the given IDs on the given DEXId.
        CantExchange,
        /// Increment account reference error.
        IncRefError,
    }

    /// Technical account used to store collateral tokens.
    #[pallet::storage]
    #[pallet::getter(fn reserves_account_id)]
    pub type ReservesAcc<T: Config> = StorageValue<_, T::TechAccountId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn free_reserves_account_id)]
    pub type FreeReservesAccountId<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_free_reserves)]
    pub type PendingFreeReserves<T: Config> =
        StorageValue<_, Vec<(T::AssetId, Balance)>, ValueQuery>;

    #[pallet::type_value]
    pub(super) fn DefaultForInitialPrice() -> Fixed {
        fixed!(634)
    }

    /// Buy price starting constant. This is the price users pay for new XOR.
    #[pallet::storage]
    #[pallet::getter(fn initial_price)]
    pub(super) type InitialPrice<T: Config> =
        StorageValue<_, Fixed, ValueQuery, DefaultForInitialPrice>;

    /// Coefficient which determines the fraction of input collateral token to be exchanged to XOR and
    /// be distributed to predefined accounts. Relevant for the Buy function (when a user buys new XOR).
    #[pallet::storage]
    #[pallet::getter(fn always_distribute_coefficient)]
    pub(super) type AlwaysDistributeCoefficient<T: Config> =
        StorageValue<_, Fixed, ValueQuery, DefaultForAlwaysDistributeCoefficient>;

    #[pallet::type_value]
    pub(super) fn DefaultForBaseFee() -> Fixed {
        fixed!(0.003)
    }

    /// Base fee in XOR which is deducted on all trades, currently it's burned: 0.3%.
    #[pallet::storage]
    #[pallet::getter(fn base_fee)]
    pub(super) type BaseFee<T: Config> = StorageValue<_, Fixed, ValueQuery, DefaultForBaseFee>;

    /// XST Assets allowed to be traded using XST.
    #[pallet::storage]
    #[pallet::getter(fn enabled_synthetics)]
    pub type EnabledSynthetics<T: Config> = StorageValue<_, BTreeSet<T::AssetId>, ValueQuery>;

    /// Asset that is used to compare collateral assets by value, e.g., DAI.
    #[pallet::storage]
    #[pallet::getter(fn reference_asset_id)]
    pub type ReferenceAssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

    /// Current reserves balance for collateral tokens, used for client usability.
    #[pallet::storage]
    pub(super) type CollateralReserves<T: Config> =
        StorageMap<_, Twox64Concat, T::AssetId, Balance, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Technical account used to store collateral tokens.
        pub reserves_account_id: T::TechAccountId,
        /// Asset that is used to compare collateral assets by value, e.g., DAI.
        pub reference_asset_id: T::AssetId,
        /// List of tokens enabled as collaterals initially.
        pub initial_synthetic_assets: Vec<T::AssetId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                reference_asset_id: USDT.into(), //TODO:rename to DAI
                incentives_account_id: Default::default(),
                initial_collateral_assets: [USDT.into(), VAL.into(), PSWAP.into()].into(),
                free_reserves_account_id: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            ReferenceAssetId::<T>::put(&self.reference_asset_id);
            self.initial_collateral_assets
                .iter()
                .cloned()
                .for_each(|asset_id| {
                    Pallet::<T>::initialize_pool_unchecked(asset_id, false)
                        .expect("Failed to initialize XST synthetics.")
                });
        }
    }
}

/// This function is used by `exchange` function to burn calculated `input_amount` of
/// `in_asset_id` and mint `output_amount` of `out_asset_id`.
///
struct BuyMainAsset<T: Config> {
    collateral_asset_id: T::AssetId,
    main_asset_id: T::AssetId,
    amount: SwapAmount<Balance>,
    from_account_id: T::AccountId,
    to_account_id: T::AccountId,
    synthetics_tech_account_id: T::TechAccountId,
    synthetics_account_id: T::AccountId,
}

impl<T: Config> BuyMainAsset<T> {
    pub fn new(
        synthetic_asset_id: T::AssetId,
        main_asset_id: T::AssetId,
        amount: SwapAmount<Balance>,
        from_account_id: T::AccountId,
        to_account_id: T::AccountId,
    ) -> Result<Self, DispatchError> {
        let synthetics_tech_account_id = ReservesAcc::<T>::get();
        let synthetics_account_id =
            Technical::<T>::tech_account_id_to_account_id(&synthetics_tech_account_id)?;
        Ok(BuyMainAsset {
            synthetic_asset_id,
            main_asset_id,
            amount,
            from_account_id,
            to_account_id,
            synthetics_tech_account_id,
            synthetics_account_id,
        })
    }

    fn burn_input(&self, input_asset: T::AssetId, input_amount: Balance) -> Result<(), DispatchError> {
        Assets::<T>::burn_from(
            input_asset,
            &self.synthetics_account_id,
            &self.from_account_id,
            input_amount,
        )?;
        Ok(())
    }

    fn mint_output(&self, output_asset: T::AssetId, output_amount: Balance) -> Result<(), DispatchError> {
        Assets::<T>::mint_to(
            output_asset,
            &self.synthetics_account_id,
            &self.to_account_id,
            output_amount,
        )?;
        Ok(())
    }

    fn swap(&self) -> Result<SwapOutcome<Balance>, DispatchError> {
        common::with_transaction(|| {
            self.burn_input(input_amount.clone())?;
            self.mint_output(output_amount.clone())?;

            Ok(match self.amount {
                SwapAmount::WithDesiredInput { .. } => SwapOutcome::new(output_amount, fee),
                SwapAmount::WithDesiredOutput { .. } => SwapOutcome::new(input_amount, fee),
            })
        })
    }
}

#[allow(non_snake_case)]
impl<T: Config> Module<T> {
    fn add_free_reserves_to_pending_list(
        holder: &T::AccountId,
        collateral_asset_id: T::AssetId,
        amount: Balance,
    ) -> DispatchResult {
        let free_reserves_acc = FreeReservesAccountId::<T>::get();
        Assets::<T>::transfer_from(&collateral_asset_id, holder, &free_reserves_acc, amount)?;
        PendingFreeReserves::<T>::mutate(|vec| vec.push((collateral_asset_id, amount)));
        Ok(())
    }

    #[inline]
    fn self_excluding_filter() -> LiquiditySourceFilter<T::DEXId, LiquiditySourceType> {
        LiquiditySourceFilter::with_forbidden(
            DEXId::Polkaswap.into(),
            [LiquiditySourceType::XSTPool].into(),
        )
    }

    fn initialize_pool_unchecked(
        synthetic_asset_id: T::AssetId,
        transactional: bool,
    ) -> DispatchResult {
        let code = || {
            ensure!(
                !EnabledSynthetics::<T>::get().contains(&synthetic_asset_id),
                Error::<T>::PoolAlreadyInitializedForPair
            );
            T::EnsureTradingPairExists::ensure_trading_pair_exists(
                &DEXId::Polkaswap.into(),
                &T::GetBaseAssetId::get(),
                &collateral_asset_id,
            )?;
            trading_pair::Module::<T>::enable_source_for_trading_pair(
                &DEXId::Polkaswap.into(),
                &T::GetBaseAssetId::get(),
                &collateral_asset_id,
                LiquiditySourceType::XSTPool,
            )?;
            if Self::collateral_is_incentivised(&synthetic_asset_id) {
                IncentivisedCurrenciesNum::<T>::mutate(|num| *num += 1)
            }
            EnabledSynthetics::<T>::mutate(|set| set.insert(synthetic_asset_id));
            Self::deposit_event(Event::PoolInitialized(
                DEXId::Polkaswap.into(),
                synthetic_asset_id,
            ));
            Ok(())
        };
        if transactional {
            common::with_transaction(|| code())
        } else {
            code()
        }
    }

    /// Buy function with regards to asset total supply and its change delta. It represents the amount of
    /// input collateral required from User in order to receive requested XOR amount. I.e. the price User buys at.
    ///
    /// XOR is also referred as main asset.
    /// Value of `delta` is assumed to be either positive or negative.
    /// For every `price_change_step` tokens the price goes up by `price_change_rate`.
    ///
    /// `buy_price_usd = (xor_total_supply + xor_supply_delta) / (price_change_step * price_change_rate) + initial_price_usd`
    ///
    pub fn buy_function(main_asset_id: &T::AssetId, delta: Fixed) -> Result<Fixed, DispatchError> {
        let total_supply: FixedWrapper = Assets::<T>::total_issuance(main_asset_id)?.into();
        let initial_price: FixedWrapper = Self::initial_price().into();
        let price_change_step: FixedWrapper = Self::price_change_step().into();
        let price_change_rate: FixedWrapper = Self::price_change_rate().into();

        let price =
            (total_supply + delta) / (price_change_step * price_change_rate) + initial_price;
        price
            .get()
            .map_err(|_| Error::<T>::PriceCalculationFailed.into())
    }

    /// Calculates and returns the current buy price, assuming that input is the collateral asset and output is the main asset.
    ///
    /// To calculate price for a specific amount of assets (with desired main asset output),
    /// one needs to calculate the area of a right trapezoid.
    ///
    /// `AB` : buy_function(xor_total_supply)
    /// `CD` : buy_function(xor_total_supply + xor_supply_delta)
    ///
    /// ```nocompile
    ///          ..  C
    ///        ..  │
    ///   B  ..    │
    ///     │   S  │
    ///     │      │
    ///   A └──────┘ D
    /// ```
    ///
    /// 1) Amount of collateral tokens needed in USD to get `xor_supply_delta`(AD) XOR tokens
    /// ```nocompile
    /// S = ((AB + CD) / 2) * AD
    ///
    /// or
    ///
    /// buy_price_usd = ((buy_function(xor_total_supply) + buy_function(xor_total_supply + xor_supply_delta)) / 2) * xor_supply_delta
    /// ```
    /// 2) Amount of XOR tokens received by depositing `S` collateral tokens in USD:
    ///
    /// Solving right trapezoid area formula with respect to `xor_supply_delta` (AD),
    /// actual square `S` is known and represents collateral amount.
    /// We have a quadratic equation:
    /// ```nocompile
    /// buy_function(x) = price_change_coefficient * x + initial_price
    /// Assume `M` = 1 / price_change_coefficient = 1 / 1337
    ///
    /// M * AD² + 2 * AB * AD - 2 * S = 0
    /// equation with two solutions, taking only positive one:
    /// AD = (√((AB * 2 / M)² + 8 * S / M) - 2 * AB / M) / 2
    ///
    /// or
    ///
    /// xor_supply_delta = (√((buy_function(xor_total_supply) * 2 / price_change_coeff)²
    ///                    + 8 * buy_price_usd / price_change_coeff) - 2 * buy_function(xor_total_supply)
    ///                    / price_change_coeff) / 2
    /// ```
    pub fn buy_price(
        main_asset_id: &T::AssetId,
        synthetic_asset_id: &T::AssetId,
        quantity: QuoteAmount<Balance>,
    ) -> Result<Fixed, DispatchError> {

        let current_state: FixedWrapper = Self::buy_function(main_asset_id, Fixed::ZERO)?.into();
        let main_asset_price_per_reference_unit: FixedWrapper =
            Self::reference_price(main_asset_id)?.into();

        match quantity {
            // Input target amount of XST(USD) to get some XOR
            QuoteAmount::WithDesiredInput {
                desired_amount_in: synthetic_quantity,
            } => {
                //TODO: here we assume only DAI-pegged XST(USD) synthetics. Need to have a price oracle to handle other synthetics in the future!
                let main_out = synthetic_quantity / main_asset_price_per_reference_unit;
                main_out
                    .get()
                    .map_err(|_| Error::<T>::PriceCalculationFailed.into())
                    .map(|value| value.max(Fixed::ZERO))
            }
            // Input some XST(USD) to get a target amount of XOR
            QuoteAmount::WithDesiredOutput {
                desired_amount_out: main_quantity,
            } => {
                let new_state: FixedWrapper = Self::buy_function(
                    main_asset_id,
                    FixedWrapper::from(main_quantity)
                        .get()
                        .map_err(|_| Error::<T>::PriceCalculationFailed)?,
                )?
                .into();
                //TODO: here we assume only DAI-pegged XST(USD) synthetics. Need to have a price oracle to handle other synthetics in the future!
                let synthetic_quantity = main_quantity * main_asset_price_per_reference_unit;
                synthetic_quantity
                    .get()
                    .map_err(|_| Error::<T>::PriceCalculationFailed.into())
                    .map(|value| value.max(Fixed::ZERO))
            }
        }
    }

    /// Calculates and returns the current sell price, assuming that input is the main asset and output is the collateral asset.
    ///
    /// To calculate sell price for a specific amount of assets:
    /// 1. Current reserves of collateral token are taken
    /// 2. Same amount by value is assumed for main asset
    ///   2.1 Values are compared via getting prices for both main and collateral tokens with regard to another token
    ///       called reference token which is set for particular pair. This should be e.g. stablecoin DAI.
    ///   2.2 Reference price for base token is taken as 80% of current bonding curve buy price.
    ///   2.3 Reference price for collateral token is taken as current market price, i.e. price for 1 token on liquidity proxy.
    /// 3. Given known reserves for main and collateral, output collateral amount is calculated by applying x*y=k model resulting
    ///    in curve-like dependency.
    pub fn sell_price(
        main_asset_id: &T::AssetId,
        synthetic_asset_id: &T::AssetId,
        quantity: QuoteAmount<Balance>,
    ) -> Result<Fixed, DispatchError> {
        let reserves_tech_account_id = ReservesAcc::<T>::get();
        let reserves_account_id =
            Technical::<T>::tech_account_id_to_account_id(&reserves_tech_account_id)?;
        let collateral_supply: FixedWrapper =
            Assets::<T>::free_balance(synthetic_asset_id, &reserves_account_id)?.into();
        // Get reference prices for base and collateral to understand token value.
        let main_price_per_reference_unit: FixedWrapper =
            Self::sell_function(main_asset_id, Fixed::ZERO)?.into();
        let collateral_price_per_reference_unit: FixedWrapper =
            Self::reference_price(synthetic_asset_id)?.into();
        // Assume main token reserve is equal by reference value to collateral token reserve.
        let main_supply = collateral_supply.clone() * collateral_price_per_reference_unit
            / main_price_per_reference_unit;
        let collateral_supply_unwrapped = collateral_supply
            .clone()
            .get()
            .map_err(|_| Error::<T>::PriceCalculationFailed)?;

        match quantity {
            // Sell desired amount of XOR for some XST(USD)
            QuoteAmount::WithDesiredInput {
                desired_amount_in: quantity_main,
            } => {
                let output_synthetic = quantity_main * main_price_per_reference_unit;
                let output_synthetic_unwrapped = output_synthetic
                    .get()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                Ok(output_collateral_unwrapped)
            }
            // Sell some amount of XOR for desired amount of XST(USD)
            QuoteAmount::WithDesiredOutput {
                desired_amount_out: quantity_collateral,
            } => {
                let collateral_supply_unwrapped = collateral_supply_unwrapped
                    .into_bits()
                    .try_into()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                ensure!(
                    quantity_collateral < collateral_supply_unwrapped,
                    Error::<T>::NotEnoughReserves
                );
                let output_main = quantity_collateral / main_price_per_reference_unit;
                output_main
                    .get()
                    .map_err(|_| Error::<T>::PriceCalculationFailed.into())
            }
        }
    }

    /// Sell function with regards to asset total supply and its change delta. It represents the amount of
    /// output collateral tokens received by User by indicating exact sold XOR amount. I.e. the price User sells at.
    ///
    /// Value of `delta` is assumed to be either positive or negative.
    /// Sell function is `sell_price_coefficient`% of buy function (see `buy_function`).
    ///
    /// `sell_price = sell_price_coefficient * buy_price`
    ///
    pub fn sell_function(main_asset_id: &T::AssetId, delta: Fixed) -> Result<Fixed, DispatchError> {
        let buy_price = Self::buy_function(main_asset_id, delta)?;
        let sell_price_coefficient = FixedWrapper::from(Self::sell_price_coefficient());
        let sell_price = sell_price_coefficient * buy_price;
        sell_price
            .get()
            .map_err(|_| Error::<T>::PriceCalculationFailed.into())
    }

    /// Decompose SwapAmount into particular buy quotation query.
    ///
    /// Returns ordered pair: (input_amount, output_amount, fee_amount).
    fn decide_buy_amounts(
        main_asset_id: &T::AssetId,
        collateral_asset_id: &T::AssetId,
        amount: SwapAmount<Balance>,
    ) -> Result<(Balance, Balance, Balance), DispatchError> {
        Ok(match amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => {
                let mut output_amount: Balance = FixedWrapper::from(Self::buy_price(
                    main_asset_id,
                    collateral_asset_id,
                    QuoteAmount::with_desired_input(desired_amount_in),
                )?)
                .try_into_balance()
                .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                let fee_amount = (FixedWrapper::from(BaseFee::<T>::get()) * output_amount)
                    .try_into_balance()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                output_amount = output_amount.saturating_sub(fee_amount);
                ensure!(
                    output_amount >= min_amount_out,
                    Error::<T>::SlippageLimitExceeded
                );
                (desired_amount_in, output_amount, fee_amount)
            }
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => {
                let desired_amount_out_with_fee = (FixedWrapper::from(desired_amount_out)
                    / (fixed_wrapper!(1) - BaseFee::<T>::get()))
                .try_into_balance()
                .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                let input_amount = Self::buy_price(
                    main_asset_id,
                    collateral_asset_id,
                    QuoteAmount::with_desired_output(desired_amount_out_with_fee.clone()),
                )?;
                let input_amount = input_amount
                    .into_bits()
                    .try_into()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                ensure!(
                    input_amount <= max_amount_in,
                    Error::<T>::SlippageLimitExceeded
                );
                (
                    input_amount,
                    desired_amount_out,
                    desired_amount_out_with_fee.saturating_sub(desired_amount_out),
                )
            }
        })
    }

    /// Calculate percentage of fee penalty that is applied to trades when XOR is sold while
    /// reserves are low for target collateral asset.
    fn sell_penalty(collateral_asset_id: &T::AssetId) -> Result<Fixed, DispatchError> {
        let reserves_account_id =
            Technical::<T>::tech_account_id_to_account_id(&Self::reserves_account_id())?;
        // USD price for XOR supply on network
        let ideal_reserves_price: FixedWrapper =
            Self::ideal_reserves_reference_price(Fixed::ZERO)?.into();
        // USD price for amount of indicated collateral asset stored in reserves
        let collateral_reserves_price =
            Self::actual_reserves_reference_price(&reserves_account_id, collateral_asset_id)?;
        ensure!(
            !collateral_reserves_price.is_zero(),
            Error::<T>::NotEnoughReserves
        );
        // ratio of stored reserves to ideal reserves
        let collateralized_fraction = (FixedWrapper::from(collateral_reserves_price)
            / ideal_reserves_price)
            .get()
            .map_err(|_| Error::<T>::FeeCalculationFailed)?;
        Ok(Self::map_collateralized_fraction_to_penalty(
            collateralized_fraction,
        ))
    }

    /// Decompose SwapAmount into particular sell quotation query.
    ///
    /// Returns ordered pair: (input_amount, output_amount, fee_amount).
    fn decide_sell_amounts(
        main_asset_id: &T::AssetId,
        collateral_asset_id: &T::AssetId,
        amount: SwapAmount<Balance>,
    ) -> Result<(Balance, Balance, Balance), DispatchError> {
        Ok(match amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => {
                let fee_percentage = FixedWrapper::from(BaseFee::<T>::get())
                    + Self::sell_penalty(collateral_asset_id)?;
                let fee_amount = (fee_percentage * FixedWrapper::from(desired_amount_in))
                    .try_into_balance()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                let output_amount = Self::sell_price(
                    main_asset_id,
                    collateral_asset_id,
                    QuoteAmount::with_desired_input(
                        desired_amount_in.saturating_sub(fee_amount.clone()),
                    ),
                )?;
                let output_amount = output_amount
                    .into_bits()
                    .try_into()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                ensure!(
                    output_amount >= min_amount_out,
                    Error::<T>::SlippageLimitExceeded
                );
                (desired_amount_in, output_amount, fee_amount)
            }
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => {
                let input_amount: Balance = FixedWrapper::from(Self::sell_price(
                    main_asset_id,
                    collateral_asset_id,
                    QuoteAmount::with_desired_output(desired_amount_out),
                )?)
                .try_into_balance()
                .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                let fee_percentage = FixedWrapper::from(BaseFee::<T>::get())
                    + Self::sell_penalty(collateral_asset_id)?;
                let input_amount_with_fee =
                    FixedWrapper::from(input_amount) / (fixed_wrapper!(1) - fee_percentage);
                let input_amount_with_fee = input_amount_with_fee
                    .try_into_balance()
                    .map_err(|_| Error::<T>::PriceCalculationFailed)?;
                ensure!(
                    input_amount <= max_amount_in,
                    Error::<T>::SlippageLimitExceeded
                );
                (
                    input_amount_with_fee,
                    desired_amount_out,
                    input_amount_with_fee.saturating_sub(input_amount),
                )
            }
        })
    }

    /// This function is used by `exchange` function to burn `input_amount` derived from `amount` of `main_asset_id`
    /// and transfer calculated amount of `collateral_asset_id` to the receiver from reserves.
    ///
    /// If there's not enough reserves in the pool, `NotEnoughReserves` error will be returned.
    ///
    fn sell_main_asset(
        _dex_id: &T::DEXId,
        main_asset_id: &T::AssetId,
        collateral_asset_id: &T::AssetId,
        amount: SwapAmount<Balance>,
        from_account_id: &T::AccountId,
        to_account_id: &T::AccountId,
    ) -> Result<SwapOutcome<Balance>, DispatchError> {
        common::with_transaction(|| {
            let reserves_tech_account_id = Self::reserves_account_id();
            let reserves_account_id =
                Technical::<T>::tech_account_id_to_account_id(&reserves_tech_account_id)?;
            let (input_amount, output_amount, fee_amount) =
                Self::decide_sell_amounts(main_asset_id, collateral_asset_id, amount)?;

            technical::Module::<T>::transfer_out(
                collateral_asset_id,
                &reserves_tech_account_id,
                &to_account_id,
                output_amount,
            )?;
            Assets::<T>::burn_from(
                main_asset_id,
                &reserves_account_id,
                from_account_id,
                input_amount,
            )?;
            Ok(SwapOutcome::new(output_amount, fee_amount))
        })
    }

    /// Assign account id that is used to store deposited collateral tokens.
    pub fn set_reserves_account_id(account: T::TechAccountId) -> Result<(), DispatchError> {
        common::with_transaction(|| {
            ReservesAcc::<T>::set(account.clone());
            let account_id = Technical::<T>::tech_account_id_to_account_id(&account)?;
            let permissions = [BURN, MINT, TRANSFER];
            for permission in &permissions {
                permissions::Module::<T>::assign_permission(
                    account_id.clone(),
                    &account_id,
                    *permission,
                    Scope::Unlimited,
                )?;
            }
            Ok(())
        })
    }

    /// This function is used to determine particular asset price in terms of a reference asset, which is set for
    /// bonding curve (there could be only single token chosen as reference for all comparisons). Basically, the
    /// reference token is expected to be a USD-bound stablecoin, e.g. DAI.
    ///
    /// Example use: understand actual value of two tokens in terms of USD.
    fn reference_price(asset_id: &T::AssetId) -> Result<Balance, DispatchError> {
        let reference_asset_id = ReferenceAssetId::<T>::get();
        let price = if asset_id == &reference_asset_id {
            balance!(1)
        } else {
            <T as pallet::Config>::LiquidityProxy::quote(
                asset_id,
                &reference_asset_id,
                SwapAmount::with_desired_input(balance!(1), Balance::zero()),
                Self::self_excluding_filter(),
            )?
            .amount
        };
        Ok(price)
    }

impl<T: Config> LiquiditySource<T::DEXId, T::AccountId, T::AssetId, Balance, DispatchError>
    for Module<T>
{
    fn can_exchange(
        dex_id: &T::DEXId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
    ) -> bool {
        if *dex_id != DEXId::Polkaswap.into() {
            return false;
        }
        if input_asset_id == &T::GetBaseAssetId::get() {
            EnabledSynthetics::<T>::get().contains(&output_asset_id)
        } else {
            EnabledSynthetics::<T>::get().contains(&input_asset_id)
        }
    }

    fn quote(
        dex_id: &T::DEXId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        swap_amount: SwapAmount<Balance>,
    ) -> Result<SwapOutcome<Balance>, DispatchError> {
        if !Self::can_exchange(dex_id, input_asset_id, output_asset_id) {
            fail!(Error::<T>::CantExchange);
        }
        let base_asset_id = &T::GetBaseAssetId::get();
        let (input_amount, output_amount, fee_amount) = if input_asset_id == base_asset_id {
            Self::decide_sell_amounts(&input_asset_id, &output_asset_id, swap_amount)?
        } else {
            Self::decide_buy_amounts(&output_asset_id, &input_asset_id, swap_amount)?
        };
        match swap_amount {
            SwapAmount::WithDesiredInput { .. } => Ok(SwapOutcome::new(output_amount, fee_amount)),
            SwapAmount::WithDesiredOutput { .. } => Ok(SwapOutcome::new(input_amount, fee_amount)),
        }
    }

    fn exchange(
        sender: &T::AccountId,
        receiver: &T::AccountId,
        dex_id: &T::DEXId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        desired_amount: SwapAmount<Balance>,
    ) -> Result<SwapOutcome<Balance>, DispatchError> {
        if !Self::can_exchange(dex_id, input_asset_id, output_asset_id) {
            fail!(Error::<T>::CantExchange);
        }
        let reserves_account_id =
            &Technical::<T>::tech_account_id_to_account_id(&Self::reserves_account_id())?;
        // This is needed to prevent recursion calls.
        if sender == reserves_account_id && receiver == reserves_account_id {
            fail!(Error::<T>::CannotExchangeWithSelf);
        }
        let base_asset_id = &T::GetBaseAssetId::get();
        if input_asset_id == base_asset_id {
            let outcome = Self::sell_main_asset(
                dex_id,
                input_asset_id,
                output_asset_id,
                desired_amount,
                sender,
                receiver,
            );
            Module::<T>::update_collateral_reserves(output_asset_id, reserves_account_id)?;
            outcome
        } else {
            let outcome = BuyMainAsset::<T>::new(
                *input_asset_id,
                *output_asset_id,
                desired_amount,
                sender.clone(),
                receiver.clone(),
            )?
            .swap();
            Module::<T>::update_collateral_reserves(input_asset_id, reserves_account_id)?;
            outcome
        }
    }
}

impl<T: Config> GetMarketInfo<T::AssetId> for Module<T> {
    fn buy_price(
        base_asset: &T::AssetId,
        collateral_asset: &T::AssetId,
    ) -> Result<Fixed, DispatchError> {
        let base_price_wrt_ref: FixedWrapper = Self::buy_function(base_asset, fixed!(0))?.into();
        let collateral_price_per_reference_unit: FixedWrapper =
            Self::reference_price(collateral_asset)?.into();
        let output = (base_price_wrt_ref / collateral_price_per_reference_unit)
            .get()
            .map_err(|_| Error::<T>::PriceCalculationFailed)?;
        Ok(output)
    }

    fn sell_price(
        base_asset: &T::AssetId,
        collateral_asset: &T::AssetId,
    ) -> Result<Fixed, DispatchError> {
        let base_price_wrt_ref: FixedWrapper = Self::sell_function(base_asset, fixed!(0))?.into();
        let collateral_price_per_reference_unit: FixedWrapper =
            Self::reference_price(collateral_asset)?.into();
        let output = (base_price_wrt_ref / collateral_price_per_reference_unit)
            .get()
            .map_err(|_| Error::<T>::PriceCalculationFailed)?;
        Ok(output)
    }

    fn enabled_synthetics() -> BTreeSet<T::AssetId> {
        EnabledSynthetics::<T>::get()
    }
}
