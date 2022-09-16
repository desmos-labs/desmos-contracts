# ADR 003: Remarkables Contract

## Changelog

- Aug 5, 2022: Initial draft;
- Aug 9, 2022: First review
- Aug 12, 2022: Accepted

## Status
ACCEPTED (Not implemented)

## Abstract
This ADR defines the architecture of the Remarkables contract. This contract manage the creation of _Remarkables_.
_Remarkables_ are Desmos Posts that has reached particular goals in terms of engagement (reactions).
For example, a _remarkable_ post can be one that reached the goal of 100 reactions. The higher is the engagement
goal reached, the rarer the _remarkable_ will be.

## Context
Applications like the new DFP (_Desmos Flagship Product_) are going to be a new definition of social apps merging classic social networks features to
crypto ones. Lately, we saw a daily increase of interest towards NFTs from the major players in the social network market
such as Instagram, Facebook, Twitter. This because the growing interest of the masses towards them. NFTs can, and are the bridges
with which layman users started moving their first steps in the crypto world. Even tho they can interact with them and buy/sell/send them,
there's no concept of giving them the ability to mint them for something they did.

## Decision
The idea here is to write a contract called `Remarkables`, that handles the minting of these NFTs created from the applications
contents. The _Remarkables_ have a `Rarity` level based on the engagement (reactions) threshold they've reached.
To spin up a good tokenomics around these, we need to make sure that minting them requires a fee in DSM proportional to
their rarity but still not exaggerated.
The contract will take care of:
* pairing the rarity levels to fees
* checking that `mint` conditions are filled;
* forwarding the mint message to the underlying `cw-721` contract
* update the configuration of the contract (such as admin and other config options)

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
  pub admin: String,
  pub cw721_code_id: u64,
  pub cw721_instantiate_msg: Cw721InstantiateMsg,
  pub subspace_id: u64,
  pub rarities: Vec<Rarity>,
}
```

* The `admin` identifies the user that controls the contract;
* The `cw721_code_id` refers to a previously uploaded `CW721-base` contract on the chain;
* The `cw721_instantiate_msg` contains the info to instantiate the `CW721-base`;
* The `subspace_id` identifies the application which is deploying the contract;
* The `rarities` identifies the list of rarities related info, which are level, mint fees and engagement.

***NOTE***
The `Config` of the contract should also contain the `minter` address associated to the `CW721-base` contract

### Rarity
The `Rarity` shows the required mint fees and engagement threshold to mint the remarkables.
```rust
pub struct Rarity {
    pub level: u32,
    pub engagement_threshold: u32,
    pub mint_fees: Vec<Coin>,
}
```
* The `level` identifies the rarity level of the Remarkable
* The `engagement_threshold` field identifies the amounts of reactions received
* The `mint_fees` identifies the amount of tokens needed to mint the remarkable

During the instantiation, we should:
* Check that the `subspace_id` identifies an existent subspace
* Check that the `admin` is the subspace admin

#### Metadata
The `Metadata` is used to store the extended information inside CW721-base.

```rust
pub struct Metadata {
  pub rarity_level: u32
  pub subspace_id: u64,
  pub post_id: u64
}
```
* The `rarity_level` field identifies the rarity level of the token.
* The `subspace_id` field identifies the subspace which the token belongs to.
* The `post_id` field identifies the post which the token is associated to.

#### Execute
```rust
pub enum ExecuteMsg{
  Mint{post_id: u64, remarkable_uri: String, rarity_level: u64},
  UpdateRarityMintFee{rarity_level: u64, new_fee: Vec<Coin>},
  UpdateAdmin{new_admin: String}
}
```

##### Mint
With the `Mint{post_id}` message the user call the contract to try minting the Remarkable. The contract will perform some checks before
calling the `CW721-base` to proceed with the mint:
* Checks if the `post_id` exists inside the subspace;
* Checks the validity of the `remarkable_uri` (as IPFS uri);
* Checks that the `sender` is the posts author;
* Check if the `rarity_level` reached match the `engagement_threshold` of the post (sum the amount of post's reactions);
* Check that the rarity level exists and the fees are covered (we can pass the fees with the `MessageInfo` `funds` field).

The `Mint<T>` message fields of the `CW721-base` should be filled as follows:
* The `token_id` should be equal to the `post_id`. This will grant the uniqueness of the NFT;
* The `owner` is the contract `sender` (the post `author`);
* The `token_uri` should be equal to the `remarkable_uri`;

The usage of the extention field here is not needed.

##### UpdateRarityMintFee
With the `UpdateRarityMintFee{rarity_level, new_fee}` message the `admin` of the contract can update the fees associated with
a given rarity level. Here we need to check that:
* The contract `sender` is the admin;
* The `rarity_level` exists;
* The `new_fee` is not equal to the existent one.

##### UpdateAdmin
With the `UpdateAdmin{new_admin}` message, the current admin can choose another admin to which give the control of the contract.
Here we need to check that:
* The contract `sender` is the admin;
* The `new_admin` is the new admin of the subspace also.

### Query
```rust
pub enum QueryMsg {
  /// Return a ConfigResponse containing the configuration info of the contract
  Config{},
  // Returns all the rarities info as a QueryRaritiesResponse.
  Rarities{},
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

#### Config
The `Config{}` query returns the contract's configuration inside a `QueryConfigResponse`.
```rust
pub struct QueryConfigResponse {
  pub admin: Addr,
  pub cw721_minter: Addr,
  pub cw721_code_id: u64,
  pub subspace_id: u64,
}
```

#### Rarities
The `Rarities{}` query return all the info rarities required to mint a specific Remarkable inside a `QueryRaritiesResponse`.
```rust
pub struct QueryRaritiesResponse {
  pub rarities: Vec<Rarity>,
}
```

### AllNftInfo
This query returns all the nft info of the token from cw721 contract.
```rust
pub struct AllNftInfoResponse<Metadata> {
    pub access: OwnerOfResponse,
    pub info: NftInfoResponse<Metadata>,
}
```

### Tokens
This query returns all the ids of tokens owned by the given owner from cw721 contract.
```rust
pub struct TokensResponse {
    pub tokens: Vec<String>,
}
```
