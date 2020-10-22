use crate::uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable};
use chrono::NaiveDateTime;

use super::schema::{
    actions,
    actions::dsl::actions as actions_dsl,
};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "actions"]
pub struct Action {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub done_at: Option<NaiveDateTime>
}

impl Action {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        actions_dsl.load::<Action>(conn).expect("Error loading Actions")
    }

    pub fn by_id(id: &Uuid, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = actions_dsl.find(id).get_result::<Action>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(title: &str, description: Option<&str>, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new();

        let action = Self::new_internal(&new_id, title, description);

        diesel::insert_into(actions_dsl)
            .values(&action)
            .execute(conn)
            .expect("Error storing new Action");

        Self::by_id(&new_id, conn)
    }

    fn new_internal(id: &Uuid, title: &str, description: Option<&str>) -> Self {
        Self {
            id: *id,
            title: title.into(),
            description: description.map(Into::into),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            done_at: None
        }
    }
}

#[cfg(test)]
mod action_test;

use super::schema::{
    labels,
    labels::dsl::labels as labels_dsl,
};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[table_name = "labels"]
#[belongs_to(Label, foreign_key = "parent_id")]
pub struct Label {
    pub id: Uuid,
    parent_id: Option<Uuid>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Label {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        labels_dsl.load::<Label>(conn).expect("Error loading Actions")
    }

    pub fn by_id(id: &Uuid, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = labels_dsl.find(id).get_result::<Label>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(name: &str, parent: Option<&Label>, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new();

        let label = Self::new_internal(&new_id, name, parent);

        diesel::insert_into(labels_dsl)
            .values(&label)
            .execute(conn)
            .expect("Error storing new Label");

        Self::by_id(&new_id, conn)
    }

    fn new_internal(id: &Uuid, name: &str, parent: Option<&Label>) -> Self {
        Self {
            id: *id,
            name: name.into(),
            parent_id: parent.map(|pl| pl.id.clone()),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[cfg(test)]
mod label_test;

use super::schema::{
    actions_labels,
    actions_labels::dsl::actions_labels as actions_labels_dsl,
};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[table_name = "actions_labels"]
#[belongs_to(Action, foreign_key = "action_id")]
#[belongs_to(Label, foreign_key = "label_id")]
pub struct ActionLabel {
    pub id: Uuid,
    pub action_id: Uuid,
    pub label_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl ActionLabel {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        actions_labels_dsl.load::<ActionLabel>(conn).expect("Error loading Actions")
    }

    pub fn by_id(id: &Uuid, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = actions_labels_dsl.find(id).get_result::<ActionLabel>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(action_id: Uuid, label_id: Uuid, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new();

        let action_label = Self::new_internal(new_id.clone(), action_id, label_id);

        diesel::insert_into(actions_labels_dsl)
            .values(&action_label)
            .execute(conn)
            .expect("Error storing new Label");

        Self::by_id(&new_id, conn)
    }
    
    fn new_internal(id: Uuid, action_id: Uuid, label_id: Uuid) -> Self {
        Self {
            id: id,
            action_id: action_id,
            label_id: label_id,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[cfg(test)]
mod actions_labels_test;