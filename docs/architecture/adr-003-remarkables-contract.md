# ADR 002: Remarkables Contract

## Changelog

- Aug 5, 2022: Initial draft;

## Status
DRAFTED

## Abstract
This ADR defines the architecture of the Remarkables contract. This contract manage the creation of _Remarkables_.
_Remarkables_ are Desmos Posts that has reached particular goals in terms of engagement (combination of reactions + comments).
For example, a _remarkable_ post can be one that reached the goal of 50 comments and 100 reactions. The higher is the engagement
goal reached, the rarer the _remarkable_ will be.

## Context
Applications like the new DFP are going to be a new definition of social apps merging classic social networks features to
crypto ones. Lately, we saw a daily increase of interest towards NFTs from the major players in the social network market
such as Instagram, Facebook, Twitter. This because the growing interest of the masses towards them. NFTs can, and are the bridges
with which layman users started moving their first steps in the crypto world. Even tho they can interact with them and buy/sell/send them,
there's no concept of giving them the ability to mint them for something they did.

## Decision
The idea here is to write a contract called `Remarkables`, that handles the minting of these NFTs created from the applications
contents. The _Remarkables_ have a `Rarity` level based on the engagement (reactions+comments) threshold they've reached.
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
  pub rarity_fees: (u64, )
}
```

* The `admin` identifies the user that controls the contract;
* The `poap_contract_code_id` identifies the code id of the contract necessary for the instantiation;
* The `poap_instantiate_msg` instantiate the poap contract that the manager controls;

#### Execute
```rust
pub enum ExecuteMsg{
  Claim{post_id: u64},
  MintTo{user: String},
  UpdateAdmin{new_admin: String}
}
```

##### Claim
With the `Claim{}` message the user call the contract to try claiming the POAP. The claim will be successful only if the user has created a profile before.

##### MintTo
With the `MintTo{user}` message the admin of the contract can bypass the claim procedure and mint the POAP to a user (that still need to have a profile).

##### UpdateAdmin
With the `UpdateAdmin{new_admin}` message, the current admin can choose another admin to which give the control of the contract.

### Query
```rust
pub enum QueryMsg {
  /// Return a ConfigResponse containing the configuration info of the contract
  Config{},
}
```

#### Config
The `Config{}` query returns the contract's information inside a `ConfigResponse`.
```rust
pub struct ConfigResponse {
  pub admin: Addr,
  pub poap_contract_code_id: u64,
  pub poap_contract_address: Addr,
}
```

## References
- [POAP-Contract](https://github.com/desmos-labs/desmos-contracts/blob/leonardo/adr-001/docs/architecture/adr-001-poap-contract.md)
