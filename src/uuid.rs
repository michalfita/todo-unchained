use uuid;
use diesel::deserialize::{self, FromSql};
use diesel::sqlite::Sqlite;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::backend::Backend;
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::ops::Deref;

const UUID_BYTE_SIZE: usize = 16;

#[derive(Debug, AsExpression, FromSqlRow, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[sql_type = "Binary"]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<Self, uuid::Error> {
        uuid::Uuid::parse_str(input).map(|r| r.into())
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(id: uuid::Uuid) -> Self {
        Uuid(id)
    }
}

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromSql<Binary, Sqlite> for Uuid {
    fn from_sql(input: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = <*const [u8] as FromSql<Binary, Sqlite>>::from_sql(input)?;
        let bytes = unsafe {&*bytes};
        assert_eq!(bytes.len(), UUID_BYTE_SIZE);
        let uuid = uuid::Uuid::from_slice(bytes)?;
        Ok(uuid.into())
    }
}

impl ToSql<Binary, Sqlite> for Uuid {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let bytes = self.0.as_bytes();
        <[u8] as ToSql<Binary, Sqlite>>::to_sql(bytes, out)
    }
}