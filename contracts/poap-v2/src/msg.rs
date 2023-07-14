use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Timestamp};
use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, QueryMsg as Cw721BaseQueryMsg};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use cw_utils::Expiration;
use schemars::JsonSchema;
use std::fmt::Debug;

#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the POAP contract.
    pub name: String,
    /// Symbol of the POAP contract
    pub symbol: String,
    /// Who controls the contract.
    /// If None will be used the address of who is instantiating the contract.
    pub admin: Option<String>,
    /// The URI where users can view the associated metadata for the POAPs,
    /// ideally following the ERC-721 metadata scheme in a JSON file.
    pub metadata_uri: String,
    /// Additional address that is allowed to mint tokens on behalf of other users.
    pub minter: Option<String>,
    /// Specifies whether each POAP can be transferred from one user to another.
    pub is_transferable: bool,
    /// Indicates whether users can mint the POAPs.
    pub is_mintable: bool,
    /// Identifies the timestamp at which the minting of the POAP will be enabled.
    /// If not set, the minting is always enabled.
    pub mint_start_time: Option<Timestamp>,
    /// Identifies the timestamp at which the minting of the POAP will be disabled.
    /// If not set, the minting will never end.
    pub mint_end_time: Option<Timestamp>,
}

/// This is like Cw721ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg<T, E> {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft { recipient: String, token_id: String },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke { spender: String, token_id: String },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },
    /// Mint a new POAP for the caller
    Mint { extension: T },
    /// Mint a new POAP for the provided users, can
    /// only be called from the contract minter.
    MintTo { users: Vec<String>, extension: T },
    /// Burn an NFT the sender has access to.
    Burn { token_id: String },
    /// Allow to update the user with the mint permissions,
    /// can only be called from the contract admin.
    UpdateMinter { minter: String },
    /// Sets if the users can mint their POAP,
    /// can only be called from the contract admin.
    SetMintable { mintable: bool },
    /// Sets if the users can transfer their POAP,
    /// can only be called from the contract admin.
    SetTransferable { transferable: bool },
    /// Sets the time period of when the POAP can be minted from
    /// the users, can only be called from the contract admin.
    SetMintStartEndTime {
        /// Identifies the timestamp at which the minting of the POAP will be enabled.
        /// If None, the minting is always enabled.
        start_time: Option<Timestamp>,
        /// Identifies the timestamp at which the minting of the POAP will be disabled.
        /// If None, the minting will never end.
        end_time: Option<Timestamp>,
    },
    /// Extension msg
    Extension { msg: E },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<Q: JsonSchema> {
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse<Q>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse<Q>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Return the minter
    #[returns(cw721_base::MinterResponse)]
    Minter {},
    /// Extension query
    #[returns(())]
    Extension { msg: Q },
}

impl<T, E> From<ExecuteMsg<T, E>> for Cw721BaseExecuteMsg<T, E>
where
    T: Debug,
    E: Debug,
{
    fn from(execute_msg: ExecuteMsg<T, E>) -> Self {
        match execute_msg {
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Cw721BaseExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                msg,
                token_id,
                contract,
            } => Cw721BaseExecuteMsg::SendNft {
                msg,
                token_id,
                contract,
            },
            ExecuteMsg::Approve {
                token_id,
                spender,
                expires,
            } => Cw721BaseExecuteMsg::Approve {
                token_id,
                spender,
                expires,
            },
            ExecuteMsg::Revoke { spender, token_id } => {
                Cw721BaseExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::ApproveAll { expires, operator } => {
                Cw721BaseExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::RevokeAll { operator } => Cw721BaseExecuteMsg::RevokeAll { operator },
            ExecuteMsg::Burn { token_id } => Cw721BaseExecuteMsg::Burn { token_id },
            ExecuteMsg::Extension { msg } => Cw721BaseExecuteMsg::Extension { msg },
            _ => unreachable!("cannot convert {:?} to Cw721BaseExecuteMsg", execute_msg),
        }
    }
}

impl<Q> From<QueryMsg<Q>> for Cw721BaseQueryMsg<Q>
where
    Q: JsonSchema + Debug,
{
    fn from(query_msg: QueryMsg<Q>) -> Self {
        match query_msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721BaseQueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::Approval {
                include_expired,
                token_id,
                spender,
            } => Cw721BaseQueryMsg::Approval {
                include_expired,
                token_id,
                spender,
            },
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Cw721BaseQueryMsg::Approvals {
                token_id,
                include_expired,
            },
            QueryMsg::Operator {
                operator,
                owner,
                include_expired,
            } => Cw721BaseQueryMsg::Operator {
                operator,
                owner,
                include_expired,
            },
            QueryMsg::AllOperators {
                owner,
                limit,
                include_expired,
                start_after,
            } => Cw721BaseQueryMsg::AllOperators {
                owner,
                limit,
                include_expired,
                start_after,
            },
            QueryMsg::NumTokens {} => Cw721BaseQueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721BaseQueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721BaseQueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721BaseQueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721BaseQueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721BaseQueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Extension { msg } => Cw721BaseQueryMsg::Extension { msg },
            _ => unreachable!("cannot convert {:?} to Cw721BaseQueryMsg", query_msg),
        }
    }
}
