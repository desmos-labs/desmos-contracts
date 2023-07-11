# ADR 006: POAP v2 Contract

## Changelog

- Jul 11, 2023: Initial draft;

## Status
PROPOSED Not Implemented

## Abstract
This ADR defines the new architecture of a POAP v2 Smart Contract that will replace the current `cw721-poap`, `poap-manager`, and `poap` smart contracts.

## Context
The name POAP stands for Proof of Attendance Protocol, which is a simple NFT that attests to the fact that a user has participated in a particular event.

Currently, we have implemented the ability to create and manage POAPs with three smart contracts: `cw721-poap`, `poap-manager`, and `poap`. Although this division works and makes sense under the Single Responsibility Principle, there are several downsides to it. First, users can easily upload and instantiate multiple instances of each of those contracts, leading to an unnecessary amount of contracts on the chain. Second, it makes it harder for newcomers to understand the interactions between the three contracts. Lastly, having multiple smart contracts for something as simple as a POAP seems excessive, as ideally, a POAP should have minimal operations.

## Decision
The decision is to merge all POAP-related operations into a single smart contract called `poap`, replacing all existing POAP-related smart contracts.

## Specifications
Below are the specifications for the various messages that the `poap` contract should support.

### Messages

#### Instantiate
To instantiate this smart contract, the user needs to provide:

* `admin` address: Identifies the user that controls the contract.
* `poap_name`: Identifies the name of the minted NFTs.
* `poap_symbol`: Identifies the symbol of the minted NFTs.
* `poap_metadata_uri`: Identifies the URI where users can view the associated metadata for the POAPs, ideally following the ERC-721 metadata scheme in a JSON file.
* `poap_mint_limit_per_address`: Specifies the maximum number of POAPs that can be minted per address.
* `poap_mint_enabled`: Indicates whether users can mint the POAPs.
* `poap_is_transferable`: Specifies whether each POAP can be transferred from one user to another.
* `event_start_date`: Identifies the start date of the event associated with the POAP.
* `event_end_date`: Identifies the end date of the event associated with the POAP.

#### Execute
The `MsgExecute` should allow the following operations:

##### User operations
Operations allowed for any user:

1. Mint a new POAP if the minting conditions are met.
2. Transfer a POAP they own to another user if transferring is allowed.
3. Burn a POAP they own.

##### Admin operations
The admin of the contract should be allowed to perform the following:

1. Update the POAP-related limitations (transferability, mintability).
2. Update the associated event data.

## Consequences

### Positive
* By consolidating everything into a single smart contract, managing POAP instances becomes easier for users.
* Most open issues regarding the POAP system can be resolved, making it more versatile.

### Negative
* Users of the current POAP smart contracts may need to migrate to the new contract to access future support for new features.

### Neutral
(None known)

## References
* [Issue #182](https://github.com/desmos-labs/desmos-contracts/issues/182)
* [Issue #183](https://github.com/desmos-labs/desmos-contracts/issues/183)
* [Issue #184](https://github.com/desmos-labs/desmos-contracts/issues/184)
* [Issue #185](https://github.com/desmos-labs/desmos-contracts/issues/185)
* [Issue #186](https://github.com/desmos-labs/desmos-contracts/issues/186)
* [Issue #187](https://github.com/desmos-labs/desmos-contracts/issues/187)
* [Issue #188](https://github.com/desmos-labs/desmos-contracts/issues/188)
* [Issue #189](https://github.com/desmos-labs/desmos-contracts/issues/189)
