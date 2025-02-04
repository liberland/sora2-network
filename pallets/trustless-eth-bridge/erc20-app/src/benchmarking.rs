//! ERC20App pallet benchmarking

use crate::*;
use bridge_types::types::{AssetKind, ChannelId};
use bridge_types::EthNetworkId;
use common::{balance, AssetId32, AssetName, AssetSymbol, PredefinedAssetId, DAI, ETH, XOR};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::{Get, UnfilteredDispatchable};
use frame_system::RawOrigin;
use sp_core::H160;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;
use traits::MultiCurrency;

pub const BASE_NETWORK_ID: EthNetworkId = EthNetworkId::zero();

benchmarks! {
    where_clause {where T: basic_channel::outbound::Config + incentivized_channel::outbound::Config, <T as frame_system::Config>::Origin: From<dispatch::RawOrigin>, T::AssetId: From<AssetId32<PredefinedAssetId>>}

    burn_basic_channel {
        let caller: T::AccountId = whitelisted_caller();
        let asset_id: T::AssetId = XOR.into();
        let recipient = H160::repeat_byte(2);
        let amount = balance!(500);

        basic_channel::outbound::Pallet::<T>::register_operator(RawOrigin::Root.into(), BASE_NETWORK_ID, caller.clone()).unwrap();

        <T as assets::Config>::Currency::deposit(asset_id.clone(), &caller, amount)?;

    }: burn(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID, ChannelId::Basic, asset_id.clone(), recipient, amount)
    verify {
        assert_eq!(assets::Pallet::<T>::free_balance(&asset_id, &caller).unwrap(), 0);
    }

    burn_incentivized_channel {
        let caller: T::AccountId = whitelisted_caller();
        let asset_id: T::AssetId = XOR.into();
        let recipient = H160::repeat_byte(2);
        let amount = balance!(500);

        let fee_asset = <T as incentivized_channel::outbound::Config>::FeeCurrency::get();

        // deposit enough money to cover fees
        <T as assets::Config>::Currency::deposit(fee_asset.clone(), &caller, incentivized_channel::outbound::Fee::<T>::get())?;
        <T as assets::Config>::Currency::deposit(asset_id.clone(), &caller, amount)?;
    }: burn(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID, ChannelId::Incentivized, asset_id.clone(), recipient, amount)
    verify {
        assert_eq!(assets::Pallet::<T>::free_balance(&asset_id, &caller).unwrap(), 0);
    }

    // Benchmark `mint` extrinsic under worst case conditions:
    // * `mint` successfully adds amount to recipient account
    mint {
        let asset_id: T::AssetId = DAI.into();
        let token = TokenAddresses::<T>::get(BASE_NETWORK_ID, &asset_id).unwrap();
        let asset_kind = AssetKinds::<T>::get(BASE_NETWORK_ID, &asset_id).unwrap();
        let caller = AppAddresses::<T>::get(BASE_NETWORK_ID, asset_kind).unwrap();
        let origin = dispatch::RawOrigin::from((BASE_NETWORK_ID, caller));

        let recipient: T::AccountId = account("recipient", 0, 0);
        let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
        let sender = H160::zero();
        let amount = balance!(500);

        let call = Call::<T>::mint { token, sender, recipient: recipient_lookup, amount: amount.into()};

    }: { call.dispatch_bypass_filter(origin.into())? }
    verify {
        assert_eq!(assets::Pallet::<T>::free_balance(&asset_id, &recipient).unwrap(), amount);
    }

    register_erc20_app {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID + 1;
        assert!(!AppAddresses::<T>::contains_key(network_id, AssetKind::Sidechain));
    }: _(RawOrigin::Root, network_id, address)
    verify {
        assert!(AppAddresses::<T>::contains_key(network_id, AssetKind::Sidechain));
    }

    register_native_app {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID + 1;
        assert!(!AppAddresses::<T>::contains_key(network_id, AssetKind::Thischain));
    }: _(RawOrigin::Root, network_id, address)
    verify {
        assert!(AppAddresses::<T>::contains_key(network_id, AssetKind::Thischain));
    }

    register_erc20_asset {
        let asset_id: T::AssetId = ETH.into();
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID;
        let symbol = AssetSymbol(b"ETH".to_vec());
        let name = AssetName(b"ETH".to_vec());
        assert!(!AssetsByAddresses::<T>::contains_key(network_id, address));
    }: _(RawOrigin::Root, network_id, address, symbol, name)
    verify {
        assert!(AssetsByAddresses::<T>::contains_key(network_id, address));
    }

    register_native_asset {
        let asset_id: T::AssetId = ETH.into();
        let network_id = BASE_NETWORK_ID;
    }: _(RawOrigin::Root, network_id, asset_id)
    verify {
    }

    register_asset_internal {
        let asset_id: T::AssetId = ETH.into();
        let who = AppAddresses::<T>::get(BASE_NETWORK_ID, AssetKind::Thischain).unwrap();
        let origin = dispatch::RawOrigin(BASE_NETWORK_ID, who);
        let address = H160::repeat_byte(98);
        assert!(!TokenAddresses::<T>::contains_key(BASE_NETWORK_ID, asset_id));
    }: _(origin, asset_id, address)
    verify {
        assert_eq!(AssetKinds::<T>::get(BASE_NETWORK_ID, asset_id), Some(AssetKind::Thischain));
        assert!(TokenAddresses::<T>::contains_key(BASE_NETWORK_ID, asset_id));
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_tester(), crate::mock::Test,);
}
