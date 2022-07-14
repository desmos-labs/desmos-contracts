# ADR 002: POAP Manager Contract

## Changelog

- July 11, 2022: Initial draft started;
- July 12, 2022: Initial draft finished;
- July 14, 2022: First review.

## Status
DRAFTED

## Abstract
This ADR defines the architecture of the POAP Manager contract. This contract give a user the possibility to manage
the minting process of a POAP by setting a range of conditions that users needs to fulfill in order to be able to claim
the POAP.

## Context
Inside Desmos based social networks, users have a profile with which they interact with the different features that the
applications provides. From the `v4.1.0` release, users can also create contents on-chain through the post module.
Looking forward to the next Cosmoverse 2022, we would like to have a way to be able to check the eligibility to mint a POAP. Attendees that wants to receive the badge, will be required to fulfill two simple tasks:
1. Comment the Cosmoverse post we previously created
2. React to the Cosmoverse post

Once done, the contract will trigger the mint of the POAP automatically. The mint will take place separately inside the
`POAP-Contract` and no one else besides the `POAP-Manager` will be able to perform the operation.

## Decision
The idea is to write a simple contract that make usage of the recently tagged `v1.0.0` of the `Desmos-Bindings` package
to check on the 2 mint conditions and then call the `POAP-contract` to mint the POAP.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
  pub admin: Addr,
  pub poap_instantiate_msg: POAPContractInstantiateMsg,
  pub subspace_id: u64,
  pub cosmoverse_post_id: u64
}
```

* The `admin` identifies the user that controls the contract;
* The `poap_instantiate_msg` instantiate the poap contract that the manager controls;
* The `subspace_id` identifies the dApp where the contract lives;
* THe `cosmoverse_post_id` identifies the cosmoverse post to wich the user will react.

#### Execute
```rust
pub enum ExecuteMsg{
  Claim{post_id: u64},
  MintTo{user: String},
  UpdateAdmin{new_admin: String}
}
```

##### Claim
With the `Claim{post_id}` message the user call the contract to try claiming the POAP. The claim will be successful only if the user match the minting requirements explained above. The `post_id` passed with the message is the ID of the post comment made by the user. The requirements can be checked using the `Posts` bindings queries to directly interact with the Desmos chain
and retrieve the information needed.

##### MintTo
With the `MintTo{user}` message the admin of the contract can bypass the claim procedure and mint the POAP to a user (that still need to have a profile).

##### UpdateAdmin
With the `UpdateAdmin{new_admin}` message, the current admin can choose another admin to which give the control
of the contract.

### Query
```rust
pub enum QueryMsg {
  /// Return a ManagerInfoResponse containing the useful information of the Manager contract
  ManagerInfo{},

  /// Inherited from the POAP Contract
  EventInfo{}
}
```

#### ManagerInfo
The `ManagerInfo{}` query returns the contract's information inside a `ManagerInfoResponse`.
```rust
pub struct ManagerInfoResponse {
  pub admin: Addr,
  pub subspace_id: u64,
  pub cosmoverse_post_id: u64
}
```

#### EventInfo
The `EventInfo{}` query is inherited from the `POAP-Contract` and returns all the information about the Event.

## References
- [POAP-Contract](https://github.com/desmos-labs/desmos-contracts/blob/leonardo/adr-001/docs/architecture/adr-001-poap-contract.md)
