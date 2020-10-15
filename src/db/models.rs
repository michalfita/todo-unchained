use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable};
use chrono::NaiveDateTime;

use super::schema::actions;
use super::schema::actions::dsl::actions as actions_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "actions"]
pub struct Action {
    pub id: String,
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
        if let Ok(record) = actions_dsl.find(id.to_hyphenated().to_string()).get_result::<Action>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(title: &str, description: Option<&str>, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4();

        let action = Self::new_internal(&new_id.to_hyphenated().to_string(), title, description);

        diesel::insert_into(actions_dsl)
            .values(&action)
            .execute(conn)
            .expect("Error storing new Action");

        Self::by_id(&new_id, conn)
    }

    fn new_internal(id: &str, title: &str, description: Option<&str>) -> Self {
        Self {
            id: id.into(),
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

