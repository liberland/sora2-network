use bridge_types::H160;

use crate::cli::prelude::*;
use crate::substrate::AccountId;

#[derive(Args, Clone, Debug)]
pub struct Command {
    #[clap(short, long)]
    network: u32,
    #[clap(short, long)]
    peers: Vec<AccountId>,
    #[clap(short, long)]
    contract: H160,
}

impl Command {
    pub(super) async fn run(&self, args: &BaseArgs) -> AnyResult<()> {
        let sub = args.get_signed_substrate().await?;

        sub.api()
            .tx()
            .eth_bridge()
            .register_bridge(false, self.contract, self.peers.clone())?
            .sign_and_submit_then_watch_default(&sub)
            .await?
            .wait_for_in_block()
            .await?
            .wait_for_success()
            .await?;
        Ok(())
    }
}
