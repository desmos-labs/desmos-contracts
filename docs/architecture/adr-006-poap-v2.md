# ADR 006: POAP v2 Contract

## Changelog

- Jul 11, 2023: Initial draft;

## Status

PROPOSED Not Implemented

## Abstract

This ADR defines the new architecture of a POAP v2 Smart Contract that will replace the
current `cw721-poap`, `poap-manager`, and `poap` smart contracts.

## Context

The name POAP stands for Proof of Attendance Protocol, which is a simple NFT that attests to the fact that a user has
participated in a particular event.

Currently, we have implemented the ability to create and manage POAPs with three smart
contracts: `cw721-poap`, `poap-manager`, and `poap`. Although this division works and makes sense under the Single
Responsibility Principle, there are several downsides to it. First, users can easily upload and instantiate multiple
instances of each of those contracts, leading to an unnecessary amount of contracts on the chain. Second, it makes it
harder for newcomers to understand the interactions between the three contracts. Lastly, having multiple smart contracts
for something as simple as a POAP seems excessive, as ideally, a POAP should have minimal operations.

## Decision

The decision is to merge all POAP-related operations into a single smart contract called `poap`, replacing all existing
POAP-related smart contracts.

## Specifications

To ensure compatibility with the [`cw721`](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721) standard, we
will create a smart contract that extends
the [`cw721-base`](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base) contract and implements the
necessary interfaces. By doing this, we can enable the transfer of POAPs in the future, if desired by the
administrators, and ensure proper display in end clients. The following specifications outline the custom messages that
the `poap` contract should support.

### Messages

#### Instantiate

To instantiate this smart contract, the user needs to provide:

|      Variable       |  Type   | Description                                                                                                                                 |
|:-------------------:|:-------:|:--------------------------------------------------------------------------------------------------------------------------------------------|
|       `admin`       | Address | Identifies the user that controls the contract                                                                                              |
|     `poap_name`     | String  | Identifies the name of the minted NFTs                                                                                                      |
|    `poap_symbol`    | String  | Identifies the symbol of the minted NFTs                                                                                                    |
| `poap_metadata_uri` | String  | Identifies the URI where users can view the associated metadata for the POAPs, ideally following the ERC-721 metadata scheme in a JSON file |

The user can also provide the following:

|        Variable        |   Type    |  Default  | Description                                                                                                           |
|:----------------------:|:---------:|:---------:|:----------------------------------------------------------------------------------------------------------------------|
|        `minter`        |  Address  | Undefined | Additional address that is allowed to mint tokens on behalf of other users                                            | 
| `poap_is_transferable` |  Boolean  |  `False`  | Specifies whether each POAP can be transferred from one user to another                                               |
|   `poap_is_mintable`   |  Boolean  |  `True`   | Indicates whether users can mint the POAPs                                                                            |
| `poap_mint_start_time` | Timestamp | Undefined | Identifies the timestamp at which the minting of the POAP will be enabled. If not set, the minting is always enabled. |
|  `poap_mint_end_time`  | Timestamp | Undefined | Identifies the timestamp at which the minting of the POAP will be disabled. If not set, the minting will never end.   |

#### Execute

The `MsgExecute` should allow the following operations:

##### User operations

Operations allowed for any user:

1. Mint a new POAP if the minting conditions are met.
2. Transfer a POAP they own to another user if transferring is allowed.
3. Burn a POAP they own.

##### Minter operations

If set, the `minter` associated with the contract should be allowed to perform the following:

1. Mint POAPs for a single user (`mintToSingleUser`).
2. Mint POAPs for multiple users (`mintToManyUsers`).

> **Note**  
> The `minter` is not subject to the POAP minting conditions. This means that they act like an admin and can mint
> tokens for other users even if the minting of the POAP is currently disabled.

##### Admin operations

The admin of the contract should be allowed to perform the following:

1. Update the `minter` address.
2. Update the POAP-related limitations (transferability, mintability).

## Consequences

### Positive

* By consolidating everything into a single smart contract, managing POAP instances becomes easier for users.
* Most open issues regarding the POAP system can be resolved, making it more versatile.

### Negative

* Users of the current POAP smart contracts may need to migrate to the new contract to access future support for new
  features.

### Neutral

(None known)

## References

* [Cw721 Standard](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721)
* [Cw721 Base Contract](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base)
* [Issue #182](https://github.com/desmos-labs/desmos-contracts/issues/182)
* [Issue #183](https://github.com/desmos-labs/desmos-contracts/issues/183)
* [Issue #184](https://github.com/desmos-labs/desmos-contracts/issues/184)
* [Issue #185](https://github.com/desmos-labs/desmos-contracts/issues/185)
* [Issue #186](https://github.com/desmos-labs/desmos-contracts/issues/186)
* [Issue #187](https://github.com/desmos-labs/desmos-contracts/issues/187)
* [Issue #188](https://github.com/desmos-labs/desmos-contracts/issues/188)
* [Issue #189](https://github.com/desmos-labs/desmos-contracts/issues/189)
