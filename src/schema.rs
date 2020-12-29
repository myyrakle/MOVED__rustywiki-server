table! {
    tb_user (id) {
        id -> Int8,
        email -> Varchar,
        password -> Text,
        user_type -> Varchar,
        nickname -> Varchar,
        use_yn -> Bool,
        reg_time -> Timestamp,
    }
}