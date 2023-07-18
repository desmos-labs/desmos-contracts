use cosmwasm_std::{Addr, CustomMsg, Timestamp};
use cw721_base::Cw721Contract;
use cw_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct PoapContract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomMsg,
    E: CustomMsg,
{
    pub cw721_base: Cw721Contract<'a, T, C, E, Q>,
    /// The URI where users can view the associated metadata for the POAPs,
    /// ideally following the ERC-721 metadata scheme in a JSON file.
    pub metadata_uri: Item<'a, String>,
    /// Additional address that is allowed to mint tokens on behalf of other users.
    pub minter: Item<'a, Option<Addr>>,
    /// Specifies whether each POAP can be transferred from one user to another.
    pub is_transferable: Item<'a, bool>,
    /// Indicates whether users can mint the POAPs.
    pub is_mintable: Item<'a, bool>,
    /// Identifies the timestamp at which the minting of the POAP will be enabled.
    /// If not set, the minting is always enabled.
    pub mint_start_time: Item<'a, Option<Timestamp>>,
    /// Identifies the timestamp at which the minting of the POAP will be disabled.
    /// If not set, the minting will never end.
    pub mint_end_time: Item<'a, Option<Timestamp>>,
}

impl<'a, T, C, E, Q> PoapContract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn new(
        metadata_uri_key: &'a str,
        minter_key: &'a str,
        is_transferable_key: &'a str,
        is_mintable_key: &'a str,
        mint_start_time_key: &'a str,
        mint_end_time_key: &'a str,
    ) -> Self {
        Self {
            cw721_base: Cw721Contract::default(),
            metadata_uri: Item::new(metadata_uri_key),
            minter: Item::new(minter_key),
            is_transferable: Item::new(is_transferable_key),
            is_mintable: Item::new(is_mintable_key),
            mint_start_time: Item::new(mint_start_time_key),
            mint_end_time: Item::new(mint_end_time_key),
        }
    }
}

impl<T, C, E, Q> Default for PoapContract<'static, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn default() -> Self {
        Self::new(
            "poap_metadata_uri",
            "minter",
            "is_transferable",
            "is_mintable",
            "mint_start_time",
            "mint_end_time",
        )
    }
}
