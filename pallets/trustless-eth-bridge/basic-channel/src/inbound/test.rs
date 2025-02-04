use std::marker::PhantomData;

use super::*;

use frame_support::assert_noop;
use frame_support::dispatch::DispatchError;
use frame_support::traits::{Everything, GenesisBuild};
use frame_support::{assert_err, assert_ok, parameter_types};
use frame_system::RawOrigin;
use sp_core::{H160, H256, U256};
use sp_keyring::AccountKeyring as Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify};
use sp_runtime::{DispatchResult, MultiSignature};
use sp_std::convert::From;

use bridge_types::traits::{MessageDispatch, OutboundRouter};
use bridge_types::types::{Message, Proof};
use bridge_types::Log;

use hex_literal::hex;

use crate::inbound as basic_inbound_channel;
use crate::inbound::Error;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

const BASE_NETWORK_ID: EthNetworkId = EthNetworkId::zero();

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        BasicInboundChannel: basic_inbound_channel::{Pallet, Call, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<65536>;
}
// Mock verifier
pub struct MockVerifier;

impl Verifier for MockVerifier {
    fn verify(_: EthNetworkId, message: &Message) -> Result<Log, DispatchError> {
        let log: Log = rlp::decode(&message.data).unwrap();
        Ok(log)
    }

    fn initialize_storage(
        _network_id: EthNetworkId,
        _headers: Vec<bridge_types::Header>,
        _difficulty: u128,
        _descendants_until_final: u8,
    ) -> Result<(), &'static str> {
        Ok(())
    }
}

// Mock Dispatch
pub struct MockMessageDispatch;

impl MessageDispatch<Test, MessageId> for MockMessageDispatch {
    fn dispatch(_: EthNetworkId, _: H160, _: MessageId, _: &[u8]) {}

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_dispatch_event(_: MessageId) -> Option<<Test as frame_system::Config>::Event> {
        None
    }
}

impl basic_inbound_channel::Config for Test {
    type Event = Event;
    type Verifier = MockVerifier;
    type MessageDispatch = MockMessageDispatch;
    type WeightInfo = ();
    type OutboundRouter = MockOutboundRouter<Self::AccountId>;
}

pub struct MockOutboundRouter<AccountId>(PhantomData<AccountId>);

impl<AccountId> OutboundRouter<AccountId> for MockOutboundRouter<AccountId> {
    fn submit(
        _: EthNetworkId,
        channel: ChannelId,
        _: &RawOrigin<AccountId>,
        _: H160,
        _: U256,
        _: &[u8],
    ) -> DispatchResult {
        if channel == ChannelId::Incentivized {
            return Err(DispatchError::Other("some error!"));
        }
        Ok(())
    }
}

pub fn new_tester(source_channel: H160) -> sp_io::TestExternalities {
    new_tester_with_config(basic_inbound_channel::GenesisConfig {
        networks: vec![(BASE_NETWORK_ID, source_channel)],
    })
}

pub fn new_tester_with_config(
    config: basic_inbound_channel::GenesisConfig,
) -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    GenesisBuild::<Test>::assimilate_storage(&config, &mut storage).unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}

// The originating channel address for the messages below
const SOURCE_CHANNEL_ADDR: [u8; 20] = hex!["2d02f2234d0b6e35d8d8fd77705f535ace681327"];

// Ethereum Log:
//   address: 0x2d02f2234d0b6e35d8d8fd77705f535ace681327 (outbound channel contract)
//   topics: ...
//   data:
//     source: 0x8f5acf5f15d4c3d654a759b96bb674a236c8c0f3  (ETH bank contract)
//     nonce: 1
//     payload ...
const MESSAGE_DATA_0: [u8; 284] = hex!(
    "
	f90119942d02f2234d0b6e35d8d8fd77705f535ace681327e1a0779b38144a38
	cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e000000000
	00000000000000000a42cba2b7960a0ce216ade5d6a82574257023d800000000
	0000000000000000000000000000000000000000000000000000000100000000
	0000000000000000000000000000000000000000000000000000006000000000
	000000000000000000000000000000000000000000000000000000570c018213
	dae5f9c236beab905c8305cb159c5fa1aae500d43593c715fdd31c61141abd04
	a99fd6822c8558854ccde39a5684e7a56da27d0000d9e9ac2d78030000000000
	00000000000000000000000000000000000000000000000000000000
"
);

