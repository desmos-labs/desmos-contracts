use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cw_desmos_dtag_auctions::msg::{ExecuteMsg, InstantiateMsg, SudoMsg, QueryMsg};
use std::{env::current_dir, fs::create_dir_all};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("contracts/dtag-auctions/schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(SudoMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}
