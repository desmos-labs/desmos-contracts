use crate::{
    query_types::{DesmosQuery, DesmosQueryWrapper},
    types::DesmosRoute,
};
use cosmwasm_std::{QuerierWrapper, StdResult};

pub struct DesmosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>,
}

impl<'a> DesmosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, DesmosQueryWrapper>) -> Self {
        DesmosQuerier { querier }
    }
}
