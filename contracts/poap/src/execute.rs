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
    ) -> Result<Response<C>, ContractError> {
        // Instantiate the base cw721 that we are extending.
        let cw721_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            // Here we pass the admin as minter since the minter is the admin in the cw721-base.
            minter: msg.admin.unwrap_or(info.sender.to_string()),
        };
        self.cw721_base
            .instantiate(deps.branch(), env, info.clone(), cw721_instantiate_msg)?;

        // Save the poap metadata uri
        self.metadata_uri.save(deps.storage, &msg.metadata_uri)?;

        // Get the minter address or fallback to the sender address.
        let minter = msg
            .minter
            .map(|minter| deps.api.addr_validate(&minter))
            .transpose()?;
        self.minter.save(deps.storage, &minter)?;
        self.is_transferable
            .save(deps.storage, &msg.is_transferable)?;
        self.is_mintable.save(deps.storage, &msg.is_mintable)?;

        if msg.mint_start_time.is_some()
            && msg.mint_end_time.is_some()
            && msg.mint_start_time.unwrap() >= msg.mint_end_time.unwrap()
        {
            return Err(ContractError::InvalidTimestampValues {});
        }

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
    /// Transfer a POAP to another user.
    /// * `recipient` - Address of the user that will receive the POAP.
    /// * `token_id` - Id of the POAP to transfer.
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

    /// Send a POAP to a contract and trigger an action on the contract.
    /// * `contract` - Address of the contract that will receive the POAP.
    /// * `token_id` - Id of the POAP to transfer.
    /// * `msg` - Message that the recipient contract will execute.
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

    /// Updates who have the minting permissions, this action can be executed only from
    /// the contract admin.
    /// * `minter` - The new minter address.
    pub fn update_minter(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        minter: Option<String>,
    ) -> Result<Response<C>, ContractError> {
        self.assert_is_admin(deps.storage, &info.sender)?;
        let new_minter = minter.map(|m| deps.api.addr_validate(&m)).transpose()?;
        self.minter.save(deps.storage, &new_minter)?;

        Ok(Response::new()
            .add_attribute("action", "update_minter")
            .add_attribute("sender", info.sender)
            .add_attribute(
                "new_minter",
                new_minter.map_or_else(|| "none".to_string(), |minter| minter.to_string()),
            ))
    }

    /// Updates the POAP mintability, this action can be executed only from
    /// the contract admin.
    /// * `mintable` - true if the POAP can be minted from the users, false otherwise.
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

    /// Sets if the users are allowed to transfer their POAP, this action can be executed only from
    /// the contract admin.
    /// * `transferable` - true if the POAP can be transferred, false otherwise.
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

    /// Sets the time period on which the minting is allowed, this action can be executed only from
    /// the contract admin.
    /// * `start_time` - Timestamp at which the minting of the POAP will be enabled.
    /// If None, the minting is always enabled.
    /// * `end_time` - Timestamp at which the minting of the POAP will be disabled.    
    /// If not set, the minting will never end.
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
                start_time.map_or_else(|| "none".to_string(), |t| t.to_string()),
            )
            .add_attribute(
                "end_time",
                end_time.map_or_else(|| "none".to_string(), |t| t.to_string()),
            ))
    }

    /// Mint a POAP to the user that is calling this action.
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

    /// Mint a POAP to a list of user, this action can be executed only from the contract minter.
    /// * `users` - List of users for whom the POAP will be minted.
    pub fn mint_to(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        users: Vec<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        // Check if the sender is the admin or the minter.
        let can_mint = self.assert_is_minter(deps.storage, &info.sender).is_ok()
            || self.assert_is_admin(deps.storage, &info.sender).is_ok();

        if !can_mint {
            return Err(ContractError::MintUnauthorized {});
        }

        let mut minted_tokens = Vec::<String>::with_capacity(users.len());
        for user in users {
            let user_addr = deps.api.addr_validate(&user)?;
            self.assert_user_dont_own_a_poap(deps.storage, &user_addr)?;
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
    /// Computes the id of the next POAP to mint.
    pub fn generate_poap_id(&self, storage: &dyn Storage) -> StdResult<String> {
        Ok(format!("{}", 1 + self.cw721_base.token_count(storage)?))
    }

    /// Mint a POAP to an user.
    /// * `owner` - User for whom the POAP will be minted.
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
        let token_id = self.generate_poap_id(storage)?;
        self.cw721_base
            .tokens
            .update(storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.cw721_base.increment_tokens(storage)?;

        Ok(token_id)
    }

    /// Asserts that the provided address is the contract admin.
    /// * `sender` - Address that will be checked.
    pub fn assert_is_admin(
        &self,
        storage: &dyn Storage,
        sender: &Addr,
    ) -> Result<(), ContractError> {
        cw_ownable::assert_owner(storage, sender).map_err(|e| ContractError::Ownership(e))
    }

    /// Asserts that the provided address is the contract minter.
    /// * `sender` - Address that will be checked.
    pub fn assert_is_minter(
        &self,
        storage: &dyn Storage,
        sender: &Addr,
    ) -> Result<(), ContractError> {
        let minter = self.minter.load(storage)?;
        if minter.is_none() || minter.unwrap().ne(sender) {
            return Err(ContractError::MintUnauthorized {});
        }

        Ok(())
    }

    /// Check whether a user owns a POAP.
    /// * `user` - Address that will be checked.
    pub fn assert_user_dont_own_a_poap(
        &self,
        storage: &dyn Storage,
        user: &Addr,
    ) -> Result<(), ContractError> {
        // Check if this user have already minted a poap.
        if !self
            .cw721_base
            .tokens
            .idx
            .owner
            .prefix(user.clone())
            .is_empty(storage)
        {
            return Err(ContractError::PoapAlreadyMinted {
                user: user.to_string(),
            });
        }

        return Ok(());
    }

    /// Asserts if an user can mint a POAP.
    /// * `user` - Address that will be checked.
    pub fn assert_user_can_mint(
        &self,
        storage: &dyn Storage,
        user: &Addr,
        env: &Env,
    ) -> Result<(), ContractError> {
        // Check if this user have already minted a poap.
        self.assert_user_dont_own_a_poap(storage, user)?;

        // Check if the user is the minter.
        if self.assert_is_minter(storage, user).is_ok() {
            // The minter can always perform the mint operation.
            return Ok(());
        }

        // Check if the user is the admin.
        if self.assert_is_admin(storage, user).is_ok() {
            // The admin can always perform the mint operation.
            return Ok(());
        }

        // Check if mint is enabled
        if !self.is_mintable.load(storage)? {
            return Err(ContractError::MintDisabled {});
        }

        // Check if we have a mint start time
        if let Some(start_time) = self.mint_start_time.load(storage)? {
            // Check if the event has started.
            if start_time.gt(&env.block.time) {
                return Err(ContractError::MintTimeNotStarted {});
            }
        }

        // Check if we have a mint end time
        if let Some(end_time) = self.mint_end_time.load(storage)? {
            // Check if the event is still in progress.
            if env.block.time.ge(&end_time) {
                return Err(ContractError::MintTimeAlreadyEnded {});
            }
        }

        Ok(())
    }

    /// Asserts if the transfer is allowed.
    pub fn assert_is_transferable(&self, storage: &dyn Storage) -> Result<(), ContractError> {
        let is_transferable = self.is_transferable.load(storage)?;
        if !is_transferable {
            return Err(ContractError::TransferDisabled {});
        }

        Ok(())
    }
}
