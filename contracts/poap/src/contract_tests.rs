use crate::msg::MintStartEndTimeResponse;
use crate::ContractError::{
    EventNotStarted, EventTerminated, InvalidTimestampValues, MintDisabled, MintUnauthorized,
    Ownership, TransferDisabled,
};
use crate::ExecuteMsg::{
    Approve, ApproveAll, Burn, Mint, MintTo, Revoke, RevokeAll, SendNft, SetMintStartEndTime,
    SetMintable, SetTransferable, TransferNft, UpdateMinter,
};
use crate::{ExecuteMsg, Extension, InstantiateMsg, PoapContract};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{to_binary, Addr, CosmosMsg, Deps, DepsMut, Empty, Env, Timestamp, WasmMsg};
use cw721::{Cw721Query, NftInfoResponse};
use cw721_base::Cw721Contract;
use cw_ownable::OwnershipError;
use cw_utils::Expiration;

const ADMIN: &str = "admin";
const MINTER: &str = "minter";
const USER: &str = "user";
const CONTRACT: &str = "contract";
const CONTRACT_NAME: &str = "Test POAP";
const SYMBOL: &str = "TPOAP";
const METADATA_URI: &str = "ipfs://poap-metadata";

fn setup_contract(
    deps: DepsMut<'_>,
    is_transferable: bool,
    is_mintable: bool,
    mint_start_time: Option<Timestamp>,
    mint_end_time: Option<Timestamp>,
) -> PoapContract<'static, Extension, Empty, Empty, Empty> {
    let contract = PoapContract::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        metadata_uri: METADATA_URI.to_string(),
        admin: Some(ADMIN.to_string()),
        minter: Some(MINTER.to_string()),
        is_transferable,
        is_mintable,
        mint_start_time,
        mint_end_time,
    };
    let info = mock_info(ADMIN, &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

fn mock_env_with_time(timestamp: Timestamp) -> Env {
    let mut env = mock_env();
    env.block.time = timestamp;
    env
}

fn assert_poap_minted(
    contract: &PoapContract<Extension, Empty, Empty, Empty>,
    deps: Deps,
    poap_id: String,
    owner: String,
) {
    // Check if the poap has been minted
    let poap_info = contract.cw721_base.nft_info(deps, poap_id.clone()).unwrap();
    assert_eq!(
        NftInfoResponse {
            token_uri: Some(METADATA_URI.to_string()),
            extension: None,
        },
        poap_info
    );

    // Check if the user is the owner of the poap
    let owner_info = contract
        .cw721_base
        .owner_of(deps, mock_env(), poap_id, true)
        .unwrap();
    assert_eq!(owner, owner_info.owner);
}

#[test]
fn proper_instantiation_without_admin_and_minter() {
    let mut deps = mock_dependencies();
    let contract = PoapContract::<Extension, Empty, Empty, Empty>::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        metadata_uri: METADATA_URI.to_string(),
        admin: None,
        minter: None,
        is_transferable: false,
        is_mintable: true,
        mint_start_time: None,
        mint_end_time: None,
    };
    let info = mock_info(ADMIN, &[]);
    contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();

    // the instantiation worked, lets query ensure that the admin and minter are correct.
    let minter = contract.minter(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(Some(ADMIN.to_string()), minter.minter);

    let admin = Cw721Contract::<Extension, Empty, Empty, Empty>::ownership(deps.as_ref()).unwrap();
    assert_eq!(Some(Addr::unchecked(ADMIN)), admin.owner)
}

#[test]
fn proper_instantiation_without_minter() {
    let mut deps = mock_dependencies();
    let contract = PoapContract::<Extension, Empty, Empty, Empty>::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        metadata_uri: METADATA_URI.to_string(),
        admin: Some(ADMIN.to_string()),
        minter: None,
        is_transferable: false,
        is_mintable: true,
        mint_start_time: None,
        mint_end_time: None,
    };
    let info = mock_info(MINTER, &[]);
    contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();

    // Ensure that the minter is who instantiate the contract.
    let minter = contract.minter(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(Some(MINTER.to_string()), minter.minter);

    // Ensure that the admin is the value that has been passed in the InstantiateMsg message.
    let admin = Cw721Contract::<Extension, Empty, Empty, Empty>::ownership(deps.as_ref()).unwrap();
    assert_eq!(Some(Addr::unchecked(ADMIN)), admin.owner)
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = PoapContract::<Extension, Empty, Empty, Empty>::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        metadata_uri: METADATA_URI.to_string(),
        admin: Some(ADMIN.to_string()),
        minter: Some(MINTER.to_string()),
        is_transferable: false,
        is_mintable: true,
        mint_start_time: None,
        mint_end_time: None,
    };
    let info = mock_info(ADMIN, &[]);
    contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();

    // Ensure that the minter is the value that has been passed in the InstantiateMsg message.
    let minter = contract.minter(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(Some(MINTER.to_string()), minter.minter);

    // Ensure that the admin is the value that has been passed in the InstantiateMsg message.
    let admin = Cw721Contract::<Extension, Empty, Empty, Empty>::ownership(deps.as_ref()).unwrap();
    assert_eq!(Some(Addr::unchecked(ADMIN)), admin.owner)
}

#[test]
fn user_can_mint_if_mintable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();

    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), info, Mint { extension: None })
        .unwrap();

    assert_poap_minted(&contract, deps.as_ref(), poap_id, USER.to_string());
}

