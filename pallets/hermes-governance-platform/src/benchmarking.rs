//! Hermes governance platform module benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use codec::Decode;
use common::{balance, FromGenericPair, CERES_ASSET_ID};
use frame_benchmarking::benchmarks;
use frame_support::assert_ok;
use frame_support::PalletId;
use frame_system::{EventRecord, RawOrigin};
use hex_literal::hex;
use sp_runtime::traits::AccountIdConversion;
use sp_std::prelude::*;

use crate::Pallet as HermesGovernancePlatform;
use assets::Pallet as Assets;
use technical::Pallet as Technical;

// Support Functions
fn alice<T: Config>() -> T::AccountId {
    let bytes = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    T::AccountId::decode(&mut &bytes[..]).unwrap()
}

#[allow(non_snake_case)]
pub fn AUTHORITY<T: frame_system::Config>() -> T::AccountId {
    let bytes = hex!("96ea3c9c0be7bbc7b0656a1983db5eed75210256891a9609012362e36815b132");
    AccountIdOf::<T>::decode(&mut &bytes[..]).expect("Failed to decode account ID")
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

benchmarks! {
    vote{
        let caller = alice::<T>();
        let poll_id = "Poll".to_string();
        let title = "Title".to_string();
        let description = "Description".to_string();
        let voting_option = 2;
        let hermes_amount = balance!(1000);
        let hermes_locked = pallet::MinimumHermesAmountForCreatingPoll::<T>::get();
        let poll_start_timestamp = Timestamp::<T>::get();
        let poll_end_timestamp = Timestamp::<T>::get() + (172800*1000u32).into();

        frame_system::Pallet::<T>::inc_providers(&caller);
        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let owner: T::AccountId = assets::AssetOwners::<T>::get::<T::AssetId>(CERES_ASSET_ID.clone().into()).unwrap();

        Assets::<T>::mint(
            RawOrigin::Signed(owner).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            hermes_amount
        ).unwrap();

        let hermes_poll_info = HermesPollInfo {
            creator: caller.clone(),
            hermes_locked,
            poll_start_timestamp,
            poll_end_timestamp,
            title,
            description,
            creator_hermes_withdrawn: false,
        };

        pallet::HermesPollData::<T>::insert(&poll_id, &hermes_poll_info);
    }: {
        let _ = HermesGovernancePlatform::<T>::vote(
            RawOrigin::Signed(caller.clone()).into(),
            poll_id.clone(),
            voting_option,
        ).unwrap();
    }
    verify{
        assert_last_event::<T>(Event::Voted(caller, poll_id, voting_option).into())
    }

    create_poll {
        let caller = alice::<T>();
        let title = "Title".to_string();
        let descripton = "Description".to_string();
        let poll_start_timestamp = Timestamp::<T>::get();
        let poll_end_timestamp = Timestamp::<T>::get() + (172800*1000u32).into();
        let hermes_amount = balance!(1000);
        let owner: T::AccountId = assets::AssetOwners::<T>::get::<T::AssetId>(CERES_ASSET_ID.clone().into()).unwrap();

        Assets::<T>::mint(
            RawOrigin::Signed(owner).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            hermes_amount
        ).unwrap();

    }: {
        let _ = HermesGovernancePlatform::<T>::create_poll(
            RawOrigin::Signed(caller.clone()).into(),
            poll_start_timestamp,
            poll_end_timestamp,
            title.clone(),
            descripton
        );
    }
    verify{
        assert_last_event::<T>(Event::Created(caller, title.clone(), poll_start_timestamp, poll_end_timestamp).into())
    }

    withdraw_funds_voter {
        let caller = alice::<T>();
        let poll_id = "Poll".to_string();
        let title = "Title".to_string();
        let description = "Description".to_string();
        let voting_option = 2;
        let number_of_hermes = balance!(1000);
        let hermes_locked = pallet::MinimumHermesAmountForCreatingPoll::<T>::get();
        let poll_start_timestamp = Timestamp::<T>::get();
        let poll_end_timestamp = Timestamp::<T>::get() + (172800*1000u32).into();
        let current_timestamp = Timestamp::<T>::get();
        let owner: T::AccountId = assets::AssetOwners::<T>::get::<T::AssetId>(CERES_ASSET_ID.clone().into()).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(owner.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            number_of_hermes
        );

        let hermes_poll_info = HermesPollInfo {
            creator: caller.clone(),
            hermes_locked,
            poll_start_timestamp,
            poll_end_timestamp,
            title,
            description,
            creator_hermes_withdrawn: false,
        };

        pallet::HermesPollData::<T>::insert(&poll_id, &hermes_poll_info);

        let _ = HermesGovernancePlatform::<T>::vote(
            RawOrigin::Signed(caller.clone()).into(),
            poll_id.clone(),
            voting_option,
        );

        let hermes_voting_info = pallet::HermesVotings::<T>::get(&poll_id, &caller);

        pallet_timestamp::Now::<T>::put(current_timestamp + (172801*1000u32).into());

    }: _(RawOrigin::Signed(caller.clone()), poll_id.clone())
    verify {
        assert_last_event::<T>(Event::VoterFundsWithdrawn(caller, hermes_voting_info.number_of_hermes).into())
    }

    withdraw_funds_creator {
        let caller = alice::<T>();
        let poll_id = "Poll".to_string();
        let title = "Title".to_string();
        let description = "Description".to_string();
        let voting_option = 2;
        let number_of_hermes = balance!(2000);
        let hermes_locked = pallet::MinimumHermesAmountForCreatingPoll::<T>::get();
        let poll_start_timestamp = Timestamp::<T>::get();
        let poll_end_timestamp = Timestamp::<T>::get() + (172800*1000u32).into();
        let owner: T::AccountId = assets::AssetOwners::<T>::get::<T::AssetId>(CERES_ASSET_ID.clone().into()).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(owner).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            number_of_hermes
        );

        let hermes_poll_info = HermesPollInfo {
            creator: caller.clone(),
            hermes_locked,
            poll_start_timestamp,
            poll_end_timestamp,
            title,
            description,
            creator_hermes_withdrawn: false,
        };

        pallet::HermesPollData::<T>::insert(&poll_id, &hermes_poll_info);

        let pallet_account: AccountIdOf<T> = PalletId(*b"hermsgov").into_account_truncating();
        assert_ok!(Assets::<T>::transfer_from(
            &CERES_ASSET_ID.into(),
            &caller,
            &pallet_account,
            hermes_locked,
        ));

        pallet_timestamp::Now::<T>::put(poll_start_timestamp + (172801*1000u32).into());

    }: _(RawOrigin::Signed(caller.clone()), poll_id.clone())
    verify {
        assert_last_event::<T>(Event::CreatorFundsWithdrawn(caller, hermes_locked).into())
    }

    change_min_hermes_for_voting {
        let caller = AUTHORITY::<T>();
        let hermes_amount = balance!(20);
    }: _(RawOrigin::Signed(caller.clone()), hermes_amount.clone())
    verify {
        assert_last_event::<T>(Event::MinimumHermesForVotingChanged(hermes_amount).into())
    }

    change_min_hermes_for_creating_poll {
        let caller = AUTHORITY::<T>();
        let hermes_amount = balance!(20);
    }: _(RawOrigin::Signed(caller.clone()), hermes_amount.clone())
    verify {
        assert_last_event::<T>(Event::MinimumHermesForCreatingPollChanged(hermes_amount).into())
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::ExtBuilder::default().build(),
        crate::mock::Runtime
    );
}
