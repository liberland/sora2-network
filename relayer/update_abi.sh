#!/bin/bash -v

jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/test/TestBasicInboundChannel.sol/TestBasicInboundChannel.json > ethereum-gen/src/bytes/BasicInboundChannel.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/test/TestBasicOutboundChannel.sol/TestBasicOutboundChannel.json > ethereum-gen/src/bytes/BasicOutboundChannel.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/test/TestIncentivizedInboundChannel.sol/TestIncentivizedInboundChannel.json > ethereum-gen/src/bytes/IncentivizedInboundChannel.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/test/TestIncentivizedOutboundChannel.sol/TestIncentivizedOutboundChannel.json > ethereum-gen/src/bytes/IncentivizedOutboundChannel.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/test/TestBeefyLightClient.sol/TestBeefyLightClient.json > ethereum-gen/src/bytes/BeefyLightClient.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/ValidatorRegistry.sol/ValidatorRegistry.json > ethereum-gen/src/bytes/ValidatorRegistry.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/ETHApp.sol/ETHApp.json > ethereum-gen/src/bytes/ETHApp.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/SidechainApp.sol/SidechainApp.json > ethereum-gen/src/bytes/SidechainApp.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/ERC20App.sol/ERC20App.json > ethereum-gen/src/bytes/ERC20App.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol/IERC20Metadata.json > ethereum-gen/src/bytes/IERC20Metadata.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/MigrationApp.sol/MigrationApp.json > ethereum-gen/src/bytes/MigrationApp.abi.json
jq ".abi" ../ethereum-bridge-contracts/artifacts/contracts/test/TestToken.sol/TestToken.json > ethereum-gen/src/bytes/TestToken.abi.json
