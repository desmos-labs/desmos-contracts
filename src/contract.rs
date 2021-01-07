use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, MessageInfo, Querier, StdResult, Storage, Deps, DepsMut};

use crate::error::ContractError;
use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {

}

// And declare a custom Error variant for the ones where you will want to make use of it
pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {

    }
}

pub fn query(
    deps: DepsMut,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg:: {} => to_binary(),
    }
}
