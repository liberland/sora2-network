use alloc::string::ToString;

use codec::Decode;
use frame_benchmarking::benchmarks;
use frame_support::assert_ok;
use frame_support::traits::OnInitialize;
use frame_system::{EventRecord, RawOrigin};
use hex_literal::hex;
use sp_std::prelude::*;

use common::VAL;

use crate::{
    Balances, Config, Event, MigratedAccounts, Module, Pallet, PendingMultiSigAccounts, PublicKeys,
    Quorums,
};

fn alice<T: Config>() -> T::AccountId {
    let bytes = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    T::AccountId::decode(&mut &bytes[..]).unwrap_or_default()
}

fn bob<T: Config>() -> T::AccountId {
    let bytes = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27f");
    T::AccountId::decode(&mut &bytes[..]).unwrap_or_default()
}

// Adds `n` of unaccessible accounts and after adds 1 account that will be migrated
fn add_accounts<T: Config>(n: u32) {
    let unaccessible_account_id = "did_sora_d9bda3688c6f608ab15c@sora".to_string();
    for _i in 0..n {
        Balances::<T>::insert(&unaccessible_account_id, 0);
        PublicKeys::<T>::insert(
            &unaccessible_account_id,
            vec![(
                false,
                "D9BDA3688c6f608ab15c03a55b171da0413788a40a25722b4ae4d3672890bcd7".to_lowercase(),
            )],
        );
    }

    let account_id = "did_sora_balance@sora".to_string();
    Balances::<T>::insert(&account_id, 300);
    PublicKeys::<T>::insert(
        &account_id,
        vec![(
            false,
            "9A685d77BCd3f60e6cc1e91eedc7a48e11bbcf1a036b920f3bae0372a78a5432".to_lowercase(),
        )],
    );

    let multi_sig_account_id = "did_sora_multi_sig@sora".to_string();
    Balances::<T>::insert(&multi_sig_account_id, 1000);
    PublicKeys::<T>::insert(
        &multi_sig_account_id,
        vec![
            (
                false,
                "f7d89d39d48a67e4741a612de10650234f9148e84fe9e8b2a9fad322b0d8e5bc".to_lowercase(),
            ),
            (
                false,
                "f56b4880ed91a25b257144acab749f615855c4b1b6a5d7891e1a6cdd9fd695e9".to_lowercase(),
            ),
            (
                false,
                "57571ec82cff710143eba60c05d88de14a22799048137162d63c534a8b02dc20".to_lowercase(),
            ),
        ],
    );
    Quorums::<T>::insert(&multi_sig_account_id, 2);
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Module::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

benchmarks! {
    migrate {
        let n in 1 .. 1000 => add_accounts::<T>(n);
        let caller = alice::<T>();
        let caller_origin: <T as frame_system::Config>::Origin = RawOrigin::Signed(caller.clone()).into();
    }: {
        Pallet::<T>::migrate(
            caller_origin,
            "did_sora_balance@sora".to_string(),
            "9a685d77bcd3f60e6cc1e91eedc7a48e11bbcf1a036b920f3bae0372a78A5432".to_string(),
            "233896712f752760713539f56c92534ff8f4f290812e8f129Ce0b513b99cbdffcea95abeed68edd1b0a4e4b52877c13c26c6c89e5bb6bf023ac6c0f4f53c0c02".to_string())?;
    }
    verify {
        assert_last_event::<T>(Event::Migrated("did_sora_balance@sora".to_string(), caller).into())
    }

    on_initialize {
        let n in 1 .. 1000 => add_accounts::<T>(n);
        let alice = alice::<T>();
        let alice_origin: <T as frame_system::Config>::Origin = RawOrigin::Signed(alice.clone()).into();
        let iroha_address = "did_sora_multi_sig@sora".to_string();
        assert_ok!(Pallet::<T>::migrate(
            alice_origin,
            iroha_address.clone(),
            "f7d89d39d48a67e4741a612de10650234f9148e84fE9e8b2a9fad322b0d8e5bc".to_string(),
            "d5f6dcc6967aa05df71894dd2c253085b236026efC1c66d4b33ee88dda20fc751b516aef631d1f96919f8cba2e15334022e04ef6602298d6b9820daeefe13e03".to_string())
        );
        let bob = bob::<T>();
        let bob_origin: <T as frame_system::Config>::Origin = RawOrigin::Signed(bob.clone()).into();
        assert_ok!(Pallet::<T>::migrate(
            bob_origin,
            iroha_address.clone(),
            "f56b4880ed91a25b257144acab749f615855c4b1b6A5d7891e1a6cdd9fd695e9".to_string(),
            "5c0f4296175b9836baac7c2d92116c90961bb80f87C30e3e2e2b2d5819d0c278fa55d3f04793d7fbf19a78afeb8b52f17b5ba55bf7373e726723da7155cad70d".to_string())
        );
        let multi_account_of_2 = {
            let mut signatories = [alice, bob];
            signatories.sort();
            pallet_multisig::Module::<T>::multi_account_id(&signatories, 2)
        };
    }: {
        Pallet::<T>::on_initialize(crate::blocks_till_migration::<T>() + 1u32.into())
    }
    verify {
        assert!(MigratedAccounts::<T>::contains_key(&iroha_address));
        assert!(!PendingMultiSigAccounts::<T>::contains_key(&iroha_address));
        assert_eq!(assets::Pallet::<T>::free_balance(&VAL.into(), &multi_account_of_2).unwrap(), 1000);
    }
}

#[cfg(test)]
mod tests {
    use frame_support::assert_ok;

    use crate::mock::{self, Runtime};

    #[test]
    fn migrate() {
        mock::test_ext(false).execute_with(|| {
            assert_ok!(super::test_benchmark_migrate::<Runtime>());
            assert_ok!(super::test_benchmark_on_initialize::<Runtime>());
        });
    }
}
