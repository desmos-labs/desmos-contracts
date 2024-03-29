# ADR 001: POAP Contract

## Changelog

- June 27, 2022: Initial draft;
- June 28, 2022: Second review;
- June 29, 2022: Third review;
- July 12, 2022: Fourth review.
- July 14, 2022: Fifth review;
- July 15, 2022: Sixth review;
- July 18, 2022: Accepted ADR;
- August 8, 2022: Reviewed ADR.

## Status
ACCEPTED (Implemented)

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
- Associations between POAPs and attendees/claimers;
- Association between POAPs and owners;
- Associations between POAPs and metadata;
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
    pub poap_uri: String,
    pub cw721_code_id: u64,
}
```

* The `creator` field identifies the address of the event's creator;
* The `start_time` identifies the start time of the event;
* The `end_time` identifies the end time of the event;
* The `per_address_limit` identifies the max num of tokens that can be minted by an address;
* The `poap_uri` identifies a valid `IPFS` URI corresponding to where the asset and metadata of the POAP are stored.
  * The `metadata` file is a `.json` that follow the `ERC-721` [metadata standard](https://docs.opensea.io/docs/metadata-standards#metadata-structure)
* The field will be used to initialize the `token_uri` field of the `cw721-base` [`MintMsg<T>`](https://github.com/CosmWasm/cw-nfts/blob/1e992ccf640f07a384d6442625d6780a8e48ef1e/contracts/cw721-base/src/msg.rs#L61)
The metadata should be filled as the following ones:
```json
{
  "name": "POAP Cosmoverse",
  "description": "Cosmoverse POAP badge",
  "image": "ipfs://bafybeidyiza7igkgezh2uzma6dkbq7gmjzqrn6nwwoplm26qtj7i4mlbym/poap_cosmoverse.jpg",
  "external_url": "https://twitter.com/CosmoverseHQ",
  "animation_url": "ipfs://bafybeia5r3hwyou3iggzfvakjkxu2zy5pt3kjil6nyqzvrqwrrtkwe6xrm/images/poap_rotating.m4a"
  "attributes": [
    {
      "display_type": "City",
      "trait_type": "city",
      "value": "Medellìn"
    },
    {
      "display_type": "Country",
      "trait_type": "country",
      "value": "Colombia"
    },
    {
      "display_type": "Coordinates",
      "trait_type": "coordinates",
      "value": "6.2476° N, 75.5658° W"
    },
    {
      "display_type": "Start date",
      "trait_type": "start_date",
      "value": "2022-09-26T09:00:00.000000000Z"
    },
    {
      "display_type": "End date",
      "trait_type": "end_date",
      "value": "2022-09-26T09:00:00.000000000Z"
    },
    {
      "display_type": "Year",
      "trait_type": "year",
      "value": 2022
    },
    {
      "display_type": "Event website",
      "trait_type": "event_website",
      "value": "https://www.cosmoverse.com"
    },
    {
      "display_type": "Virtual Event",
      "trait_type": "virtual_event",
      "value": false
    }
  ]
}
```
* The `cw721_code_id` identifies the code of the `CW721-base` contract initialised by this contract.

#### Metadata
The `Metadata` is used to store the extended information inside CW721-base.

```rust
pub struct Metadata {
  pub claimer: Addr,
}
```
* The `claimer` field identifies the address who mint the POAP token.

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
  UpdateMinter{new_minter: String},
}
```

#### EnableMint
With the `EnableMint{}` message the admin can enable the `Mint{}` message for everyone.

#### DisableMint
With the `DisableMint{}` message the admin can disable the `Mint{}` message for everyone.

#### Mint
If enabled, the `Mint{}` message allows users to mint their own POAP. It also stores the user address inside CW721-base extension field as the claimer by `Metadata`.
It's disabled after the event's end.

#### MintTo
With the `MintTo{recipient}` message the contract's admin or the minter can mint the POAP for a specific recipient. It also stores the user address inside CW721-base extension field as the claimer by `Metadata`.
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
    Config{},

    /// Return the nft info with approvals from cw721 contract as a AllNftInfoResponse
    AllNftInfo{
      token_id:String, 
      include_expired: Option<bool>,
    },

    /// Return all the tokens ids owned by the given owner from cw721 contract as a TokensResponse
    Tokens{
      owner: String,
      start_after: Option<String>,
      limit: Option<u32>,
    }
}
```

#### EventInfo
This query returns all the useful information of the event associated to the POAPs.

```rust
pub struct QueryEventInfoResponse {
  pub creator: Addr,
  pub start_time: Timestamp,
  pub end_time: Timestamp,
  pub poap_uri: String,
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

### AllNftInfo
This query returns all the nft info of the token from cw721 contract
```rust
pub struct AllNftInfoResponse<Metadata> {
    pub access: OwnerOfResponse,
    pub info: NftInfoResponse<Metadata>,
}
```

### Tokens
This query returns all the ids of tokens owned by the given owner from cw721 contract
```rust
pub struct TokensResponse {
    pub tokens: Vec<String>,
}
```

## References

- https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md
- https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base
- https://github.com/public-awesome/launchpad/tree/main/contracts/minter