#[test]
fn user_cant_mint_if_not_mintable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, false, None, None);

    // Check that the user can't mint if the poap is not mintable
    let user_info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
    assert_eq!(MintDisabled {}, err);
}

#[test]
fn minter_can_mint_if_not_mintable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, false, None, None);
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();

    // Check that the user can't mint if the poap is not mintable
    let user_info = mock_info(MINTER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            user_info,
            Mint { extension: None },
        )
        .unwrap();

    assert_poap_minted(&contract, deps.as_ref(), poap_id, MINTER.to_string());
}

#[test]
fn user_can_mint_after_start_time() {
    let mut deps = mock_dependencies();
    let start_time = Timestamp::from_seconds(200);
    let contract = setup_contract(deps.as_mut(), true, true, Some(start_time.clone()), None);
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let env_event_started = mock_env_with_time(start_time);

    // Check that an user can mint after the mint start time.
    let user_info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            env_event_started,
            user_info,
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, USER.to_string());
}

#[test]
fn user_cant_mint_before_start_time() {
    let mut deps = mock_dependencies();

    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        Some(Timestamp::from_seconds(200)),
        None,
    );

    let env_event_not_started = mock_env_with_time(Timestamp::from_seconds(199));

    // Check that an user can't mint before the mint start time.
    let user_info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            env_event_not_started,
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
    assert_eq!(EventNotStarted {}, err);
}

#[test]
fn minter_can_mint_before_start_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        Some(Timestamp::from_seconds(200)),
        None,
    );
    let env_event_not_started = mock_env_with_time(Timestamp::from_seconds(199));
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();

    // Check that an user can't mint before the mint start time.
    let user_info = mock_info(MINTER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            env_event_not_started,
            user_info,
            Mint { extension: None },
        )
        .unwrap();

    assert_poap_minted(&contract, deps.as_ref(), poap_id, MINTER.to_string())
}

#[test]
fn user_can_mint_before_end_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        None,
        Some(Timestamp::from_seconds(200)),
    );
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let env_before_end_time = mock_env_with_time(Timestamp::from_seconds(199));

    // Check that an user can mint after the mint start time.
    let user_info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            env_before_end_time,
            user_info,
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, USER.to_string());
}

#[test]
fn user_cant_mint_after_end_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        None,
        Some(Timestamp::from_seconds(200)),
    );
    let env_after_end_time = mock_env_with_time(Timestamp::from_seconds(200));

    // Check that an user can't mint before the mint start time.
    let user_info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            env_after_end_time,
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
    assert_eq!(EventTerminated {}, err);
}

