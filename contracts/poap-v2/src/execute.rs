use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::PoapContract;
use cosmwasm_std::{
    Addr, Binary, CustomMsg, DepsMut, Env, MessageInfo, Response, StdResult, Storage, Timestamp,
};
use cw721::Cw721Execute;
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
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => self.transfer_poap(deps, env, info, recipient, token_id),
            ExecuteMsg::SendNft {
                contract,
                msg,
                token_id,
            } => self.send_poap(deps, env, info, contract, token_id, msg),
            ExecuteMsg::UpdateMinter { minter } => self.update_minter(deps, env, info, minter),
            ExecuteMsg::SetMintable { mintable } => self.set_mintable(deps, env, info, mintable),
            ExecuteMsg::SetTransferable { transferable } => {
                self.set_transferable(deps, env, info, transferable)
            }
            ExecuteMsg::SetMintStartEndTime {
                start_time,
                end_time,
            } => self.set_mint_start_end_time(deps, env, info, start_time, end_time),
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
    pub fn transfer_poap(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        // Check if the transfer is allowed.
        self.assert_is_transferable(deps.storage)?;

        return self
            .cw721_base
            .transfer_nft(deps, env, info, recipient, token_id)
            .map_err(|e| e.into());
    }

    pub fn send_poap(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, ContractError> {
        // Check if the transfer is allowed.
        self.assert_is_transferable(deps.storage)?;

        return self
            .cw721_base
            .send_nft(deps, env, info, contract, token_id, msg)
            .map_err(|e| e.into());
    }

    pub fn update_minter(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        minter: String,
    ) -> Result<Response<C>, ContractError> {
        self.assert_is_admin(deps.storage, &info.sender)?;
        let minter_addr = deps.api.addr_validate(&minter)?;
        self.minter.save(deps.storage, &minter_addr)?;

        Ok(Response::new()
            .add_attribute("action", "update_minter")
            .add_attribute("sender", info.sender)
            .add_attribute("new_minter", minter_addr))
    }

    pub fn set_mintable(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        mintable: bool,
    ) -> Result<Response<C>, ContractError> {
        self.assert_is_admin(deps.storage, &info.sender)?;
        self.is_mintable.save(deps.storage, &mintable)?;

        Ok(Response::new()
            .add_attribute("action", "set_mintable")
            .add_attribute("sender", info.sender)
            .add_attribute("mintable", mintable.to_string()))
    }

    pub fn set_transferable(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        transferable: bool,
    ) -> Result<Response<C>, ContractError> {
        self.assert_is_admin(deps.storage, &info.sender)?;
        self.is_transferable.save(deps.storage, &transferable)?;

        Ok(Response::new()
            .add_attribute("action", "set_transferable")
            .add_attribute("sender", info.sender)
            .add_attribute("transferable", transferable.to_string()))
    }

    pub fn set_mint_start_end_time(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
    ) -> Result<Response<C>, ContractError> {
        self.assert_is_admin(deps.storage, &info.sender)?;

        // Ensure that if we have both start time and end time the start time is lower then
        // the end time.
        if start_time.is_some() && end_time.is_some() && start_time.unwrap() >= end_time.unwrap() {
            return Err(ContractError::InvalidTimestampValues {});
        }

        // Update the start and end time
        self.mint_start_time.save(deps.storage, &start_time)?;
        self.mint_end_time.save(deps.storage, &end_time)?;

        Ok(Response::new()
            .add_attribute("action", "set_mint_start_end_time")
            .add_attribute("sender", info.sender)
            .add_attribute(
                "start_time",
                start_time.map_or_else(|| "None".to_string(), |t| t.to_string()),
            )
            .add_attribute(
                "end_time",
                end_time.map_or_else(|| "None".to_string(), |t| t.to_string()),
            ))
    }

    pub fn mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        self.assert_user_can_mint(deps.storage, &info.sender, &env)?;
        let token_id = self.mint_to_user(deps.storage, &info.sender, extension)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("owner", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn mint_to(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        users: Vec<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        self.assert_is_minter(deps.storage, &info.sender)?;

        let mut minted_tokens = Vec::<String>::with_capacity(users.len());
        for user in users {
            let user_addr = deps.api.addr_validate(&user)?;
            minted_tokens.push(self.mint_to_user(deps.storage, &user_addr, extension.clone())?);
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
    /// Mint a POAP to an user.
    pub fn mint_to_user(
        &self,
        storage: &mut dyn Storage,
        owner: &Addr,
        extension: T,
    ) -> Result<String, ContractError> {
        // Create the token
        let token = cw721_base::state::TokenInfo {
            owner: owner.clone(),
            approvals: vec![],
            token_uri: Some(self.metadata_uri.load(storage)?),
            extension,
        };

        // Generate the token id
        let token_id = format!("{}", self.cw721_base.token_count(storage)?);

        self.cw721_base
            .tokens
            .update(storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.cw721_base.increment_tokens(storage)?;

        Ok(token_id)
    }

    pub fn assert_is_admin(
        &self,
        storage: &dyn Storage,
        sender: &Addr,
    ) -> Result<(), ContractError> {
        cw_ownable::assert_owner(storage, sender).map_err(|e| ContractError::Ownership(e))
    }

    /// Checks if who has sent the message is the minter.
    pub fn assert_is_minter(
        &self,
        storage: &dyn Storage,
        sender: &Addr,
    ) -> Result<(), ContractError> {
        let minter = self.minter.load(storage)?;
        if !sender.eq(&minter) {
            return Err(ContractError::MintUnauthorized {});
        }

        Ok(())
    }

    /// Checks if an user can mint a POAP.
    pub fn assert_user_can_mint(
        &self,
        storage: &dyn Storage,
        user: &Addr,
        env: &Env,
    ) -> Result<(), ContractError> {
        // Check if the user is the minter.
        if self.minter.load(storage)?.eq(user) {
            // The minter can always perform the mint operation.
            return Ok(());
        }

        // Check if mint is enabled
        if !self.is_mintable.load(storage)? {
            return Err(ContractError::MintDisabled {});
        }

        // Check if we have a mint start time
        if let Some(start_time) = self.mint_start_time.load(storage)? {
            // Check if the event has started.
            if start_time.ge(&env.block.time) {
                return Err(ContractError::EventNotStarted {});
            }
        }

        // Check if we have a mint end time
        if let Some(end_time) = self.mint_end_time.load(storage)? {
            // Check if the event is still in progress.
            if env.block.time.ge(&end_time) {
                return Err(ContractError::EventTerminated {});
            }
        }

        Ok(())
    }

    pub fn assert_is_transferable(&self, storage: &dyn Storage) -> Result<(), ContractError> {
        let is_transferable = self.is_transferable.load(storage)?;
        if !is_transferable {
            return Err(ContractError::TransferDisabled {});
        }

        Ok(())
    }
}
