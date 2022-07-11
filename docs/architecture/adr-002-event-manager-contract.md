# ADR 002: POAP Manager Contract

## Changelog

- July 11, 2022: Initial draft;

## Status
DRAFTED

## Abstract
This ADR defines the architecture of the POAP Manager contract. This contract give a user the possibility to manage
the minting process of a POAP by setting a range of conditions that users needs to fulfill in order to be able to get
the POAP.

## Context
Inside Desmos based social networks, users have a profile with which they interact with the different features that the
applications provides. From the `v4.1.0` release, users can also create contents on-chain through the post module. Looking
forward to the next Cosmoverse 2022, we would like to be able to check the eligibility to mint a POAP. Attendees that wants
to receive the badge, will be required to fulfill three simple tasks:
1. Create a post
2. Comment a post
3. React to a post

Once done, the contract will trigger the mint of the POAP automatically. The mint will take place separately inside the
`POAP-Contract` and no one else except the `POAP-Manager` will be able to perform the operation.

## Decision
The idea is to write a simple contract that make usage of the recently tagged `v1.0.0` of the Desmos-Bindings package
to check on the 3 mint conditions and then call the `POAP-contract` to mint the POAP.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
  pub admin: Addr,
  pub poap_contract: Addr,
  pub subspace_id: u64,
}
```

* The `admin` identifies the user that controls the contract;
* The `poap_contract` that the event manager control;
* The `subspace_id` identifies the dApp where the contract lives;

#### Execute
```rust
pub enum ExecuteMsg{
  TryMint{},
  MintTo{
    user: Addr,
  },
  EditAdmin{
    new_admin: Addr,
  }
}
```

### Query

## References
