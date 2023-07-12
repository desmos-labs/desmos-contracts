use crate::msg::QueryMsg;
use crate::state::PoapContract;
use cosmwasm_std::{to_binary, Binary, CustomMsg, Deps, Env, StdResult};
use cw721_base::MinterResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

impl<'a, T, C, E, Q> PoapContract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone + Debug,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            QueryMsg::Minter {} => to_binary(&MinterResponse {
                minter: Some(self.minter.load(deps.storage)?.to_string()),
            }),
            _ => self.cw721_base.query(deps, env, msg.into()),
        }
    }
}
