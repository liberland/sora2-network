use std::path::PathBuf;

use super::error::*;
use crate::prelude::*;
use bridge_types::network_config::NetworkConfig;
use clap::*;

#[derive(Args, Debug)]
pub struct BaseArgs {
    #[clap(flatten)]
    pub sub: SubstrateUrl,
    #[clap(flatten)]
    pub subkey: SubstrateKey,
    #[clap(flatten)]
    pub eth: EthereumUrl,
    #[clap(flatten)]
    pub ethkey: EthereumKey,
}

#[derive(Clone, Debug)]
pub enum Network {
    Mainnet,
    Ropsten,
    Sepolia,
    Rinkeby,
    Goerli,
    Classic,
    Mordor,
    Custom { path: PathBuf },
    None,
}

const NETWORKS: [&str; 8] = [
    "mainnet", "ropsten", "sepolia", "rinkeby", "goerli", "classic", "mordor", "custom",
];

impl Args for Network {
    fn augment_args(app: App<'_>) -> App<'_> {
        let mut app = app;
        for network in NETWORKS.iter() {
            let mut arg = Arg::new(*network).long(network).required(false);
            if *network == "custom" {
                arg = arg.value_name("PATH").takes_value(true);
            }
            app = app.arg(arg);
        }
        app
    }

    fn augment_args_for_update(app: App<'_>) -> App<'_> {
        Self::augment_args(app)
    }
}

impl FromArgMatches for Network {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self> {
        let mut network = None;
        let mut occurrences = 0;
        for network_name in NETWORKS.iter() {
            if matches.is_present(network_name) {
                occurrences += 1;
                if occurrences > 1 {
                    return Err(Error::raw(
                        ErrorKind::ArgumentConflict,
                        "Only one network can be specified at a time",
                    ));
                }
                network = Some(match *network_name {
                    "mainnet" => Network::Mainnet,
                    "ropsten" => Network::Ropsten,
                    "sepolia" => Network::Sepolia,
                    "rinkeby" => Network::Rinkeby,
                    "goerli" => Network::Goerli,
                    "classic" => Network::Classic,
                    "mordor" => Network::Mordor,
                    "custom" => {
                        let path = matches.value_of(network_name).expect("required value");
                        Network::Custom {
                            path: PathBuf::from(path),
                        }
                    }
                    _ => unreachable!(),
                });
            }
        }
        Ok(network.unwrap_or(Network::None))
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<()> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl Network {
    pub fn config(&self) -> AnyResult<NetworkConfig> {
        let res = match self {
            Network::Mainnet => NetworkConfig::Mainnet,
            Network::Ropsten => NetworkConfig::Ropsten,
            Network::Sepolia => NetworkConfig::Sepolia,
            Network::Rinkeby => NetworkConfig::Rinkeby,
            Network::Goerli => NetworkConfig::Goerli,
            Network::Classic => NetworkConfig::Classic,
            Network::Mordor => NetworkConfig::Mordor,
            Network::Custom { path } => {
                let bytes = std::fs::read(path)?;
                serde_json::de::from_slice(&bytes)?
            }
            Network::None => {
                return Err(
                    Error::raw(ErrorKind::MissingRequiredArgument, "No network specified").into(),
                )
            }
        };
        Ok(res)
    }
}

impl BaseArgs {
    pub async fn get_unsigned_substrate(&self) -> AnyResult<SubUnsignedClient> {
        let sub = SubUnsignedClient::new(self.sub.get()?).await?;
        Ok(sub)
    }

    pub async fn get_signed_substrate(&self) -> AnyResult<SubSignedClient> {
        let sub = self
            .get_unsigned_substrate()
            .await?
            .try_sign_with(self.subkey.get_key_string()?.as_str())
            .await?;
        Ok(sub)
    }

    pub async fn get_unsigned_ethereum(&self) -> AnyResult<EthUnsignedClient> {
        let eth = EthUnsignedClient::new(self.eth.get()?).await?;
        Ok(eth)
    }

    pub async fn get_signed_ethereum(&self) -> AnyResult<EthSignedClient> {
        let eth = self
            .get_unsigned_ethereum()
            .await?
            .sign_with_string(self.ethkey.get_key_string()?.as_str())
            .await?;
        Ok(eth)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SubstrateKey {
    #[clap(long)]
    substrate_key: Option<String>,
    #[clap(long)]
    substrate_key_file: Option<String>,
}

impl SubstrateKey {
    pub fn get_key_string(&self) -> AnyResult<String> {
        match (&self.substrate_key, &self.substrate_key_file) {
            (Some(_), Some(_)) => Err(CliError::BothKeyTypesProvided.into()),
            (None, None) => Err(CliError::SubstrateKey.into()),
            (Some(key), _) => Ok(key.clone()),
            (_, Some(key_file)) => Ok(std::fs::read_to_string(key_file)?),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct EthereumKey {
    #[clap(long)]
    ethereum_key: Option<String>,
    #[clap(long)]
    ethereum_key_file: Option<String>,
}

impl EthereumKey {
    pub fn get_key_string(&self) -> AnyResult<String> {
        match (&self.ethereum_key, &self.ethereum_key_file) {
            (Some(_), Some(_)) => Err(CliError::BothKeyTypesProvided.into()),
            (None, None) => Err(CliError::EthereumKey.into()),
            (Some(key), _) => Ok(key.clone()),
            (_, Some(key_file)) => Ok(std::fs::read_to_string(key_file)?),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct SubstrateUrl {
    #[clap(long)]
    substrate_url: Option<String>,
}

impl SubstrateUrl {
    pub fn get(&self) -> AnyResult<String> {
        Ok(self
            .substrate_url
            .clone()
            .ok_or(CliError::SubstrateEndpoint)?)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EthereumUrl {
    #[clap(long)]
    ethereum_url: Option<Url>,
}

impl EthereumUrl {
    pub fn get(&self) -> AnyResult<Url> {
        Ok(self
            .ethereum_url
            .clone()
            .ok_or(CliError::EthereumEndpoint)?)
    }
}
