# CW721 POAP contract

Contract that defines the cw721-base contract having custom POAP related `Metadata`, which is used with POAP contract.
To easily interact with the contract you can use the `cw721-poap` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils), 
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message
Allows to initialize the contract. This message has the following parameters:
* `name`: Name of the NFT contract;
* `symbol`: Symbol of the NFT contract;
* `minter`: Address who is the only one to be able to create new NFTs.

An example of the instantiate message:
```json
{
    "name": "test_name",
    "symbol": "test",
    "minter": "desmos1......"
}
```

## Execute Messages

### TransferNft
Allows to move a token to another account without triggering actions. This message has the following parameters:
* `recipient`: Address where the token transfer to;
* `token_id`: Id of the token which would be transferred.

An example of the message to transfer nft:
```json
{
    "recipient": "desmos1......",
    "token_id": "1"
}
```

### SendNft
Allows to move a token to another contract then trigger an action. This message has the following parameters:
* `contract`: Contract address where the token transfer to;
* `token_id`: Id of the token which would be transferred;
* `msg`: Base64 encoded message to trigger on the receiver contract.

An example of the message to send nft having a trigger message:
```json
{
    "contract": "desmos1......",
    "token_id": "1",
    "msg": "eyJleGVjdXRlX2V4YW1wbGUiOnt9fQ==" // {"execute_example":{}} 
}
```

### Approve

### Revoke

### ApproveAll

### RevokeAll

### Mint

### Burn

## Query Messages

### OwnerOf

### Approval

### Approvals

### AllOperators

### NumTokens

### ContractInfo

### NftInfo

### AllNftInfo

### Tokens

### AllTokens

### Minter

