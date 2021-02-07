table! {
    examples (id) {
        id -> Int4,
        english -> Nullable<Text>,
        korean -> Nullable<Text>,
        grammar_id -> Nullable<Int4>,
    }
}

table! {
    grammars (id) {
        id -> Int4,
        name -> Text,
        meaning_en -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        account_type -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    examples,
    grammars,
    users,
);
