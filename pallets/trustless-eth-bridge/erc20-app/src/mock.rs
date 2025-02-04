use bridge_types::traits::AppRegistry;
use currencies::BasicCurrencyAdapter;
use sp_std::marker::PhantomData;

// Mock runtime
use bridge_types::types::{AssetKind, ChannelId};
use bridge_types::{EthNetworkId, U256};
use common::mock::ExistentialDeposits;
use common::{
    balance, Amount, AssetId32, AssetName, AssetSymbol, Balance, DEXId, FromGenericPair,
    PredefinedAssetId, DAI, ETH, PSWAP, VAL, XOR, XST,
};
use frame_support::dispatch::DispatchResult;
use frame_support::parameter_types;
use frame_support::traits::{Everything, GenesisBuild};
use frame_system as system;
use hex_literal::hex;
use sp_core::{H160, H256};
use sp_keyring::sr25519::Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{
    BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Keccak256, Verify,
};
use sp_runtime::MultiSignature;
use system::RawOrigin;

use crate as erc20_app;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type AssetId = AssetId32<common::PredefinedAssetId>;

parameter_types! {
    pub GetTrustlessBridgeTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetTrustlessBridgeAccountId: AccountId = {
        let tech_account_id = GetTrustlessBridgeTechAccountId::get();
        let account_id =
            technical::Pallet::<Test>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetTrustlessBridgeFeesTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_FEES.to_vec(),
        );
        tech_account_id
    };
    pub GetTrustlessBridgeFeesAccountId: AccountId = {
        let tech_account_id = GetTrustlessBridgeFeesTechAccountId::get();
        let account_id =
            technical::Pallet::<Test>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
}

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Assets: assets::{Pallet, Call, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        Permissions: permissions::{Pallet, Call, Config<T>, Storage, Event<T>},
        Technical: technical::{Pallet, Call, Config<T>, Event<T>},
        Dispatch: dispatch::{Pallet, Call, Storage, Origin, Event<T>},
        BasicOutboundChannel: basic_channel::outbound::{Pallet, Storage, Event<T>, Config<T>},
        IncentivizedOutboundChannel: incentivized_channel::outbound::{Pallet, Config<T>, Storage, Event<T>},
        Erc20App: erc20_app::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub const BASE_NETWORK_ID: EthNetworkId = EthNetworkId::zero();

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<65536>;
}

impl common::Config for Test {
    type DEXId = common::DEXId;
    type LstId = common::LiquiditySourceType;
}

impl permissions::Config for Test {
    type Event = Event;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 0;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

impl tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Test as assets::Config>::AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
    type DustRemovalWhitelist = Everything;
}

impl currencies::Config for Test {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, u64>;
    type GetNativeCurrencyId = <Test as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}
parameter_types! {
    pub const GetBaseAssetId: AssetId = XOR;
}

parameter_types! {
    pub const GetBuyBackAssetId: AssetId = XST;
    pub GetBuyBackSupplyAssets: Vec<AssetId> = vec![VAL, PSWAP];
    pub const GetBuyBackPercentage: u8 = 10;
    pub const GetBuyBackAccountId: AccountId = AccountId::new(hex!(
            "0000000000000000000000000000000000000000000000000000000000000023"
    ));
    pub const GetBuyBackDexId: DEXId = DEXId::Polkaswap;
}

impl assets::Config for Test {
    type Event = Event;
    type ExtraAccountId = [u8; 32];
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<DEXId, common::LiquiditySourceType, [u8; 32]>;
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type GetBuyBackAssetId = GetBuyBackAssetId;
    type GetBuyBackSupplyAssets = GetBuyBackSupplyAssets;
    type GetBuyBackPercentage = GetBuyBackPercentage;
    type GetBuyBackAccountId = GetBuyBackAccountId;
    type GetBuyBackDexId = GetBuyBackDexId;
    type BuyBackLiquidityProxy = ();
    type Currency = currencies::Pallet<Test>;
    type WeightInfo = ();
    type GetTotalBalance = ();
}

impl dispatch::Config for Test {
    type Origin = Origin;
    type Event = Event;
    type MessageId = u64;
    type Call = Call;
    type CallFilter = Everything;
}

const INDEXING_PREFIX: &'static [u8] = b"commitment";

pub struct OutboundRouter<T>(PhantomData<T>);

impl<T> bridge_types::traits::OutboundRouter<T::AccountId> for OutboundRouter<T>
where
    T: basic_channel::outbound::Config + incentivized_channel::outbound::Config,
{
    fn submit(
        network_id: bridge_types::EthNetworkId,
        channel_id: ChannelId,
        who: &RawOrigin<T::AccountId>,
        target: H160,
        max_gas: U256,
        payload: &[u8],
    ) -> DispatchResult {
        match channel_id {
            ChannelId::Basic => {
                basic_channel::outbound::Pallet::<T>::submit(who, network_id, target, payload)
            }
            ChannelId::Incentivized => incentivized_channel::outbound::Pallet::<T>::submit(
                who, network_id, target, max_gas, payload,
            ),
        }
    }
}

