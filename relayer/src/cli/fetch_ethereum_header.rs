use super::*;
use crate::ethereum::make_header;
use crate::prelude::*;
use clap::*;
use ethers::prelude::*;

#[derive(Args, Clone, Debug)]
pub(super) struct Command {
    #[clap(long, short)]
    descendants_until_final: Option<usize>,
    #[clap(long, short)]
    number: Option<usize>,
}

impl Command {
    pub(super) async fn run(&self, args: &BaseArgs) -> AnyResult<()> {
        let client = args.get_unsigned_ethereum().await?;
        let number = match (self.descendants_until_final, self.number) {
            (Some(v), None) => {
                let latest_block = client
                    .get_block(BlockId::Number(BlockNumber::Latest))
                    .await?
                    .unwrap();
                let number = latest_block.number.unwrap() - U64::from(v);
                number
            }
            (None, Some(v)) => U64::from(v),
            _ => return Err(anyhow::anyhow!("Invalid arguments")),
        };
        let finalized_block = client
            .get_block(BlockId::Number(BlockNumber::Number(number)))
            .await?
            .unwrap();
        let expected_hash = finalized_block.hash.unwrap_or_default();
        let header = make_header(finalized_block);
        let hash = header.compute_hash();
        info!("Hash: {:?}", hash);
        info!("Expected: {:?}", expected_hash);
        let result = serde_json::to_string(&header)?;
        println!("{}", result);
        Ok(())
    }
}
