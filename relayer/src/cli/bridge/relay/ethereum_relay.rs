use crate::cli::BaseArgs;
use crate::ethereum::proof_loader::ProofLoader;
use crate::prelude::*;
use crate::relay::ethereum::Relay;
use crate::relay::ethereum_messages::SubstrateMessagesRelay;
use clap::*;
use std::path::PathBuf;

#[derive(Args, Clone, Debug)]
pub(crate) struct Command {
    #[clap(long)]
    base_path: PathBuf,
    #[clap(long)]
    disable_incentivized: bool,
    #[clap(long)]
    disable_basic: bool,
}

impl Command {
    pub async fn run(&self, args: &BaseArgs) -> AnyResult<()> {
        let eth = args.get_unsigned_ethereum().await?;
        let sub = args.get_signed_substrate().await?;
        let proof_loader = ProofLoader::new(eth.clone(), self.base_path.clone());
        let relay = Relay::new(sub.clone(), eth.clone(), proof_loader.clone()).await?;
        let messages_relay = SubstrateMessagesRelay::new(
            sub,
            eth,
            proof_loader,
            self.disable_basic,
            self.disable_incentivized,
        )
        .await?;
        tokio::try_join!(relay.run(), messages_relay.run())?;
        Ok(())
    }
}