parameter_types! {
    pub const MaxMessagePayloadSize: u64 = 2048;
    pub const MaxMessagesPerCommit: u64 = 3;
    pub const MaxTotalGasLimit: u64 = 5_000_000;
    pub const Decimals: u32 = 12;
}

impl basic_channel::outbound::Config for Test {
    const INDEXING_PREFIX: &'static [u8] = INDEXING_PREFIX;
    type Event = Event;
    type Hashing = Keccak256;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerCommit = MaxMessagesPerCommit;
    type WeightInfo = ();
}

pub struct FeeConverter;
impl Convert<U256, Balance> for FeeConverter {
    fn convert(amount: U256) -> Balance {
        common::eth::unwrap_balance(amount, Decimals::get())
            .expect("Should not panic unless runtime is misconfigured")
    }
}

parameter_types! {
    pub const FeeCurrency: AssetId32<PredefinedAssetId> = XOR;
}

impl incentivized_channel::outbound::Config for Test {
    const INDEXING_PREFIX: &'static [u8] = INDEXING_PREFIX;
    type Event = Event;
    type Hashing = Keccak256;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerCommit = MaxMessagesPerCommit;
    type MaxTotalGasLimit = MaxTotalGasLimit;
    type FeeTechAccountId = GetTrustlessBridgeFeesTechAccountId;
    type FeeCurrency = FeeCurrency;
    type WeightInfo = ();
}

pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
pub type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;

impl technical::Config for Test {
    type Event = Event;
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction = ();
}

pub struct AppRegistryImpl;

impl AppRegistry for AppRegistryImpl {
    fn register_app(_network_id: EthNetworkId, _app: H160) -> DispatchResult {
        Ok(())
    }

    fn deregister_app(_network_id: EthNetworkId, _app: H160) -> DispatchResult {
        Ok(())
    }
}

impl erc20_app::Config for Test {
    type Event = Event;
    type OutboundRouter = OutboundRouter<Test>;
    type CallOrigin = dispatch::EnsureEthereumAccount;
    type BridgeTechAccountId = GetTrustlessBridgeTechAccountId;
    type WeightInfo = ();
    type AppRegistry = AppRegistryImpl;
}

pub fn new_tester() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    technical::GenesisConfig::<Test> {
        register_tech_accounts: vec![
            (
                GetTrustlessBridgeAccountId::get(),
                GetTrustlessBridgeTechAccountId::get(),
            ),
            (
                GetTrustlessBridgeFeesAccountId::get(),
                GetTrustlessBridgeFeesTechAccountId::get(),
            ),
        ],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let bob: AccountId = Keyring::Bob.into();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bob.clone(), balance!(1))],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    assets::GenesisConfig::<Test> {
        endowed_assets: vec![
            (
                XOR.into(),
                bob.clone(),
                AssetSymbol(b"XOR".to_vec()),
                AssetName(b"SORA".to_vec()),
                18,
                0,
                true,
                None,
                None,
            ),
            (
                DAI.into(),
                bob.clone(),
                AssetSymbol(b"DAI".to_vec()),
                AssetName(b"DAI".to_vec()),
                18,
                0,
                true,
                None,
                None,
            ),
            (
                ETH.into(),
                bob,
                AssetSymbol(b"ETH".to_vec()),
                AssetName(b"ETH".to_vec()),
                18,
                0,
                true,
                None,
                None,
            ),
        ],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &incentivized_channel::outbound::GenesisConfig {
            fee: 10000,
            interval: 10,
        },
        &mut storage,
    )
    .unwrap();
    GenesisBuild::<Test>::assimilate_storage(
        &basic_channel::outbound::GenesisConfig {
            networks: vec![(BASE_NETWORK_ID, vec![Keyring::Bob.into()])],
            interval: 10,
        },
        &mut storage,
    )
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &erc20_app::GenesisConfig {
            apps: vec![
                (BASE_NETWORK_ID, H160::repeat_byte(1), AssetKind::Sidechain),
                (BASE_NETWORK_ID, H160::repeat_byte(2), AssetKind::Thischain),
            ],
            assets: vec![
                (
                    BASE_NETWORK_ID,
                    XOR,
                    H160::repeat_byte(3),
                    AssetKind::Thischain,
                ),
                (
                    BASE_NETWORK_ID,
                    DAI,
                    H160::repeat_byte(4),
                    AssetKind::Sidechain,
                ),
            ],
        },
        &mut storage,
    )
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