#[test]
fn minter_can_mint_after_end_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        None,
        Some(Timestamp::from_seconds(200)),
    );
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let env_after_end_time = mock_env_with_time(Timestamp::from_seconds(200));

    // Check that an user can't mint before the mint start time.
    let user_info = mock_info(MINTER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            env_after_end_time,
            user_info,
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, MINTER.to_string());
}

#[test]
fn user_can_mint_during_mint_time() {
    let mut deps = mock_dependencies();

    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        Some(Timestamp::from_seconds(100)),
        Some(Timestamp::from_seconds(200)),
    );
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();

    // Check that user can mint during mint time
    let user_info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(150)),
            user_info,
            Mint { extension: None },
        )
        .unwrap();

    assert_poap_minted(&contract, deps.as_ref(), poap_id, USER.to_string());
}

#[test]
fn user_cant_mint_outside_mint_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        Some(Timestamp::from_seconds(100)),
        Some(Timestamp::from_seconds(200)),
    );

    // Check that user can't mint before start time
    let user_info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(99)),
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
    assert_eq!(EventNotStarted {}, err);

    // Check that user can't mint after end time
    let user_info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(200)),
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
    assert_eq!(EventTerminated {}, err);
}

#[test]
fn minter_can_mint_outside_mint_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(
        deps.as_mut(),
        true,
        true,
        Some(Timestamp::from_seconds(100)),
        Some(Timestamp::from_seconds(200)),
    );

    // Check that minter can mint before start time
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let user_info = mock_info(MINTER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(99)),
            user_info,
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, MINTER.to_string());

    // Check that minter can mint after end time
    let user_info = mock_info(MINTER, &[]);
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(200)),
            user_info,
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, MINTER.to_string());
}

#[test]
fn user_cant_mint_for_other_users() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            MintTo {
                users: vec![MINTER.to_string()],
                extension: None,
            },
        )
        .unwrap_err();
    assert_eq!(MintUnauthorized {}, err);
}

#[test]
fn minter_can_mint_for_other_users() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();

    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            MintTo {
                users: vec![USER.to_string()],
                extension: None,
            },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, USER.to_string());
}

#[test]
fn can_transfer_if_transferable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    // Mint a poap
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), info, Mint { extension: None })
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Transfer the poap to another user
    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            info,
            TransferNft {
                recipient: ADMIN.to_string(),
                token_id: poap_id.clone(),
            },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id, ADMIN.to_string());
}

#[test]
fn cant_transfer_if_not_transferable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), false, true, None, None);

    // Mint a poap
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), info, Mint { extension: None })
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Transfer the poap to another user
    let info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            info,
            TransferNft {
                recipient: ADMIN.to_string(),
                token_id: poap_id,
            },
        )
        .unwrap_err();
    assert_eq!(TransferDisabled {}, err);
}

#[test]
fn can_send_if_transferable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    // Mint a poap
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), info, Mint { extension: None })
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Send the poap to a contract
    let inner_msg = WasmMsg::Execute {
        contract_addr: "another_contract".into(),
        msg: to_binary("You now also have the growing power").unwrap(),
        funds: vec![],
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(inner_msg);
    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            info,
            SendNft {
                contract: CONTRACT.to_string(),
                msg: to_binary(&msg).unwrap(),
                token_id: poap_id.clone(),
            },
        )
        .unwrap();
}

#[test]
fn cant_send_if_not_transferable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), false, true, None, None);

    // Mint a poap
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Transfer the poap to another user
    // Send the poap to a contract
    let inner_msg = WasmMsg::Execute {
        contract_addr: "another_contract".into(),
        msg: to_binary("You now also have the growing power").unwrap(),
        funds: vec![],
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(inner_msg);
    let info = mock_info(USER, &[]);
    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            info,
            SendNft {
                contract: CONTRACT.to_string(),
                msg: to_binary(&msg).unwrap(),
                token_id: poap_id.clone(),
            },
        )
        .unwrap_err();
    assert_eq!(TransferDisabled {}, err);
}

