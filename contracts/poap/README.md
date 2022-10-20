# POAP contract

Contract that allows users who has a Desmos profile to mint POAP nft via cw721-poap contract.
To easily interact with the contract you can use the `poap` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils), 
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message
Allows to initialize the contract. This message has the following parameters:
* `admin`: Address of who will have the right to administer the contract;
* `minter`: Address of who can mint tokens to other users;
* `cw721_code_id`: Id of the CW721 contract to initialize together with this contract;
* `cw721_instantiate_msg`: Initialization [message](../cw721-poap/README.md#instantiate_message) that will be sent to the CW721 contract;
* `event_info`: Information about the event which is defined [here](#EventInfo).

Here an example message to instantiate the contract:
```json
{
    "admin": "desmos1......",
    "minter": "desmos1......",
    "cw721_code_id": "1",
    "cw721_instantiate_msg": {
        "name": "poap_nft",
        "symbol": "poap",
        "minter": "contract_address"
    },
    "event_info": {
        "creator": "desmos1......",
        "start_time": "2022-12-31T10:00:00Z",
        "end_time": "2022-12-31T19:00:00Z",
        "per_address_limit": 1,
        "poap_uri": "ipfs://poap.info"
    }
}
```

### EventInfo
Represents the information of the event. This structure has the following parameters:
* `creator`: User that created the event;
* `start_time`: Time at which the event begins in RFC3339 format (2022-12-31T10:00:00Z);
* `end_time`: Time at which the event ends in RFC3339 format (2022-12-31T10:00:00Z);
* `per_address_limit`: Max amount of poap that a single user can mint;
* `poap_uri`: Identifies a valid IPFS URI corresponding to where the assets and metadata of the POAPs are stored.

## Execute Messages

### EnableMint
Allows the contract's admin to enable the [Mint](#Mint).

Here an example message to enable mint:
```json
{
    "enable_mint": {}
}
```

### DisableMint
Allows the contract's admin to disable the [Mint](#Mint).

Here an example message to disable mint:
```json
{
    "disable_mint": {}
}
```

### Mint
Allows users to mint a POAP token in the event period if the contract enables mint.

Here an example message to mint a POAP:
```json
{
    "mint": {}
}
```

### MintTo
Allows the minter to mint a POAP token to a recipient in the event period if the contract enables mint. This message has the following parameter:
* `recipient`: Address who will receive the minted token.

Here an example message to mint a POAP to a user:
```json
{
    "mint_to": {
        "recipient": "desmos1......"
    }
}
```

### UpdateEventInfo
Allows the contract admin to update the event info. This message has the following parameters:
* `start_time`: New start time which will be updated;
* `end_time`: New end time which will be updated.

Here an example message to update the event information:
```json
{
    "update_event_info": {
        "start_time": "2022-12-31T10:00:00Z",
        "end_time": "2022-12-31T19:00:00Z"
    }
}
```

### UpdateAdmin
Allows the contract admin to update the contract admin. This message has the following parameter:
* `new_admin`: Address to be the new admin that controls this contract.

Here an example message to update the contract admin:
```json
{
    "update_admin": {
        "new_admin": "desmos1......"
    }
}
```

### UpdateMinter
Allows the contract admin to update the contract minter. This message has the following parameter:
* `new_minter`: Address to be the new minter that has permission to mint tokens to other users.

Here an example message to update the contract minter:
```json
{
    "update_minter": {
        "new_minter": "desmos1......"
    }
}
```

## Query Messages

### Config
Allows to query the config of the contract.

Here an example message to query the config:
```json
{
    "config": {}
}
```

Response:
```json
{
    "admin": "desmos1......",
    "minter": "desmos1......",
    "mint_enabled": true,
    "per_address_limit": 1,
    "cw721_code_id": "1",
    "cw721_address": "desmos1......"
}
```

### EventInfo
Allows to query the information of the event.

Here an example message to query the event info:
```json
{
    "event_info": {}
}
```

Response:
```json
{
    "creator": "desmos1......",
    "start_time": "2022-12-31T10:00:00Z",
    "end_time": "2022-12-31T19:00:00Z",
    "poap_uri": "ipfs://poap.info"
}
```

### MintedAmount
Allows to query the POAP minted amount from a user. This message has the following parameter:
* `user`: Address of the target user.

Here an example message to query the event info:
```json
{
    "minted_amount": {
        "user": "desmos1......"
    }
}
```

Response:
```json
{
    "user": "desmos1......",
    "amount": 1,
}
```

### AllNftInfo
Returns the all the information of the token. This message has the following parameters:
* `token_id`: Id of the target token;
* `include_expired`: Trigger to filter out expired approvals, unset or false will exclude expired approvals.

Here an example meesage to query all the info of the given token:
```json
{
    "all_nft_info": {
        "token_id": "1",
        "include_expired": true
    }
}
```

Response:
```json
{
    "access": {
        "owner": "desmos1......",
        "approvals": [
            {
                "spender": "desmos1......",
                "expiration": {
                    "at_height": 1000
                }
            }, 
            {
                "spender": "desmos1......",
                "expiration": {
                    "at_time": "2022-01-01T00:00:00Z"
                }
            },
            {
                "spender": "desmos1......",
                "expiration": {
                    "never": {}
                }
            },
        ],
    },
    "info": {
        "token_uri": "ipfs://token.erc721.metadata",
        "extension": {
            "claimer": "desmos1......"
        }
    }
}
```

### Tokens
Returns all tokens owned by the given address. This message has the following parameters:
* `owner`: Target address owned tokens to be queried;
* `start_after`: Position in token id where tokens start after;
* `limit`: Limitation to list the number of tokens, if unset would be 10 and the maximum is 100.

Here an example meesage to query all the tokens owned by the given address:
```json
{
    "tokens": {
        "owner": "desmos1......",
        "start_after": "1",
        "limit": 3
    }
}
```

Response:
```json
{
    "tokens": ["2", "3", "4"]
}
```