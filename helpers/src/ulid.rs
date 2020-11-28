use std::fmt;
use std::io::Write;
use std::{convert::TryFrom, str::FromStr};

use diesel::{deserialize, pg::Pg, serialize, sql_types::Uuid};
use rusty_ulid::{DecodingError, Ulid as RustyUlid};
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(
    AsExpression, Clone, Copy, Debug, Deserialize, Eq, FromSqlRow, Hash, PartialEq, QueryId,
    Serialize, SqlType,
)]
// find `oid` with `SELECT * FROM pg_catalog.pg_type WHERE typname LIKE '%uuid%';`
#[postgres(oid = "2950", array_oid = "2951")]
#[sql_type = "Uuid"]
pub struct Ulid(RustyUlid);

impl Ulid {
    pub fn new() -> Self {
        Self(RustyUlid::generate())
    }
}

impl Default for Ulid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Ulid {
    type Err = DecodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ulid(RustyUlid::from_str(s)?))
    }
}

impl deserialize::FromSql<Uuid, Pg> for Ulid {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        match RustyUlid::try_from(bytes) {
            Ok(ulid) => Ok(Ulid(ulid)),
            Err(e) => Err(format!("{}", e).into()),
        }
    }
}

impl serialize::ToSql<Uuid, Pg> for Ulid {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        let byte_array: [u8; 16] = self.0.into();
        out.write_all(&byte_array[..])
            .map(|_| serialize::IsNull::No)
            .map_err(Into::into)
    }
}
