use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::words;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = words)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub meaning: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = words)]
pub struct NewWord {
    pub word: String,
}
