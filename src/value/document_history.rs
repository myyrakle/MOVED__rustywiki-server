use serde::{Deserialize, Serialize};

// tb_document_history join tb_user
#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct DocumentHistory {
    pub id: i64,
    pub content: String,
    pub char_count: i64,
    pub increase: i64,
    pub reg_utc: i64,
    pub revision_number: i64,
    pub rollback_revision_number: Option<i64>,
    pub writer_id: i64,
    pub writer_name: String,
}
