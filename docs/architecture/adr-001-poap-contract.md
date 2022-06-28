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
POAPs are minted as NFTs in order to celebrate and record the attendance of an event.
Usually POAPs are given out for free to event attendees, which serve as verifiable proof that the holders attended a
specific event. POAP offers a new way for event organisers to engage and interact with their community.
POAP holders can later join additional airdrops, community polls, raffles and leaderboards.

## Specification

We will create a smart-contract that allows a selected `minter` (ideally another smart contract) to create a POAP
collection associated with a specific event (virtual or physical). The contract will implement the `CW721-base`, the basic implementation of
NFTs in CosmWasm. The `CW721-base` will take care of store all the information related to associations between POAPs and attendees
and POAPs with metadata. The `POAP-contract` will store events information and handle the `Mint` operations.

### Messages

#### Instantiate
The message to instantiate the conteact is the following:

```rust
pub struct InstantiateMsg {
pub cw721_code_id: u64,
pub cw721_instantiate_msg: Cw721InstantiateMsg,
pub event_info: EventInfo,
}
```

* The `cw721_code_id` refers to a previously uploaded `CW721-base` contract on the chain;
* The `cw721_instantiate_msg` will be used to instantiate the contract identified by the previous `cw721_code_id` field;
* The `event_info` will contains the necessary information to store the event inside the contract.

#### Cw721InstantiateMsg
The following message instantiate the [CW721-base contract](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base):
```rust
pub struct Cw721InstatiateMsg {
  /// Name of the NFT contract
  pub name: String,
  /// Symbol of the NFT contract
  pub symbol: String,

  /// The minter is the only one who can create new NFTs.
  /// This is designed for a base NFT that is controlled by an external program
  /// or contract. You will likely replace this with custom logic in custom NFTs
  pub minter: String,
}
```

* The `name` identifies the event name;
* The `symbol` identifies the event symbol (if exists);
* The `minter` field should be initialised as the `POAP-contract` address.

##### EventInfo
The `EventInfo` will be used as the way to instantiate the contract state with the information of an event from which later
mint POAPs.

```rust
pub struct EventInfo {
    pub creator: Addr,
    pub admin: Addr,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub base_poap_uri: String,
    pub event_uri: String,
    pub cw721_code_id: u64,
}
```

* The `base_poap_uri` field will be an `IPFS` valid URI corresponding to where the assets and metadata of the POAP will be stored.
  * The `metadata` file is a `.json` that follow the `ERC-721` [metadata standard](https://docs.opensea.io/docs/metadata-standards#metadata-structure)
  * The field will be used to initialize the `token_uri` field of the `cw721-base` [`MintMsg<T>`](https://github.com/CosmWasm/cw-nfts/blob/1e992ccf640f07a384d6442625d6780a8e48ef1e/contracts/cw721-base/src/msg.rs#L61)
* The `event_uri` field will be used to initialize the `extension` part of the `cw721-base` [`MintMsg<T>`](https://github.com/CosmWasm/cw-nfts/blob/1e992ccf640f07a384d6442625d6780a8e48ef1e/contracts/cw721-base/src/msg.rs#L61) and will contain a `.json` file
 with the following event's metadata:

```json
{
  "name": "My awesome event",
  "description": "Brief description of the event",
  "city": "city where the event will be taking place",
  "country": "country where the event will be taking place",
  "start_date": "Start date for your event",
  "end_date": "End date for your event",
  "expiry_date": "From this day onward, no attendees will be able to mint POAPs from your event",
  "year": 2022,
  "event_url": "Event URL",
  "virtual_event": true or false,
  "logo": "Logo of your event, 500x500, under 4MB",
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
    EventInfo {},
}
```

## References

- https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md
- https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base
