use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use desmos_contracts::msg::{HandleMsg, InitMsg, QueryMsg};
use desmos_contracts::query::{PostsQueryResponse, ReportsQueryResponse};
use desmos_contracts::state::State;
use desmos_contracts::types::{Attachment, OptionalData, PollAnswer, PollData, Post, Report};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(OptionalData), &out_dir);
    export_schema(&schema_for!(Attachment), &out_dir);
    export_schema(&schema_for!(PollData), &out_dir);
    export_schema(&schema_for!(PollAnswer), &out_dir);
    export_schema(&schema_for!(Post), &out_dir);
    export_schema(&schema_for!(Report), &out_dir);
    export_schema(&schema_for!(PostsQueryResponse), &out_dir);
    export_schema(&schema_for!(ReportsQueryResponse), &out_dir);
}
