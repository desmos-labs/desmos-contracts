use crate::ExecuteMsg::Mint;
use crate::{Extension, InstantiateMsg, PoapContract};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Addr, DepsMut, Empty, Env, Timestamp};
use cw721::{Cw721Query, NftInfoResponse};
use cw721_base::Cw721Contract;

const ADMIN: &str = "admin";
const MINTER: &str = "minter";
const USER: &str = "user";
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

    let poap_id = format!(
        "{}",
        contract.cw721_base.token_count(&deps.storage).unwrap()
    );

    let info = mock_info(USER, &[]);
    let _ = contract
        .execute(deps.as_mut(), mock_env(), info, Mint { extension: None })
        .unwrap();

    // Check if the poap has been minted
    let poap_info = contract
        .cw721_base
        .nft_info(deps.as_ref(), poap_id.clone())
        .unwrap();
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
        .owner_of(deps.as_ref(), mock_env(), poap_id, true)
        .unwrap();
    assert_eq!(USER.to_string(), owner_info.owner);
}

#[test]
fn user_cant_mint_if_not_mintable() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut(), true, false, None, None);

    // Check that the user can't mint if the poap is not mintable
    let user_info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env(),
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
}

#[test]
fn user_can_mint_after_start_time() {
    let mut deps = mock_dependencies();

    let start_time = Timestamp::from_seconds(200);
    let contract = setup_contract(deps.as_mut(), true, true, Some(start_time.clone()), None);

    let poap_id = format!(
        "{}",
        contract.cw721_base.token_count(&deps.storage).unwrap()
    );
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

    // Check if the poap has been minted
    let poap_info = contract
        .cw721_base
        .nft_info(deps.as_ref(), poap_id.clone())
        .unwrap();
    assert_eq!(
        NftInfoResponse {
            token_uri: Some(METADATA_URI.to_string()),
            extension: None,
        },
        poap_info
    );

    // Check if user is the owner of the poap
    let owner_info = contract
        .cw721_base
        .owner_of(deps.as_ref(), mock_env(), poap_id, true)
        .unwrap();
    assert_eq!(USER.to_string(), owner_info.owner);
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
    let _ = contract
        .execute(
            deps.as_mut(),
            env_event_not_started,
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
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

    let poap_id = format!(
        "{}",
        contract.cw721_base.token_count(&deps.storage).unwrap()
    );
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

    // Check if the poap has been minted
    let poap_info = contract
        .cw721_base
        .nft_info(deps.as_ref(), poap_id.clone())
        .unwrap();
    assert_eq!(
        NftInfoResponse {
            token_uri: Some(METADATA_URI.to_string()),
            extension: None,
        },
        poap_info
    );

    // Check if user is the owner of the poap
    let owner_info = contract
        .cw721_base
        .owner_of(deps.as_ref(), mock_env(), poap_id, true)
        .unwrap();
    assert_eq!(USER.to_string(), owner_info.owner);
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
    let _ = contract
        .execute(
            deps.as_mut(),
            env_after_end_time,
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
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

    let poap_id = format!(
        "{}",
        contract.cw721_base.token_count(&deps.storage).unwrap()
    );

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

    // Check if the poap has been minted
    let poap_info = contract
        .cw721_base
        .nft_info(deps.as_ref(), poap_id.clone())
        .unwrap();
    assert_eq!(
        NftInfoResponse {
            token_uri: Some(METADATA_URI.to_string()),
            extension: None,
        },
        poap_info
    );

    // Check if user is the owner of the poap
    let owner_info = contract
        .cw721_base
        .owner_of(deps.as_ref(), mock_env(), poap_id, true)
        .unwrap();
    assert_eq!(USER.to_string(), owner_info.owner);
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
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(99)),
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();

    // Check that user can't mint after end time
    let user_info = mock_info(USER, &[]);
    let _ = contract
        .execute(
            deps.as_mut(),
            mock_env_with_time(Timestamp::from_seconds(200)),
            user_info,
            Mint { extension: None },
        )
        .unwrap_err();
}