// Ethereum Log:
//   address: 0xe4ab635d0bdc5668b3fcb4eaee1dec587998f4af (outbound channel contract)
//   topics: ...
//   data:
//     source: 0x8f5acf5f15d4c3d654a759b96bb674a236c8c0f3  (ETH bank contract)
//     nonce: 1
//     payload ...
const MESSAGE_DATA_1: [u8; 284] = hex!(
    "
	f90119942d02f2234d0b6e35d8d8fd77705f535ace681327e1a0779b38144a38
	cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e000000000
	00000000000000000a42cba2b7960a0ce216ade5d6a82574257023d800000000
	0000000000000000000000000000000000000000000000000000000200000000
	0000000000000000000000000000000000000000000000000000006000000000
	000000000000000000000000000000000000000000000000000000570c018213
	dae5f9c236beab905c8305cb159c5fa1aae500d43593c715fdd31c61141abd04
	a99fd6822c8558854ccde39a5684e7a56da27d0000d9e9ac2d78030000000000
	00000000000000000000000000000000000000000000000000000000
"
);

#[test]
fn test_submit_with_invalid_source_channel() {
    new_tester(H160::zero()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message
        let message = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_noop!(
            BasicInboundChannel::submit(origin.clone(), BASE_NETWORK_ID, message.clone()),
            Error::<Test>::InvalidSourceChannel
        );
    });
}

#[test]
fn test_submit() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message 1
        let message_1 = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_ok!(BasicInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message_1
        ));
        let nonce: u64 = <ChannelNonces<Test>>::get(BASE_NETWORK_ID);
        assert_eq!(nonce, 1);

        // Submit message 2
        let message_2 = Message {
            data: MESSAGE_DATA_1.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_ok!(BasicInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message_2
        ));
        let nonce: u64 = <ChannelNonces<Test>>::get(BASE_NETWORK_ID);
        assert_eq!(nonce, 2);
    });
}

#[test]
fn test_submit_with_invalid_nonce() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message
        let message = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_ok!(BasicInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message.clone()
        ));
        let nonce: u64 = <ChannelNonces<Test>>::get(BASE_NETWORK_ID);
        assert_eq!(nonce, 1);

        // Submit the same again
        assert_noop!(
            BasicInboundChannel::submit(origin.clone(), BASE_NETWORK_ID, message.clone()),
            Error::<Test>::InvalidNonce
        );
    });
}

#[test]
fn test_submit_with_invalid_network_id() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = Origin::signed(relayer);

        // Submit message
        let message = Message {
            data: MESSAGE_DATA_0.into(),
            proof: Proof {
                block_hash: Default::default(),
                tx_index: Default::default(),
                data: Default::default(),
            },
        };
        assert_noop!(
            BasicInboundChannel::submit(origin.clone(), BASE_NETWORK_ID + 1, message.clone()),
            Error::<Test>::InvalidNetwork
        );
    });
}

#[test]
fn test_register_channel() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        assert_ok!(BasicInboundChannel::register_channel(
            Origin::root(),
            BASE_NETWORK_ID + 1,
            H160::from(SOURCE_CHANNEL_ADDR),
        ));

        assert_eq!(
            ChannelAddresses::<Test>::get(BASE_NETWORK_ID + 1),
            Some(H160::from(SOURCE_CHANNEL_ADDR))
        );
    });
}

#[test]
fn test_register_existing_channel() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        assert_noop!(
            BasicInboundChannel::register_channel(
                Origin::root(),
                BASE_NETWORK_ID,
                H160::from(SOURCE_CHANNEL_ADDR),
            ),
            Error::<Test>::ChannelExists
        );
    });
}

#[test]
fn test_register_app() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        assert_ok!(BasicInboundChannel::register_app(
            BASE_NETWORK_ID,
            H160::repeat_byte(7)
        ));
    })
}

#[test]
fn test_register_app_invalid_network() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        assert_err!(
            BasicInboundChannel::register_app(BASE_NETWORK_ID + 1, H160::repeat_byte(7)),
            Error::<Test>::InvalidNetwork
        );
    })
}

#[test]
fn test_deregister_app() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        assert_ok!(BasicInboundChannel::deregister_app(
            BASE_NETWORK_ID,
            H160::repeat_byte(7)
        ));
    })
}

#[test]
fn test_deregister_app_invalid_network() {
    new_tester(SOURCE_CHANNEL_ADDR.into()).execute_with(|| {
        assert_err!(
            BasicInboundChannel::deregister_app(BASE_NETWORK_ID + 1, H160::repeat_byte(7)),
            Error::<Test>::InvalidNetwork
        );
    })
}
