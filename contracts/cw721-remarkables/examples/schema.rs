use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema_with_title, remove_schemas, schema_for};
use cw721_remarkables::ExecuteMsg;
use cw721::{AllNftInfoResponse, NftInfoResponse};
use cw721_remarkables::Metadata;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "ExecuteMsg");
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<Metadata>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema_with_title(
        &schema_for!(NftInfoResponse<Metadata>),
        &out_dir,
        "NftInfoResponse",
    );
}
