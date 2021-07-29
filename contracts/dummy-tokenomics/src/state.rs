use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{bucket, bucket_read, ReadonlyBucket, Bucket};
use desmos::types::Reaction;

pub const POST_REACTIONS_KEY: &[u8] = b"post_reactions";

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct ReactionsAmount {
    pub post_id: String,
    pub reactions_number: u128,
}

/// Get a writable reactions amount bucket
pub fn reactions_store(storage: &mut dyn Storage) -> Bucket<ReactionsAmount> {
    bucket(storage, POST_REACTIONS_KEY)
}

/// Get a read-only reactions amount bucket
pub fn reactions_read(storage: &dyn Storage) -> ReadonlyBucket<ReactionsAmount> {
    bucket_read(storage, POST_REACTIONS_KEY)
}
