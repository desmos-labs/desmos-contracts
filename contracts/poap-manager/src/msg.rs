use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use poap::msg::InstantiateMsg as POAPInstantiateMsg;
use crate::error::ContractError;
use crate::state::Config;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub poap_code_id: u64,
    pub poap_instantiate_msg: POAPInstantiateMsg,
    pub subspace_id: u64,
    pub event_post_id: u64,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.admin.trim().is_empty() {
            return Err(ContractError::invalid_message(
                "admin can not be empty or blank",
            ));
        }
        if self.poap_code_id == 0 {
            return Err(ContractError::invalid_message("code id can not be zero"));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Claim { post_id: u64 },
    MintTo { recipient: String },
    UpdateAdmin { new_admin: String },
}

impl ExecuteMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::Claim { .. } => Ok(()),
            ExecuteMsg::MintTo { recipient } => {
                if recipient.trim().is_empty() {
                    return Err(ContractError::invalid_message(
                        "recipient can not be empty or blank",
                    ));
                }
                Ok(())
            }
            ExecuteMsg::UpdateAdmin { new_admin } => {
                if new_admin.trim().is_empty() {
                    return Err(ContractError::invalid_message(
                        "new admin can not be empty or blank",
                    ));
                }
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    /// Return a ConfigResponse containing the configuration info of the Manager contract
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {
   pub config: Config,
}
