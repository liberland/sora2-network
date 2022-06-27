use crate::cli::prelude::*;
use bridge_types::H160;
use ethers::prelude::builders::ContractCall;

#[derive(Args, Debug)]
pub(crate) struct Command {
    #[clap(flatten)]
    sub: SubstrateClient,
    #[clap(flatten)]
    eth: EthereumClient,
    #[clap(long)]
    eth_app: H160,
}

impl Command {
    pub(super) async fn run(&self) -> AnyResult<()> {
        println!("{}:{}", file!(), line!());
        let eth = self.eth.get_signed_ethereum().await?;
        println!("{}:{}", file!(), line!());
        let sub = self.sub.get_unsigned_substrate().await?;
        println!("{}:{}", file!(), line!());
        let block_hash = sub.block_hash(Some(1u32)).await?;
        println!("{}:{}", file!(), line!());
        let autorities = sub
            .api()
            .storage()
            .mmr_leaf()
            .beefy_next_authorities(false, Some(block_hash))
            .await?;
        println!("{}:{}", file!(), line!());
        let eth_app = ethereum_gen::ETHApp::new(self.eth_app.clone(), eth.inner());
        println!("{}:{}", file!(), line!());
        let (basic_inbound_address, basic_outbound_address) = eth_app.channels(0).call().await?;
        println!("{}:{}", file!(), line!());
        let (incentivized_inbound_address, incentivized_outbound_address) =
            eth_app.channels(1).call().await?;
        println!("{}:{}", file!(), line!());
        let basic_inbound =
            ethereum_gen::BasicInboundChannel::new(basic_inbound_address, eth.inner());
        println!("{}:{}", file!(), line!());
        let basic_outbound =
            ethereum_gen::BasicOutboundChannel::new(basic_outbound_address, eth.inner());
        println!("{}:{}", file!(), line!());
        let incentivized_inbound = ethereum_gen::IncentivizedInboundChannel::new(
            incentivized_inbound_address,
            eth.inner(),
        );
        println!("{}:{}", file!(), line!());
        let incentivized_outbound = ethereum_gen::IncentivizedOutboundChannel::new(
            incentivized_outbound_address,
            eth.inner(),
        );
        println!("{}:{}", file!(), line!());
        let beefy_address = basic_inbound.beefy_light_client().call().await?;
        println!("{}:{}", file!(), line!());
        let beefy = ethereum_gen::BeefyLightClient::new(beefy_address, eth.inner());
        println!("{}:{}", file!(), line!());
        let validator_registry_address = beefy.validator_registry().call().await?;
        println!("{}:{}", file!(), line!());
        let validator_registry =
            ethereum_gen::ValidatorRegistry::new(validator_registry_address, eth.inner());
        println!("{}:{}", file!(), line!());
        let registry_owner = validator_registry.owner().call().await?;
        println!("{}:{}", file!(), line!());
        if registry_owner == eth.address() {
            info!("Updating validator registry");
            let call: ContractCall<_, _> =
                validator_registry.update(autorities.root.0, autorities.len.into(), autorities.id);
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);

            info!("Transfer ownership of validator registry to Beefy");
            let call: ContractCall<_, _> = validator_registry.transfer_ownership(beefy_address);
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);
        } else if registry_owner == beefy_address && beefy.owner().call().await? == eth.address() {
            info!("Reset beefy contract");
            let call: ContractCall<_, _> =
                beefy.reset(0, autorities.root.0, autorities.len.into(), autorities.id);
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);

            info!("Reset incentivized inbound contract");
            let call: ContractCall<_, _> = incentivized_inbound.reset();
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);

            debug!("Reset incentivized outbound contract");
            let call: ContractCall<_, _> = incentivized_outbound.reset();
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);

            info!("Reset basic inbound contract");
            let call: ContractCall<_, _> = basic_inbound.reset();
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);

            debug!("Reset basic outbound contract");
            let call: ContractCall<_, _> = basic_outbound.reset();
            let call = call.legacy().from(eth.address());
            debug!("Static call: {:?}", call);
            call.call().await?;
            debug!("Send transaction");
            let pending = call.send().await?;
            debug!("Pending transaction: {:?}", pending);
            let result = pending.await?;
            debug!("Confirmed: {:?}", result);
        }
        Ok(())
    }
}
