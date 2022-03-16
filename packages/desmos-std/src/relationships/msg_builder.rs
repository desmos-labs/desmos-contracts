use crate::relationships::msg::RelationshipsMsg;
use cosmwasm_std::{Addr, Uint64};

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
        counterparty: Addr,
        subspace_id: Uint64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::CreateRelationship {
            signer: sender,
            counterparty,
            subspace_id,
        }
    }

    pub fn delete_relationship(
        &self,
        user: Addr,
        counterparty: Addr,
        subspace_id: Uint64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::DeleteRelationship {
            signer: user,
            counterparty,
            subspace_id,
        }
    }

    pub fn block_user(
        &self,
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace_id: Uint64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::BlockUser {
            blocker,
            blocked,
            reason,
            subspace_id,
        }
    }

    pub fn unblock_user(
        &self,
        blocker: Addr,
        blocked: Addr,
        subspace_id: Uint64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::UnblockUser {
            blocker,
            blocked,
            subspace_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::relationships::{msg::RelationshipsMsg, msg_builder::RelationshipsMsgBuilder};
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_create_relationship() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.create_relationship(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Uint64::new(1),
        );
        let expected = RelationshipsMsg::CreateRelationship {
            signer: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            counterparty: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_relationship() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.delete_relationship(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Uint64::new(1),
        );
        let expected = RelationshipsMsg::DeleteRelationship {
            signer: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            counterparty: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace_id: Uint64::new(1),
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
            Uint64::new(1),
        );
        let expected = RelationshipsMsg::BlockUser {
            blocker: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            blocked: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            reason: "test".to_string(),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_unblock_user() {
        let builder = RelationshipsMsgBuilder::default();
        let msg = builder.unblock_user(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Uint64::new(1),
        );
        let expected = RelationshipsMsg::UnblockUser {
            blocker: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            blocked: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }
}
