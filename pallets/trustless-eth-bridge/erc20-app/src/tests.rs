use crate::mock::{new_tester, AccountId, Erc20App, Event, Origin, System, Test, BASE_NETWORK_ID};
use crate::{AppAddresses, AssetKinds, AssetsByAddresses, TokenAddresses};
use bridge_types::types::{AssetKind, ChannelId};
use common::{balance, AssetName, AssetSymbol, ETH, XOR};
use frame_support::assert_noop;
use frame_support::assert_ok;
use sp_core::H160;
use sp_keyring::AccountKeyring as Keyring;
use traits::MultiCurrency;

fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}

#[test]
fn mints_after_handling_ethereum_event() {
    new_tester().execute_with(|| {
        let peer_contract = H160::repeat_byte(2);
        let asset_id = XOR;
        let token = TokenAddresses::<Test>::get(BASE_NETWORK_ID, asset_id).unwrap();
        let sender = H160::repeat_byte(3);
        let recipient: AccountId = Keyring::Charlie.into();
        let bob: AccountId = Keyring::Bob.into();
        let amount = balance!(10);

        <Test as assets::Config>::Currency::deposit(asset_id, &bob, balance!(500)).unwrap();
        assert_ok!(Erc20App::burn(
            Origin::signed(bob.clone()),
            BASE_NETWORK_ID,
            ChannelId::Incentivized,
            asset_id,
            H160::repeat_byte(9),
            amount
        ));

        assert_ok!(Erc20App::mint(
            dispatch::RawOrigin(BASE_NETWORK_ID, peer_contract).into(),
            token,
            sender,
            recipient.clone(),
            amount.into(),
        ));
        assert_eq!(
            <Test as assets::Config>::Currency::total_balance(asset_id, &recipient),
            amount.into()
        );

        assert_eq!(
            Event::Erc20App(crate::Event::<Test>::Minted(
                BASE_NETWORK_ID,
                asset_id,
                sender,
                recipient,
                amount.into()
            )),
            last_event()
        );
    });
}

#[test]
fn burn_should_emit_bridge_event() {
    new_tester().execute_with(|| {
        let asset_id = XOR;
        let recipient = H160::repeat_byte(2);
        let bob: AccountId = Keyring::Bob.into();
        let amount = balance!(20);
        <Test as assets::Config>::Currency::deposit(asset_id, &bob, balance!(500)).unwrap();

        assert_ok!(Erc20App::burn(
            Origin::signed(bob.clone()),
            BASE_NETWORK_ID,
            ChannelId::Incentivized,
            asset_id,
            recipient.clone(),
            amount
        ));

        assert_eq!(
            Event::Erc20App(crate::Event::<Test>::Burned(
                BASE_NETWORK_ID,
                asset_id,
                bob,
                recipient,
                amount
            )),
            last_event()
        );
    });
}

#[test]
fn should_not_burn_on_commitment_failure() {
    new_tester().execute_with(|| {
        let asset_id = XOR;
        let sender: AccountId = Keyring::Bob.into();
        let recipient = H160::repeat_byte(9);
        let amount = balance!(20);

        <Test as assets::Config>::Currency::deposit(asset_id, &sender, balance!(500)).unwrap();

        for _ in 0..3 {
            let _ = Erc20App::burn(
                Origin::signed(sender.clone()),
                BASE_NETWORK_ID,
                ChannelId::Incentivized,
                asset_id,
                recipient.clone(),
                amount,
            )
            .unwrap();
        }

        assert_noop!(
            Erc20App::burn(
                Origin::signed(sender.clone()),
                BASE_NETWORK_ID,
                ChannelId::Incentivized,
                asset_id,
                recipient.clone(),
                amount,
            ),
            incentivized_channel::outbound::Error::<Test>::QueueSizeLimitReached
        );
        // let call = crate::mock::Call::Erc20App(crate::Call::<Test>::burn {
        //     network_id: BASE_NETWORK_ID,
        //     channel_id: ChannelId::Incentivized,
        //     asset_id,
        //     recipient: recipient.clone(),
        //     amount,
        // });

        // assert_noop!(
        //     call.dispatch(Origin::signed(sender.clone())),
        //     incentivized_channel::outbound::Error::<Test>::QueueSizeLimitReached
        // );
    });
}

#[test]
fn test_register_asset_internal() {
    new_tester().execute_with(|| {
        let asset_id = ETH;
        let who = AppAddresses::<Test>::get(BASE_NETWORK_ID, AssetKind::Thischain).unwrap();
        let origin = dispatch::RawOrigin(BASE_NETWORK_ID, who);
        let address = H160::repeat_byte(98);
        assert!(!TokenAddresses::<Test>::contains_key(
            BASE_NETWORK_ID,
            asset_id
        ));
        Erc20App::register_asset_internal(origin.into(), asset_id, address).unwrap();
        assert_eq!(
            AssetKinds::<Test>::get(BASE_NETWORK_ID, asset_id),
            Some(AssetKind::Thischain)
        );
        assert!(TokenAddresses::<Test>::contains_key(
            BASE_NETWORK_ID,
            asset_id
        ));
    })
}

#[test]
fn test_register_erc20_asset() {
    new_tester().execute_with(|| {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID;
        assert!(!AssetsByAddresses::<Test>::contains_key(
            network_id, address
        ));
        Erc20App::register_erc20_asset(
            Origin::root(),
            network_id,
            address,
            AssetSymbol(b"ETH".to_vec()),
            AssetName(b"ETH".to_vec()),
        )
        .unwrap();
        assert!(AssetsByAddresses::<Test>::contains_key(network_id, address));
    })
}

#[test]
fn test_register_native_asset() {
    new_tester().execute_with(|| {
        let asset_id = ETH;
        let network_id = BASE_NETWORK_ID;
        assert!(!TokenAddresses::<Test>::contains_key(network_id, asset_id));
        Erc20App::register_native_asset(Origin::root(), network_id, asset_id).unwrap();
        assert!(!TokenAddresses::<Test>::contains_key(network_id, asset_id));
    })
}

#[test]
fn test_register_erc20_app() {
    new_tester().execute_with(|| {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID + 1;
        assert!(!AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Sidechain
        ));
        Erc20App::register_erc20_app(Origin::root(), network_id, address).unwrap();
        assert!(AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Sidechain
        ));
    })
}

#[test]
fn test_register_native_app() {
    new_tester().execute_with(|| {
        let address = H160::repeat_byte(98);
        let network_id = BASE_NETWORK_ID + 1;
        assert!(!AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Thischain
        ));
        Erc20App::register_native_app(Origin::root(), network_id, address).unwrap();
        assert!(AppAddresses::<Test>::contains_key(
            network_id,
            AssetKind::Thischain
        ));
    })
}
