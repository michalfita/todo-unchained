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
        action_id -> Text,
        label_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    labels (id) {
        id -> Text,
        parent_id -> Nullable<Text>,
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
