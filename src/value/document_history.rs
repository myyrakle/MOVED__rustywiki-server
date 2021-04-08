use crate::models::SelectDocumentHistory;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DocumentHistory {
    pub id: i64,
    pub content: String,
    pub char_count: i64,
    pub increase: i64,
    pub reg_utc: i64,
}

impl From<SelectDocumentHistory> for DocumentHistory {
    fn from(item: SelectDocumentHistory) -> DocumentHistory {
        DocumentHistory {
            id: item.id,
            content: item.content,
            char_count: item.char_count,
            increase: item.increase,
            reg_utc: item.reg_utc,
        }
    }
}
