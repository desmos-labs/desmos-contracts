use crate::{
    profiles::{
        models_app_links::ApplicationLink, models_blocks::UserBlock, models_chain_links::ChainLink,
        models_dtag_requests::DtagTransferRequest, models_profile::Profile,
        models_relationships::Relationship,
    },
    types::PageResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/** Profile query models **/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryProfileResponse {
    pub profile: Profile,
}

/** DtagTransferRequest query models **/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryIncomingDtagTransferRequestResponse {
    pub requests: Vec<DtagTransferRequest>,
    pub pagination: PageResponse,
}

/** AppLinks query models **/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinksResponse {
    pub links: Vec<ApplicationLink>,
    pub pagination: PageResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserApplicationLinkResponse {
    pub link: ApplicationLink,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinkByClientIDResponse {
    pub link: ApplicationLink,
}

/** ChainLinks query models **/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryChainLinksResponse {
    pub links: Vec<ChainLink>,
    pub pagination: PageResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserChainLinkResponse {
    pub link: ChainLink,
}

/** UserBlocks query models **/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryBlocksResponse {
    pub blocks: Vec<UserBlock>,
    pub pagination: PageResponse,
}

/** Relationships query models **/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRelationshipsResponse {
    pub relationships: Vec<Relationship>,
    pub pagination: PageResponse,
}
