table! {
    actions (id) {
        id -> Binary,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        done_at -> Nullable<Timestamp>,
    }
}

table! {
    actions_labels (id) {
        id -> Binary,
        action_id -> Binary,
        label_id -> Binary,
        created_at -> Timestamp,
    }
}

table! {
    labels (id) {
        id -> Binary,
        parent_id -> Nullable<Binary>,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(actions_labels -> actions (action_id));
joinable!(actions_labels -> labels (label_id));

allow_tables_to_appear_in_same_query!(
    actions,
    actions_labels,
    labels,
);
