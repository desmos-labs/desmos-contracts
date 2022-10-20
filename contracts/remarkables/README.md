# Remarkables contract

Contract that allows content creators to mint a "Remakable" (NFT) of their post that reaches the required rarity level.
To easily interact with the contract you can use the `remarkables` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils), 
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message
Allows to initialize the contract. This message has the following parameters:
* `admin`: Address of the user that has the rights to administer the contract;
* `cw721_code_id`: Id of the CW721 contract to be initialized together with this contract;
* `cw721_instantiate_msg`: Initialization [message](../cw721-remarkables/README.md#instantiate_message) that will be sent to the CW721 contract;
* `subspace_id`: Id of the target subspace where to launch the contract; 
* `rarities`: List of the possible Remarkables rarities and their details [here](#Rarity).

Here an example message to instantiate the contract:
```json
{
    "admin": "desmos1......",
    "minter": "desmos1......",
    "cw721_code_id": "1",
    "cw721_instantiate_msg": {
        "name": "remarkables_nft",
        "symbol": "remarkables",
        "minter": "contract_address"
    },
    "subspace_id": "1",
    "rarities": [
        {
            "engagement_threshold": 10,
            "mint_fees": [
                {
                    "amount": "100",
                    "denom": "udsm",
                }
            ]
        },
        {
            "engagement_threshold": 100,
            "mint_fees": [
                {
                    "amount": "1000",
                    "denom": "udsm",
                }
            ]
        }
    ]
}
```

### Rarity
Represents the requirement to mint a Remarkable (NFT) for a post. This structure has the folloing parameters:
* `engagement_threshold`: Threshold of the needed reactions amount to the post;
* `mint_fees`: Fees to mint a remarkables NFT for a post.

## Execute Messages

### Mint
Allows the post author to mint a Remarkable NFT for their post that satisfies the engagement threshold requirement. This message has the following parameters:
* `post_id`: Id of the target post;
* `remarkables_uri`: IPFS uri where the Remarkable metadata are stored;
* `rarity_level`: Rarity level of the post.

Here an example message to mint a Remarkable NFT for the post:
```json
{
    "mint": {
        "post_id": "1",
        "remarkables_uri": "ipfs://remarkables.info",
        "rarity_level": 1
    },
    "funds": [
        {
            "amount": "100",
            "denom": "udsm",
        }
    ]
}
```

### UpdateRarityMintFees
Allows the admin to change the mint fees of a target rarity level.
* `rarity_level`: Level at which mint fees will be changed;
* `new_fees`: Fees which replace the old mint fees.

Here an example message to update mint fees of the given rarity level:
```json
{
    "update_rarity_mint_fees": {
        "rarity_level": 1,
        "new_fees": [
            {
                "amount": "10", 
                "denom": "uatom"
            },
            {
                "amount": "100",
                "denom": "udsm"
            }
        ]
    }
}
```

### UpdateAdmin
Allows the contract's admin to transfer the admin rights to another user. This message has the following parameter:
* `new_admin`: Address to be the new admin that controls this contract.

Here an example message to update the contract admin:
```json
{
    "update_admin": {
        "new_admin": "desmos1......"
    }
}
```

### ClaimFees
Allows the contract admin to claim the fees paid from the users to execute the contract. 
This message has the following parameter:
* `receiver`: Address to which the fees will be sent.

Here an example message to claim the fees:
```json
{
  "claim_fees": {
    "receiver": "desmos1..."
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
    "cw721_code_id": "1",
    "cw721_address": "desmos1......",
    "subspace_id": "1"
}
```

### Rarities
Allows to query the rarities that could be minted in the contract.

Here an example message to query the rarities:
```json
{
    "rarities": {}
}
```

Response:
```json
{
    "rarities": [
        {
            "engagement_threshold": 10,
            "mint_fees": [
                {
                    "amount": "100",
                    "denom": "udsm",
                }
            ]
        },
        {
            "engagement_threshold": 100,
            "mint_fees": [
                {
                    "amount": "1000",
                    "denom": "udsm",
                }
            ]
        }
    ]
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
        "token_id": "1-1",
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
        "start_after": "1-1",
        "limit": 3
    }
}
```

Response:
```json
{
    "tokens": ["1-2", "1-3", "1-4"]
}
```