# ADR 001: POAP-Contract

## Changelog

- June 27, 2022: Initial draft;
- June 28, 2022: Second review;
- June 29, 2022: Third review;
- July 12, 2022: Fourth review.
- July 14, 2022: Fifth review;
- July 15, 2022: Sixth review;
- July 18, 2022: Accepted ADR.

## Status
ACCEPTED (Not implemented)

## Abstract
This ADR defines the architecture of a [**POAP**](https://academy.binance.com/en/glossary/proof-of-attendance-protocol-poap) generator contract.

## Context
Proof of Attendance Protocol (POAP) allows the creation of digital badges on the blockhain.
POAPs are NFTs given out for free to event participants and serve as verifiable proof that the holders attended a
specific event. POAPs offers a new way for event organisers to engage and interact with their community as
holders can later join additional airdrops, community polls, raffles and leaderboards.

## Decision

We will create a `POAP` contract that allows a selected `minter` (ideally another smart contract) to create a POAP
collection associated with a specific event. The contract will implement the `CW721-base`, the basic implementation of
NFTs in CosmWasm.
The `CW721-base` will take care of store all the information related to:
- Associations between POAPs and attendees;
- Associations between POAPs and metadata.
The `POAP` contract will store events information and handle the `Mint` operations.

## Specifications

### Messages

#### Instantiate
The message to instantiate the contract is the following:

```rust
pub struct InstantiateMsg {
pub cw721_code_id: u64,
pub cw721_instantiate_msg: Cw721InstantiateMsg,
pub event_info: EventInfo,
pub admin: String,
pub minter: String,
}
```

* The `cw721_code_id` refers to a previously uploaded `CW721-base` contract on the chain;
* The `cw721_instantiate_msg` contains the info to instantiate the `CW721-base`;
* The `event_info` contains the event info;
* The `admin` is the admin of the contract;
* The `minter` is the address of the user or contract that has the ability to mint.

#### Cw721InstantiateMsg
The following message instantiate the [CW721-base contract](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base):
```rust
pub struct Cw721InstatiateMsg {
  pub name: String,
  pub symbol: String,
  pub minter: String,
}
```

In the `POAP` contract case:
* The `name` identifies the event name;
* The `symbol` identifies the event logo (if exists);
* The `minter` identifies the `POAP` contract address. During the initialisation this field will be overwritten by the actual contract address.

##### EventInfo
The `EventInfo` are used to instantiate the contract state with the information of the event.

```rust
pub struct EventInfo {
    pub creator: Addr,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub per_address_limit: u32,
    pub base_poap_uri: String,
    pub event_uri: String,
    pub cw721_code_id: u64,
}
```

* The `creator` field identifies the address of the event's creator;
* The `start_time` identifies the start time of the event;
* The `end_time` identifies the end time of the event;
* The `per_address_limit` identifies the max num of tokens that can be minted by an address;
* The `base_poap_uri` identifies a valid `IPFS` URI corresponding to where the assets and metadata of the POAPs are stored.
  * The `metadata` file is a `.json` that follow the `ERC-721` [metadata standard](https://docs.opensea.io/docs/metadata-standards#metadata-structure)
* The field will be used to initialize the `token_uri` field of the `cw721-base` [`MintMsg<T>`](https://github.com/CosmWasm/cw-nfts/blob/1e992ccf640f07a384d6442625d6780a8e48ef1e/contracts/cw721-base/src/msg.rs#L61)
* The `event_uri` field is used to initialize the `extension` part of the `cw721-base` [MintMsg< T >](https://github.com/CosmWasm/cw-nfts/blob/1e992ccf640f07a384d6442625d6780a8e48ef1e/contracts/cw721-base/src/msg.rs#L61) and contains a valid `IPFS` URI corresponding to where a `.json` file with the following event's metadata is stored:

```json
{
  "name": "My awesome event",
  "description": "Brief description of the event",
  "city": "city where the event will be taking place",
  "country": "country where the event will be taking place",
  "start_date": "Start date for your event",
  "end_date": "End date for your event, no attendees will be able to mint POAPs from your event",
  "year": <number>,
  "event_url": "Event URL",
  "virtual_event": true or false,
}
```

#### Execute
```rust
pub enum ExecuteMsg {
  EnableMint{},
  DisableMint{},
  Mint{},
  MintTo{recipient: String},
  UpdateEventInfo {
    start_time: Timestamp,
    end_time: Timestamp,
  },
  UpdateAdmin{new_admin: String},
  UpdateMinter{new_minter: String}
}
```

#### EnableMint
With the `EnableMint{}` message the admin can enable the `Mint{}` message for everyone.

#### DisableMint
With the `DisableMint{}` message the admin can disable the `Mint{}` message for everyone.

#### Mint
If enabled, the `Mint{}` message allows users to mint their own POAP.  It's disabled after the event's end.

#### MintTo
With the `MintTo{recipient}` message the contract's admin or the minter can mint the POAP for a specific recipient.
It's disabled after the event's end.

#### UpdateEventInfo
With the `UpdateEventInfo{start_time, end_time}` message the event's creator can change the time frame of the event if it's not already started or finished.

#### UpdateAdmin
With the `UpdateAdmin{new_admin}` message the contract's admin can transfer the admin rights to another user.

#### UpdateMinter
With the `UpdateMinter{new_minter}` message the contract's admin can choose another minter to which give the minting
rights.

### Query
All the queries below, except for the `EventInfo`, `Admin` and `MintStatus` are inherited from [cw721-base queries](https://github.com/CosmWasm/cw-nfts/blob/1e992ccf640f07a384d6442625d6780a8e48ef1e/contracts/cw721-base/src/msg.rs#L76).
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return the event info as a  QueryEventInfoResponse
    EventInfo {},

    /// Return the configuration info as a QueryConfigResponse
    Config{}
}
```

#### EventInfo
This query returns all the useful information of the event associated to the POAPs.

```rust
pub struct QueryEventInfoResponse {
  pub creator: Addr,
  pub start_time: Timestamp,
  pub end_time: Timestamp,
  pub event_uri: String,
}
```

#### Config
This query returns the contract's configuration
```rust
pub struct QueryConfigResponse {
    pub admin: Addr,
    pub minter: Addr,
    pub mint_enabled: bool,
    pub per_address_limit: u32,
    pub cw721_contract_code: Uint64,
    pub cw721_contract: Addr,
}
```

## References

- https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md
- https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base
- https://github.com/public-awesome/launchpad/tree/main/contracts/minter
