# ADR 002: POAP Manager Contract

## Changelog

- July 11, 2022: Initial draft started;
- July 12, 2022: Initial draft finished;
- July 14, 2022: First review;
- July 15, 2022: Second review;
- July 25, 2022: Third review;

## Status
DRAFTED

## Abstract
This ADR defines the architecture of the POAP Manager contract. This contract give a user the possibility to manage
the minting process of a POAP by setting a range of conditions that users needs to fulfill in order to be able to claim
the POAP.

## Context
Inside Desmos based social networks, users have a profile with which they interact with the different features that the
applications provides. Looking forward to the next Cosmoverse 2022 and the future conferences,
we would like to have a way to be able to check the eligibility to mint a POAP. Attendees that wants to claim the badge,
will be required to have a Desmos profile. The mint will take place separately inside the `POAP-Contract` and no one
else besides the `POAP-Manager` will be able to perform the operation.

## Decision
The idea is to write a simple contract that make usage of the recently tagged `v1.0.0` of the `Desmos-Bindings` package
to check on the users profiles before allowing the claim.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
  pub admin: String,
  pub poap_contract_code_id: u64,
  pub poap_instantiate_msg: POAPContractInstantiateMsg,
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
With the `Claim{post_id}` message the user call the contract to try claiming the POAP. The claim will be successful only if the user has created a profile before.

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
