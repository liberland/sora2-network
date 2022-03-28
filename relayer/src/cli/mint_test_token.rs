use crate::cli::prelude::*;
use bridge_types::H160;
use ethers::prelude::Middleware;

#[derive(Args, Clone, Debug)]
pub(super) struct Command {
    #[clap(flatten)]
    url: EthereumUrl,
    #[clap(flatten)]
    key: EthereumKey,
    #[clap(long)]
    token: H160,
    #[clap(long, short)]
    amount: u128,
    #[clap(long)]
    dry_run: bool,
}

impl Command {
    pub(super) async fn run(&self) -> AnyResult<()> {
        let eth = EthUnsignedClient::new(self.url.get()).await?;
        let key = self.key.get_key_string()?;
        let eth = eth.sign_with_string(&key).await?;
        let token = ethereum_gen::TestToken::new(self.token, eth.inner());
        let balance = token.balance_of(eth.address()).call().await?;
        let name = token.name().call().await?;
        let symbol = token.symbol().call().await?;
        info!(
            "Current token {}({}) balance: {}",
            name,
            symbol,
            balance.as_u128()
        );
        let mut call = token.mint(eth.address(), self.amount.into()).legacy();
        eth.inner()
            .fill_transaction(&mut call.tx, call.block)
            .await?;
        debug!("Check {:?}", call);
        call.call().await?;
        eth.save_gas_price(&call, "mint-test-token").await?;
        if !self.dry_run {
            debug!("Send");
            let tx = call.send().await?.confirmations(3).await?.unwrap();
            debug!("Tx: {:?}", tx);
        }
        Ok(())
    }
}
