// SPDX-License-Identifier: Apache License 2.0
pragma solidity =0.8.13;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

contract MasterToken is ERC20Burnable, Ownable {
    bytes32 public _sidechainAssetId;

    /**
     * @dev Constructor that gives the specified address all of existing tokens.
     */
    constructor(
        string memory name,
        string memory symbol,
        address beneficiary,
        uint256 supply,
        bytes32 sidechainAssetId
    ) ERC20(name, symbol) {
        _sidechainAssetId = sidechainAssetId;
        _mint(beneficiary, supply);
    }

    fallback() external {
        revert();
    }

    function mintTokens(address beneficiary, uint256 amount) public onlyOwner {
        _mint(beneficiary, amount);
    }
}
