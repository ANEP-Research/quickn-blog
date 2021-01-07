table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        pass -> Varchar,
        email -> Varchar,
    }
}

table! {
    comments (id) {
        id -> Int4,
        author -> Int4,
        post -> Int4,
        title -> Varchar,
        body -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        author -> Int4,
        title -> Varchar,
        body -> Varchar,
        created -> Timestamp,
        modified -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    comments,
    posts,
);
