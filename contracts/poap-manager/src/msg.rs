use cosmwasm_std::Uint64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::state::Config;
use poap::msg::InstantiateMsg as POAPInstantiateMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub poap_code_id: Uint64,
    pub poap_instantiate_msg: POAPInstantiateMsg,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.admin.trim().is_empty() {
            return Err(ContractError::invalid_message(
                "admin can not be empty or blank",
            ));
        }
        if self.poap_code_id == Uint64::zero() {
            return Err(ContractError::invalid_message("code id can not be zero"));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Claim {},
    MintTo { recipient: String },
    UpdateAdmin { new_admin: String },
}

impl ExecuteMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::Claim {} => Ok(()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Timestamp;
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use poap::msg::EventInfo;
    #[test]
    fn instantiate_msg_with_empty_admin_error() {
        let msg = InstantiateMsg {
            admin: "".into(),
            poap_code_id: 1u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: "test".into(),
                minter: "test".into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        };
        let result = msg.validate();
        assert!(result.is_err());
    }

    #[test]
    fn instantiate_msg_with_blank_admin_error() {
        let msg = InstantiateMsg {
            admin: "   ".into(),
            poap_code_id: 1u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: "test".into(),
                minter: "test".into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        };
        let result = msg.validate();
        assert!(result.is_err());
    }

    #[test]
    fn instantiate_msg_with_invalid_poap_id_error() {
        let msg = InstantiateMsg {
            admin: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
            poap_code_id: 0u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: "test".into(),
                minter: "test".into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        };
        let result = msg.validate();
        assert!(result.is_err());
    }

    #[test]
    fn proper_instantiate_msg_no_error() {
        let msg = InstantiateMsg {
            admin: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
            poap_code_id: 1u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: "test".into(),
                minter: "test".into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        };
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn proper_claim_msg_no_error() {
        let msg = ExecuteMsg::Claim {};
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn mint_into_msg_with_empty_recipient_error() {
        let msg = ExecuteMsg::MintTo {
            recipient: "".into(),
        };
        assert!(msg.validate().is_err());
    }

    #[test]
    fn mint_into_msg_with_blank_recipient_error() {
        let msg = ExecuteMsg::MintTo {
            recipient: "  ".into(),
        };
        assert!(msg.validate().is_err());
    }

    #[test]
    fn proper_mint_into_msg_no_error() {
        let msg = ExecuteMsg::MintTo {
            recipient: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
        };
        assert!(msg.validate().is_ok());
    }
    #[test]
    fn update_admin_msg_with_empty_new_admin_error() {
        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: "".into(),
        };
        assert!(msg.validate().is_err());
    }

    #[test]
    fn update_admin_msg_with_blank_new_admin_error() {
        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: "  ".into(),
        };
        assert!(msg.validate().is_err());
    }

    #[test]
    fn update_admin_msg_into_msg_no_error() {
        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
        };
        assert!(msg.validate().is_ok());
    }
}
