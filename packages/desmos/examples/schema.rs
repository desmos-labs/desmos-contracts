use std::{env::current_dir, fs::create_dir_all};

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use desmos::{
    query_types::{PostsResponse, ReportsResponse},
    types::{Attachment, OptionalData, PollAnswer, PollData, Post, Report},
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("packages/desmos/schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(OptionalData), &out_dir);
    export_schema(&schema_for!(Attachment), &out_dir);
    export_schema(&schema_for!(PollData), &out_dir);
    export_schema(&schema_for!(PollAnswer), &out_dir);
    export_schema(&schema_for!(Post), &out_dir);
    export_schema(&schema_for!(Report), &out_dir);
    export_schema(&schema_for!(PostsResponse), &out_dir);
    export_schema(&schema_for!(ReportsResponse), &out_dir);
}
