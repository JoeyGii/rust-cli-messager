use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "message"]
pub struct Message {
    id: String,
    name: String,
    body: String,
    published: Bool,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}
