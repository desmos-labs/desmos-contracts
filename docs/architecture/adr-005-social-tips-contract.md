# ADR 005: Social tips contract

## Changelog

- Oct 10, 2022: Initial draft;
- Oct 12, 2022: Second review;

## Status
DRAFT

## Abstract
This ADR defines the architecture of the Social Tips contract. This contract allows sending tips to a user
using their know **handle** of a centralized application.  
The handle can be the @handle of Twitter, the user's nickname on Discord or even an e-mail address.  
Additionally, users may want to send tips to other users that don't have a Desmos profile this contract
can collect those tips and allow the users to later claim it.

# Context
Desmos based applications may want to leverage on the web3 features and one of the most popular one is for sure
the possibility to tip users. Currently, it's very easy to send tokens to a user knowing is address,
but it's more difficult to send tokens to a user knowing only his identity on a centralized application.

## Decision
The idea here is to write a contract called `Social Tips`, that allows to send tokens to another user 
through their centralized application handle. Also, if the receiver haven't linked yet his 
centralized application handle to their Desmos profile, this contract allows them to claim their tips later after having
proved that they own such identity on the centralized application.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Execute
```rust
enum ExecuteMsg {
  SendTip { 
      amount: Vec<Coin>, 
      application: String,
      handle: String,
  },
  ClaimTips {},
}
```

##### SendTip
With the `SentTip` message the user can send a tip to another user though the handle of a linked centralized application.
The `MessageInfo` fields contains:
* the funds necessary to cover the fees plus the tip amount specified inside the `funds` field
* the sender identified by the `sender` field

#### ClaimTips
With the `ClaimTips` message the user can claim their pending tips that has been sent to him before proving that
owns the identities to which the tips refer.

### Query
```rust
enum QueryMsg {
  UserPendingTips { user: String }
}
```

##### UserPendingTips
With the `UserPendingTips` message the user can query the pending tips of the given `user`.
The returned tips are provided as follows:
```rust
struct QueryPendingTipsResponse {
    pub tips: Vec<Tip>,
}

struct Tip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
    pub block_height: u64,
}
```
