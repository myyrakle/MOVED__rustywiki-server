table! {
    tb_user (id) {
        id -> Int8,
        email -> Varchar,
        password -> Text,
        salt -> Varchar,
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
    tb_user,
    test,
);
