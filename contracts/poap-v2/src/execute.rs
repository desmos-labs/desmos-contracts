use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::PoapContract;
use cosmwasm_std::{CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    Cw721Contract, Extension, InstantiateMsg as Cw721BaseInstantiateMsg, MinterResponse,
};
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
    pub fn instantiate(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response<C>> {
        // Instantiate the base cw721 that we are extending.
        let cw721_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            // Here we pass the admin as minter since the minter is the admin in the cw721-base.
            minter: msg.admin.unwrap_or(info.sender.to_string()),
        };
        self.cw721_base
            .instantiate(deps.branch(), env, info.clone(), cw721_instantiate_msg)?;

        self.metadata_uri.save(deps.storage, &msg.metadata_uri)?;
        let minter = msg
            .minter
            .map(|minter| deps.api.addr_validate(&minter))
            .transpose()?
            .unwrap_or(info.sender);
        self.minter.save(deps.storage, &minter)?;
        self.is_transferable
            .save(deps.storage, &msg.is_transferable)?;
        self.is_mintable.save(deps.storage, &msg.is_mintable)?;
        self.mint_start_time
            .save(deps.storage, &msg.mint_start_time)?;
        self.mint_end_time.save(deps.storage, &msg.mint_end_time)?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<T, E>,
    ) -> Result<Response<C>, ContractError> {
        match msg {
            ExecuteMsg::Mint { extension } => self.mint(deps, env, info, extension),
            ExecuteMsg::MintTo { extension, users } => {
                self.mint_to(deps, env, info, users, extension)
            }
            _ => self
                .cw721_base
                .execute(deps, env, info, msg.into())
                .map_err(|e| e.into()),
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
    pub fn mint(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        self.check_user_can_mint(deps.as_ref(), &env, &info)?;
        let token_id = self.mint_to_user(deps.branch(), info.sender.to_string(), extension)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("owner", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn mint_to(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        users: Vec<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        self.check_is_minter(deps.as_ref(), &env, &info)?;

        let mut minted_tokens = Vec::<String>::with_capacity(users.len());
        for user in users {
            minted_tokens.push(self.mint_to_user(deps.branch(), user, extension.clone())?);
        }

        Ok(Response::new()
            .add_attribute("action", "mint_to")
            .add_attribute("minter", info.sender)
            .add_attribute("token_ids", minted_tokens.join(", ")))
    }
}

// Utility functions
impl<'a, T, C, E, Q> PoapContract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone + Debug,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    /// Checks if an user can mint a POAP.
    pub fn check_user_can_mint(
        &self,
        deps: Deps,
        env: &Env,
        _info: &MessageInfo,
    ) -> Result<(), ContractError> {
        // Check if mint is enabled
        if !self.is_mintable.load(deps.storage)? {
            return Err(ContractError::MintDisabled {});
        }

        // Check if we have a mint start time
        if let Some(start_time) = self.mint_start_time.load(deps.storage)? {
            // Check if the event has started.
            if start_time.ge(&env.block.time) {
                return Err(ContractError::EventNotStarted {});
            }
        }

        // Check if we have a mint end time
        if let Some(end_time) = self.mint_end_time.load(deps.storage)? {
            // Check if the event is still in progress.
            if env.block.time.ge(&end_time) {
                return Err(ContractError::EventTerminated {});
            }
        }

        Ok(())
    }

    /// Mint a POAP to an user.
    pub fn mint_to_user(
        &self,
        deps: DepsMut,
        owner: String,
        extension: T,
    ) -> Result<String, ContractError> {
        // Create the token
        let token = cw721_base::state::TokenInfo {
            owner: deps.api.addr_validate(&owner)?,
            approvals: vec![],
            token_uri: Some(self.metadata_uri.load(deps.storage)?),
            extension,
        };

        // Generate the token id
        let token_id = format!("{}", self.cw721_base.token_count(deps.storage)?);

        self.cw721_base
            .tokens
            .update(deps.storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.cw721_base.increment_tokens(deps.storage)?;

        Ok(token_id)
    }

    /// Checks if who has sent the message is the minter.
    pub fn check_is_minter(
        &self,
        deps: Deps,
        _env: &Env,
        info: &MessageInfo,
    ) -> Result<(), ContractError> {
        let minter = self.minter.load(deps.storage)?;
        if !info.sender.eq(&minter) {
            return Err(ContractError::MintUnauthorized {});
        }

        Ok(())
    }
}
