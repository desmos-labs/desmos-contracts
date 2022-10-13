# CW721 POAP contract

Contract that defines the cw721-base contract having custom POAP related `Metadata`, which is used by POAP contract.
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
    "transfer_nft": {
        "recipient": "desmos1......",
        "token_id": "1"
    }
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
    "send_nft": {
        "contract": "desmos1......",
        "token_id": "1",
        "msg": "eyJleGVjdXRlX2V4YW1wbGUiOnt9fQ==" // {"execute_example":{}} 
    }
}
```

### Approve
Allows to give an approval to an operator to transfer/send the token from the owner's account. This message has the following parameters:
* `spender`: Address who would be assigned as an operator of the token;
* `token_id`: Id of the target token;
* `expires`: The expiration time/height of this allownce, if it is set as `null` then it has no time/height limit.

An example of the message to approve a token to an operator with a limit height:
```json
{
    "approve": {
        "spender": "desmos1......",
        "token_id": "1",
        "expires": {
            "at_height": "1000"
        }
    }
}
```

An example of the message to approve a token to an operator with a limit time:
```json
{
    "approve": {
        "spender": "desmos1......",
        "token_id": "1",
        "expires": {
            "at_time": "2022-01-01T00:00:00Z"
        }
    }
}
```

An example of the message to approve a token to an operator without any expiration:
```json
{
    "approve": {
        "spender": "desmos1......",
        "token_id": "1",
        "expires": null
    }
}
```

### Revoke
Allows to remove a previously granted approval. This message has the following parameters:
* `spender`: Address who would be revoked operator permission of the given token;
* `token_id`: Id of the target token.

An example of the message to revoke an operator to a token:
```json
{
    "revoke": {
        "spender": "desmos1......",
        "token_id": "1"
    }
}
```

### ApproveAll
Allows to give all the tokens transfering/sendind tokens approval to an operator from the owner's account. This message has the following parameters:
* `operator`: Address who is assigned to have all the tokens approvals in the owner's account;
* `expires`: The expiration time/height of this allownce, if it is set as `null` then it has no time/height limit.

An example of the message to approve all the tokens to an operator with a limit height:
```json
{
    "approve_all": {
        "spender": "desmos1......",
        "expires": {
            "at_height": "1000"
        }
    }
}
```

An example of the message to approve all the tokens to an operator with a limit time:
```json
{
    "approve_all": {
        "spender": "desmos1......",
        "expires": {
            "at_time": "2022-01-01T00:00:00Z"
        }
    }
}
```

An example of the message to approve all the tokens to an operator without any expiration:
```json
{
    "approve_all": {
        "spender": "desmos1......",
        "expires": null
    }
}
```

### RevokeAll
Allows to remove a previously granted approval all permission. This message has the following parameters:
* `operator`: Address who would be revoked operator permissions of all the tokens from the owner's account.

An example of the message to revoke operator permissions to all the tokens from the owner's account:
```json
{
    "revoke": {
        "spender": "desmos1......",
    }
}
```

### Mint
Allows the minter to mint a new NFT to a user. This message has the following parameters:
* `token_id`: unique id of the NFT;
* `owner`: the owner of the newly minted NFT;
* `token_uri`: universal resource identifier for this NFT;
* `extension`: the `POAP metadata` which includes claimer of this NFT.

An example of the message to mint new NFT:
```json
{
    "mint": {
        "token_id": "1",
        "owner": "desmos1......",
        "token_uri": "ipfs://token.erc721.metadata",
        "extension": {
            "claimer": "desmos1......"
        }
    }
}
```

### Burn
Allows to burn an NFT the sender has access to. This message has the following parameters:
* `token_id`: Id of the token that would be burned.

An example of the message to burn an NFT:
```json
{
    "burn": {
        "token_id": "1"
    }
}
```

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

