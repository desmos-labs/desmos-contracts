use crate::relationships::msg::RelationshipsMsg;
use cosmwasm_std::Addr;

pub struct RelationshipsMsgBuilder {}

impl RelationshipsMsgBuilder {
    pub fn new() -> Self {
        RelationshipsMsgBuilder {}
    }
}

impl Default for RelationshipsMsgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipsMsgBuilder {
    pub fn create_relationship(
        &self,
        sender: Addr,
        receiver: Addr,
        subspace: String,
    ) -> RelationshipsMsg {
        RelationshipsMsg::CreateRelationship {
            sender,
            receiver,
            subspace,
        }
    }

    pub fn delete_relationship(
        &self,
        user: Addr,
        counterparty: Addr,
        subspace: String,
    ) -> RelationshipsMsg {
        RelationshipsMsg::DeleteRelationship {
            user,
            counterparty,
            subspace,
        }
    }

    pub fn block_user(
        &self,
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace: String,
    ) -> RelationshipsMsg {
        RelationshipsMsg::BlockUser {
            blocker,
            blocked,
            reason,
            subspace,
        }
    }

    pub fn unblock_user(&self, blocker: Addr, blocked: Addr, subspace: String) -> RelationshipsMsg {
        RelationshipsMsg::UnblockUser {
            blocker,
            blocked,
            subspace,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::relationships::{msg::RelationshipsMsg, msg_builder::RelationshipsMsgBuilder};
    use cosmwasm_std::Addr;

    #[test]
    fn test_create_relationship() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.create_relationship(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            "1".to_string(),
        );
        let expected = RelationshipsMsg::CreateRelationship {
            sender: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            receiver: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace: "1".to_string(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_relationship() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.delete_relationship(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            "1".to_string(),
        );
        let expected = RelationshipsMsg::DeleteRelationship {
            user: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            counterparty: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace: "1".to_string(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_block_user() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.block_user(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            "test".to_string(),
            "1".to_string(),
        );
        let expected = RelationshipsMsg::BlockUser {
            blocker: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            blocked: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            reason: "test".to_string(),
            subspace: "1".to_string(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_unblock_user() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.unblock_user(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            "1".to_string(),
        );
        let expected = RelationshipsMsg::UnblockUser {
            blocker: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            blocked: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace: "1".to_string(),
        };
        assert_eq!(expected, msg)
    }
}
