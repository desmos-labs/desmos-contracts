# ADR 001: POAP contract

## Changelog

- June 27, 2022: Initial draft;
-
## Status
DRAFTED

## Abstract
This ADR defines the architecture of a [**POAP**](https://academy.binance.com/en/glossary/proof-of-attendance-protocol-poap) generator contract.

## Context
Proof of Attendance Protocol (POAP) is a protocol to create digital badges or collectibles on the blockchain.
POAPs are mintend as NFTs in order to celebrate and record the attendance of an event.
Usually POAPs are given out for free to event attendees, which serve as verifiable proof that the holders attended a
specific event. POAP offers a new way for event organisers to engage and interact with their community.
POAP holders can later join additional airdrops, community polls, raffles and leaderboards.

## Specification

We will create a smart-contract named `poap-contract` that allows a `minter` to create a POAP collection for a specific event.
The contract will be an extention of the CW721 base, the basic implementation of NFTs in cosmWasm, so ownership, transfer and allowances
will be handled by default.

### Messages

#### Instantiate
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
pub cw721_code_id: u64,
pub cw721_instantiate_msg: Cw721InstantiateMsg,
pub event_info: EventInfo,
}
```

##### EventInfo
```rust
pub struct EventInfo {
    pub event_id: String,
    pub creator: Addr,
    pub admin: Addr,
    pub description: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub base_poap_uri: String,
    pub cw721_code_id: u64,
}
```

#### Execute
```rust
pub enum ExecuteMsg {
  Mint{},
  UpdateStartTime(Timestamp),
  MintTo{recipient: String}
}
```

#### Query
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
    EventInfo {
      event_id: String,
    },
}
```

## References

- https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md
- https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base