#[test]
fn only_admin_can_update_the_minter() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            UpdateMinter {
                minter: USER.to_string(),
            },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            UpdateMinter {
                minter: USER.to_string(),
            },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            UpdateMinter {
                minter: USER.to_string(),
            },
        )
        .unwrap();

    let minter = contract.minter(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(USER.to_string(), minter.minter.unwrap());
}

#[test]
fn only_admin_can_set_mintable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            SetMintable { mintable: false },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            SetMintable { mintable: false },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            SetMintable { mintable: false },
        )
        .unwrap();

    let is_mintable = contract.is_mintable(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(false, is_mintable.is_mintable);
}

#[test]
fn only_admin_can_set_transferable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            SetTransferable {
                transferable: false,
            },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            SetTransferable {
                transferable: false,
            },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            SetTransferable {
                transferable: false,
            },
        )
        .unwrap();

    let is_transferable = contract.is_transferable(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(false, is_transferable.is_transferable);
}

#[test]
fn only_admin_can_set_start_end_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            SetMintStartEndTime {
                start_time: Some(Timestamp::from_seconds(1)),
                end_time: Some(Timestamp::from_seconds(200)),
            },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            SetMintStartEndTime {
                start_time: Some(Timestamp::from_seconds(1)),
                end_time: Some(Timestamp::from_seconds(200)),
            },
        )
        .unwrap_err();
    assert_eq!(Ownership(OwnershipError::NotOwner), err);

    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            SetMintStartEndTime {
                start_time: Some(Timestamp::from_seconds(1)),
                end_time: Some(Timestamp::from_seconds(200)),
            },
        )
        .unwrap();

    let start_end_time = contract
        .mint_start_end_time(deps.as_ref(), mock_env())
        .unwrap();
    assert_eq!(
        MintStartEndTimeResponse {
            start_time: Some(Timestamp::from_seconds(1)),
            end_time: Some(Timestamp::from_seconds(200))
        },
        start_end_time
    );
}

#[test]
fn start_time_cant_be_higher_equal_then_end_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            SetMintStartEndTime {
                start_time: Some(Timestamp::from_seconds(200)),
                end_time: Some(Timestamp::from_seconds(200)),
            },
        )
        .unwrap_err();
    assert_eq!(InvalidTimestampValues {}, err);

    let err = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            SetMintStartEndTime {
                start_time: Some(Timestamp::from_seconds(201)),
                end_time: Some(Timestamp::from_seconds(200)),
            },
        )
        .unwrap_err();
    assert_eq!(InvalidTimestampValues {}, err);
}

#[test]
fn can_burn_their_poap() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    // Compute the id of the poap that will be minted
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Try to burn the minted poap
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            Burn { token_id: poap_id },
        )
        .unwrap();

    // Check that the poap has been burned
    let token_count = contract.cw721_base.token_count(&deps.storage).unwrap();
    assert_eq!(0, token_count);
}

#[test]
fn can_approve_and_revoke() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    // Compute the id of the poap that will be minted
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    // Mint a poap
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Test Approve message
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            Approve {
                token_id: poap_id.clone(),
                spender: MINTER.to_string(),
                expires: Some(Expiration::Never {}),
            },
        )
        .unwrap();

    // Test Revoke message
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            Revoke {
                token_id: poap_id,
                spender: MINTER.to_string(),
            },
        )
        .unwrap();
}

#[test]
fn can_approve_all_and_revoke_all() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, true, None, None);

    // Compute the id of the poap that will be minted
    let poap_id = contract.generate_poap_id(&deps.storage).unwrap();
    // Mint a poap
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            Mint { extension: None },
        )
        .unwrap();
    assert_poap_minted(&contract, deps.as_ref(), poap_id.clone(), USER.to_string());

    // Test ApproveAll message
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            ApproveAll {
                operator: MINTER.to_string(),
                expires: Some(Expiration::Never {}),
            },
        )
        .unwrap();

    // Test RevokeAll message
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER, &[]),
            RevokeAll {
                operator: MINTER.to_string(),
            },
        )
        .unwrap();
}
