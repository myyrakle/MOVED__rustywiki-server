table! {
    Document (id) {
        id -> Int8,
        title -> Text,
        reg_utc -> Int8,
    }
}

table! {
    History (id, writer_id, document_id) {
        id -> Int8,
        writer_id -> Int8,
        document_id -> Int8,
        filepath -> Text,
        increase -> Int8,
        reg_date -> Int8,
    }
}

table! {
    tb_image (id, uploader_id) {
        id -> Int4,
        uploader_id -> Int8,
        domain -> Nullable<Text>,
        path -> Text,
        use_yn -> Bool,
        reg_utc -> Int8,
    }
}

table! {
    tb_refresh_token (token_value) {
        token_value -> Text,
        reg_utc -> Int8,
        user_id -> Int8,
        dead_yn -> Bool,
        dead_utc -> Nullable<Int8>,
    }
}

table! {
    tb_user (id) {
        id -> Int8,
        email -> Varchar,
        salt -> Varchar,
        password -> Text,
        user_type -> Varchar,
        nickname -> Varchar,
        use_yn -> Bool,
        reg_utc -> Int8,
    }
}

table! {
    test (id) {
        id -> Int8,
        text -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    Document,
    History,
    tb_image,
    tb_refresh_token,
    tb_user,
    test,
);
