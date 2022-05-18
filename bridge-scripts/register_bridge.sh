#!/usr/bin/bash -v

DEPLOYMENTS=${BASE_DIR:-ethereum-bridge-contracts}/.deployments/${NETWORK:-geth}
BASIC_OUTBOUND=$(jq '.address' $DEPLOYMENTS/BasicOutboundChannel.json | tr -d '"')
INCENTIVIZED_OUTBOUND=$(jq '.address' $DEPLOYMENTS/IncentivizedOutboundChannel.json | tr -d '"')
ETH_APP=$(jq '.address' $DEPLOYMENTS/ETHApp.json | tr -d '"')
SIDECHAIN_APP=$(jq '.address' $DEPLOYMENTS/SidechainApp.json | tr -d '"')
MIGRATION_APP=$(jq '.address' $DEPLOYMENTS/MigrationApp.json | tr -d '"')
ERC20_APP=$(jq '.address' $DEPLOYMENTS/ERC20App.json | tr -d '"')
echo "Use deployments from $DEPLOYMENTS"

cargo b --release --bin relayer

./target/release/relayer \
	bridge register \
	--ethereum-url ws://localhost:8546 \
	--substrate-url ws://localhost:9944 \
	--substrate-key //Alice \
	--incentivized-channel-outbound $INCENTIVIZED_OUTBOUND \
	--basic-channel-outbound $BASIC_OUTBOUND \
	--migration-app $MIGRATION_APP \
	--eth-app $ETH_APP \
	-d 10

sleep 60

cargo run --bin relayer --release -- \
	bridge register-app \
	--ethereum-url ws://localhost:8546 \
	--substrate-url ws://localhost:9944 \
	--substrate-key //Alice \
	--is-native \
	--contract $SIDECHAIN_APP

cargo run --bin relayer --release -- \
	bridge register-app \
	--ethereum-url ws://localhost:8546 \
	--substrate-url ws://localhost:9944 \
	--substrate-key //Alice \
	--contract $ERC20_APP