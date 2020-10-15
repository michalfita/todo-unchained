table! {
    actions (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        done_at -> Nullable<Timestamp>,
    }
}

table! {
    actions_labels (id) {
        id -> Text,
        action -> Text,
        label -> Text,
        created_at -> Timestamp,
    }
}

table! {
    labels (id) {
        id -> Text,
        parent -> Nullable<Text>,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(actions_labels -> actions (action));
joinable!(actions_labels -> labels (label));

allow_tables_to_appear_in_same_query!(
    actions,
    actions_labels,
    labels,
);
