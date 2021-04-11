use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct DocumentHistory {
    pub id: i64,
    pub content: String,
    pub char_count: i64,
    pub increase: i64,
    pub reg_utc: i64,
    pub writer_id: i64,
    pub writer_name: String,
}
