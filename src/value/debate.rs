use serde::{Deserialize, Serialize};

// tb_debate join tb_user
#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct Debate {
    pub id: i64,
    pub writer_id: i64,
    pub writer_name: String,
    pub subject: String,
    pub content: String,
    pub reg_utc: i64,
}
