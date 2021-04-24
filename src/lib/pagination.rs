use serde::{Deserialize, Serialize};

// tb_document_history join tb_user
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaginationToken {
    pub page: i64,
    pub limit: i64,
}

pub fn page_to_offset(page: i64, limit: i64) -> i64 {
    (page - 1) * limit
}

pub fn offset_to_page(offset: i64, limit: i64) -> i64 {
    (offset / limit) + 1
}

pub fn init_pagination(
    limit: Option<i64>,
    page: Option<i64>,
    next_token: Option<String>,
) -> (i64, i64) {
    let (token_page, token_limit) = from_page_token(next_token).unwrap_or((1, 10));

    let limit = limit.unwrap_or(token_limit);
    let page = page.unwrap_or(token_page);
    let offset = page_to_offset(page, limit);

    (offset, limit)
}

// 페이지 토큰에서 페이지값 추출
pub fn from_page_token(token: Option<String>) -> Option<(i64, i64)> {
    let json_string: String = String::from_utf8(base64::decode(token?).ok()?).ok()?;
    let result: PaginationToken = serde_json::from_str(&json_string).ok()?;
    Some((result.page, result.limit))
}

pub fn to_page_token(offset: i64, limit: i64, total_count: i64) -> (bool, String) {
    let page = offset_to_page(offset, limit);
    let has_next = (offset + limit) <= total_count;

    let token = if has_next {
        let page_token = PaginationToken {
            page: page + 1,
            limit: limit,
        };
        let json_string: String = serde_json::to_string(&page_token).unwrap_or("".into());
        base64::encode(json_string.as_bytes().to_vec())
    } else {
        "".into()
    };

    (has_next, token)
}
