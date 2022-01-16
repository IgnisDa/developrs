table! {
    apps (id) {
        id -> Int4,
        slug -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    deploys (id) {
        id -> Varchar,
        sha -> Varchar,
        executed_at -> Timestamp,
        app_id -> Int4,
    }
}

joinable!(deploys -> apps (app_id));

allow_tables_to_appear_in_same_query!(
    apps,
    deploys,
);
