//! # ETH
//!
//! An application that implements a bridged ETH asset.
//!
//! ## Overview
//!
//! ETH balances are stored in the tightly-coupled [`asset`] runtime module. When an account holder burns
//! some of their balance, a `Transfer` event is emit for this event
//! and relay it to the other chain.
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `burn`: Burn an ETH balance.
//!
#![cfg_attr(not(feature = "std"), no_std)]

use common::prelude::constants::EXTRINSIC_FIXED_WEIGHT;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::EnsureOrigin;
use frame_support::weights::Weight;
use frame_system::ensure_signed;
use sp_core::{H160, U256};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use bridge_types::traits::OutboundRouter;
use bridge_types::types::ChannelId;
use bridge_types::EthNetworkId;

mod payload;
use payload::OutboundPayload;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Weight functions needed for this pallet.
pub trait WeightInfo {
    fn burn() -> Weight;
    fn mint() -> Weight;
    fn register_network() -> Weight;
    fn register_network_with_existing_asset() -> Weight;
}

impl WeightInfo for () {
    fn burn() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn mint() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn register_network() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn register_network_with_existing_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use assets::AssetIdOf;
    use common::{AssetName, AssetSymbol, Balance, DEFAULT_BALANCE_PRECISION};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::StorageVersion;
    use frame_support::transactional;
    use frame_system::pallet_prelude::{OriginFor, *};
    use frame_system::RawOrigin;
    use traits::MultiCurrency;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as assets::Config>::Currency as MultiCurrency<AccountIdOf<T>>>::Balance;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + assets::Config + technical::Config + permissions::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type OutboundRouter: OutboundRouter<Self::AccountId>;

        type CallOrigin: EnsureOrigin<Self::Origin, Success = (EthNetworkId, H160)>;

        type BridgeTechAccountId: Get<Self::TechAccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn address_and_asset)]
    pub(super) type Addresses<T: Config> =
        StorageMap<_, Identity, EthNetworkId, (H160, AssetIdOf<T>), OptionQuery>;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    /// Events for the ETH module.
    pub enum Event<T: Config> {
        Burned(EthNetworkId, AccountIdOf<T>, H160, BalanceOf<T>),
        Minted(EthNetworkId, H160, AccountIdOf<T>, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The submitted payload could not be decoded.
        InvalidPayload,
        /// App for given network is not registered.
        AppIsNotRegistered,
        /// Message came from wrong address.
        InvalidAppAddress,
        /// App for given network exists.
        AppAlreadyExists,
        /// Destination account is not set.
        DestAccountIsNotSet,
        /// Call encoding failed.
        CallEncodeFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Users should burn their holdings to release funds on the Ethereum side
        #[transactional]
        #[pallet::weight(<T as Config>::WeightInfo::burn())]

        pub fn burn(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            channel_id: ChannelId,
            recipient: H160,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (target, asset_id) =
                Addresses::<T>::get(network_id).ok_or(Error::<T>::AppIsNotRegistered)?;

            T::Currency::withdraw(asset_id, &who, amount)?;

            let message = OutboundPayload::<T> {
                sender: who.clone(),
                recipient: recipient.clone(),
                amount: amount.into(),
            };

            T::OutboundRouter::submit(
                network_id,
                channel_id,
                &RawOrigin::Signed(who.clone()),
                target,
                100000u64.into(),
                &message.encode().map_err(|_| Error::<T>::CallEncodeFailed)?,
            )?;
            Self::deposit_event(Event::Burned(network_id, who, recipient, amount.into()));

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::mint())]

        pub fn mint(
            origin: OriginFor<T>,
            sender: H160,
            recipient: <T::Lookup as StaticLookup>::Source,
            amount: U256,
        ) -> DispatchResult {
            let (network_id, who) = T::CallOrigin::ensure_origin(origin)?;
            let (contract, asset_id) =
                Addresses::<T>::get(network_id).ok_or(Error::<T>::AppIsNotRegistered)?;
            ensure!(who == contract, Error::<T>::InvalidAppAddress);

            let amount: BalanceOf<T> = amount.as_u128().into();
            let recipient = T::Lookup::lookup(recipient)?;
            T::Currency::deposit(asset_id, &recipient, amount)?;
            Self::deposit_event(Event::Minted(network_id, sender, recipient.clone(), amount));

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_network())]

        pub fn register_network(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            name: AssetName,
            symbol: AssetSymbol,
            contract: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !Addresses::<T>::contains_key(network_id),
                Error::<T>::AppAlreadyExists
            );
            let bridge_account = Self::bridge_account()?;
            let asset_id = assets::Pallet::<T>::register_from(
                &bridge_account,
                symbol,
                name,
                DEFAULT_BALANCE_PRECISION,
                Balance::from(0u32),
                true,
                None,
                None,
            )?;
            Self::register_network_inner(network_id, asset_id, contract)?;
            Ok(().into())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_network())]

        pub fn register_network_with_existing_asset(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            asset_id: AssetIdOf<T>,
            contract: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !Addresses::<T>::contains_key(network_id),
                Error::<T>::AppAlreadyExists
            );
            Self::register_network_inner(network_id, asset_id, contract)?;
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn bridge_account() -> Result<T::AccountId, DispatchError> {
            Ok(technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::BridgeTechAccountId::get(),
            )?)
        }

        fn register_network_inner(
            network_id: EthNetworkId,
            asset_id: AssetIdOf<T>,
            contract: H160,
        ) -> DispatchResult {
            Addresses::<T>::insert(network_id, (contract, asset_id));
            let bridge_account = Self::bridge_account()?;
            let scope = permissions::Scope::Limited(common::hash(&asset_id));
            for permission_id in [permissions::BURN, permissions::MINT] {
                if permissions::Pallet::<T>::check_permission_with_scope(
                    bridge_account.clone(),
                    permission_id,
                    &scope,
                )
                .is_err()
                {
                    permissions::Pallet::<T>::assign_permission(
                        bridge_account.clone(),
                        &bridge_account,
                        permission_id,
                        scope,
                    )?;
                }
            }
            Ok(())
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub networks: Vec<(EthNetworkId, H160, T::AssetId)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                networks: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (network_id, contract, asset_id) in &self.networks {
                Pallet::<T>::register_network_inner(*network_id, *asset_id, *contract).unwrap();
            }
        }
    }
}
