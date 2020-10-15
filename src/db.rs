use diesel::{Connection, SqliteConnection};
#[cfg(test)] use diesel_migrations;
#[cfg(not(test))] use dotenv::dotenv;
#[cfg(not(test))] use std::env;

pub mod models;
pub mod schema;

embed_migrations!();

#[cfg(test)]
pub fn establish_connection() -> SqliteConnection {
    let conn = SqliteConnection::establish(":memory:")
        .unwrap_or_else(|_| panic!("Error creating test database"));

    let _ = diesel_migrations::run_pending_migrations(&conn);

    conn
}

#[cfg(not(test))]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}