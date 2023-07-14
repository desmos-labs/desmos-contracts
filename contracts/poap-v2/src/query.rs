use crate::msg::{IsMintableResponse, IsTransferableResponse, MintStartEndTimeResponse, QueryMsg};
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
            QueryMsg::Minter {} => to_binary(&self.minter(deps, env)?),
            QueryMsg::IsMintable {} => to_binary(&self.is_mintable(deps, env)?),
            QueryMsg::IsTransferable {} => to_binary(&self.is_transferable(deps, env)?),
            QueryMsg::MintStartEndTime {} => to_binary(&self.mint_start_end_time(deps, env)?),
            _ => self.cw721_base.query(deps, env, msg.into()),
        }
    }
}

impl<'a, T, C, E, Q> PoapContract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone + Debug,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn minter(&self, deps: Deps, _env: Env) -> StdResult<MinterResponse> {
        Ok(MinterResponse {
            minter: Some(self.minter.load(deps.storage)?.to_string()),
        })
    }

    pub fn is_mintable(&self, deps: Deps, _env: Env) -> StdResult<IsMintableResponse> {
        Ok(IsMintableResponse {
            is_mintable: self.is_mintable.load(deps.storage)?,
        })
    }

    pub fn is_transferable(&self, deps: Deps, _env: Env) -> StdResult<IsTransferableResponse> {
        Ok(IsTransferableResponse {
            is_transferable: self.is_transferable.load(deps.storage)?,
        })
    }

    pub fn mint_start_end_time(
        &self,
        deps: Deps,
        _env: Env,
    ) -> StdResult<MintStartEndTimeResponse> {
        Ok(MintStartEndTimeResponse {
            start_time: self.mint_start_time.load(deps.storage)?,
            end_time: self.mint_end_time.load(deps.storage)?,
        })
    }
}
