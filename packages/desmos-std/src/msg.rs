use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::msg::ProfilesMsg, subspaces::msg::SubspacesMsg, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosMsg {
    pub route: DesmosRoute,
    pub msg_data: DesmosMsgRoute,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosMsgRoute {
    Profiles(ProfilesMsg),
    Subspaces(SubspacesMsg),
}

impl Into<CosmosMsg<DesmosMsg>> for DesmosMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        CosmosMsg::Custom(self)
    }
}
impl CustomMsg for DesmosMsg {}

impl From<ProfilesMsg> for DesmosMsg {
    fn from(msg: ProfilesMsg) -> Self {
        Self {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsgRoute::Profiles(msg),
        }
    }
}

impl From<SubspacesMsg> for DesmosMsg {
    fn from(msg: SubspacesMsg) -> Self {
        Self {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsgRoute::Subspaces(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        msg::{DesmosMsg, DesmosMsgRoute},
        profiles::msg::ProfilesMsg,
        types::DesmosRoute,
    };
    use cosmwasm_std::Addr;

    #[test]
    fn test_from_profile_msg() {
        let msg = ProfilesMsg::CreateRelationship {
            sender: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            receiver: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            subspace: "1".to_string(),
        };
        let expected = DesmosMsg {
            route: DesmosRoute::Profiles,
            msg_data: DesmosMsgRoute::Profiles(msg.clone()),
        };
        assert_eq!(expected, DesmosMsg::from(msg))
    }

    #[test]
    fn test_from_subspaces_msg() {
        let msg = SubspacesMsg::CreateSubspace {
            name: "test".to_string(),
            description: "test".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        let expected = DesmosMsg {
            route: DesmosRoute::Subspaces,
            msg_data: DesmosMsgRoute::Subspaces(msg.clone()),
        };
        assert_eq!(expected, DesmosMsg::from(msg));
    }
}
