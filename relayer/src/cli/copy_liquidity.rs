use super::*;
use crate::prelude::*;
use clap::*;
use common::{DAI, PSWAP, VAL, XOR, XST, XSTUSD};

#[derive(Args, Clone, Debug)]
pub(super) struct Command {
    #[clap(flatten)]
    sub: SubstrateClient,
    #[clap(long)]
    mainnet_url: String,
}

impl Command {
    pub(super) async fn run(&self) -> AnyResult<()> {
        let sub = self.sub.get_signed_substrate().await?;
        let mainnet = SubUnsignedClient::<MainnetConfig>::new(self.mainnet_url.clone()).await?;
        for (dex_id, base) in [(0, XOR), (1, XSTUSD)] {
            for asset_id in [XOR, PSWAP, DAI, XSTUSD, VAL, XST] {
                let reserves = mainnet
                    .api()
                    .storage()
                    .fetch(
                        &runtime::storage().pool_xyk().reserves(&base, &asset_id),
                        None,
                    )
                    .await?
                    .unwrap();
                let current_reserves = sub
                    .api()
                    .storage()
                    .fetch(
                        &runtime::storage().pool_xyk().reserves(&base, &asset_id),
                        None,
                    )
                    .await?
                    .unwrap();
                if reserves.0 <= 1
                    || reserves.1 <= 1
                    || current_reserves.0 > 1
                    || current_reserves.1 > 1
                {
                    continue;
                }
                info!("Add liquidity {}-{}: {:?}", base, asset_id, reserves);
                info!("Mint {}: {}", base, reserves.0 as i128 * 2);
                sub.api()
                    .tx()
                    .sign_and_submit_then_watch_default(
                        &runtime::tx()
                        .sudo()
                        .sudo(
                            runtime::runtime_types::framenode_runtime::RuntimeCall::Currencies(
                                runtime::runtime_types::orml_currencies::module::Call::update_balance {
                                    who: sub.account_id(),
                                    currency_id: base,
                                    amount: reserves.0 as i128 * 2,
                                }
                            )
                        ),
                        &sub
                    )
                    .await?.wait_for_in_block()
                    .await?
                    .wait_for_success()
                    .await?;
                info!("Mint {}: {}", asset_id, reserves.1 as i128 * 2);
                sub.api()
                    .tx()
                    .sign_and_submit_then_watch_default(
                        &runtime::tx()
                        .sudo()
                        .sudo(
                            runtime::runtime_types::framenode_runtime::RuntimeCall::Currencies(
                                runtime::runtime_types::orml_currencies::module::Call::update_balance {
                                    who: sub.account_id(),
                                    currency_id: asset_id,
                                    amount: reserves.1 as i128 * 2,
                                }
                            )
                        ),
                        &sub
                    )
                    .await?
                    .wait_for_in_block()
                    .await?
                    .wait_for_success()
                    .await?;
                let tp = sub
                    .api()
                    .storage()
                    .fetch(
                        &runtime::storage().trading_pair().enabled_sources(
                            &dex_id,
                            &runtime::runtime_types::common::primitives::TradingPair {
                                base_asset_id: base,
                                target_asset_id: asset_id,
                            },
                        ),
                        None,
                    )
                    .await?;
                if tp.is_none() {
                    info!("Registering trading pair");
                    sub.api()
                        .tx()
                        .sign_and_submit_then_watch_default(
                            &runtime::tx()
                                .trading_pair()
                                .register(dex_id, base, asset_id),
                            &sub,
                        )
                        .await?
                        .wait_for_in_block()
                        .await?
                        .wait_for_success()
                        .await?;
                }
                info!("Initializing pool");
                sub.api()
                    .tx()
                    .sign_and_submit_then_watch_default(
                        &runtime::tx()
                            .pool_xyk()
                            .initialize_pool(dex_id, base, asset_id),
                        &sub,
                    )
                    .await?
                    .wait_for_in_block()
                    .await?
                    .wait_for_success()
                    .await?;
                info!("Deposit liquidity");
                sub.api()
                    .tx()
                    .sign_and_submit_then_watch_default(
                        &runtime::tx().pool_xyk().deposit_liquidity(
                            dex_id, base, asset_id, reserves.0, reserves.1, 1, 1,
                        ),
                        &sub,
                    )
                    .await?
                    .wait_for_in_block()
                    .await?
                    .wait_for_success()
                    .await?;
            }
        }
        Ok(())
    }
}
