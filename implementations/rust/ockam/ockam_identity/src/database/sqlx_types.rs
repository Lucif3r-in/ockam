use std::net::SocketAddr;

use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::{Database, Encode, Sqlite, Type};
use time::OffsetDateTime;

/// This enum represents the set of types that we currently support in our database
/// Since we support only Sqlite at the moment, those types are close to what is supported by Sqlite:
/// https://www.sqlite.org/datatype3.html
///
/// The purpose of this type is to ease the serialization of data types in Ockam into data types in
/// our database. For example, if we describe how to translate an `Identifier` into some `Text` then
/// we can use the `Text` as a parameter in a sqlx query.
///
/// Note: see the `ToSqlxType` trait and its instances for how the conversion is done
///
pub enum SqlxType {
    /// This type represents text in the database
    Text(String),
    /// This type represents arbitrary bytes in the database
    Blob(Vec<u8>),
    /// This type represents ints, signed or unsigned
    Integer(i64),
    /// This type represents floats
    #[allow(unused)]
    Real(f64),
}

/// The SqlType implements the Type<Sqlite> trait from sqlx to allow its values to be serialized
/// to an Sqlite database
impl Type<Sqlite> for SqlxType {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <Vec<u8> as Type<Sqlite>>::type_info()
    }
}

/// The SqlType implements the Encode<Sqlite> trait from sqlx to allow its values to be serialized
/// to an Sqlite database. There is a 1 to 1 mapping with the database native types
impl Encode<'_, Sqlite> for SqlxType {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments>::ArgumentBuffer) -> IsNull {
        match self {
            SqlxType::Text(v) => <String as Encode<'_, Sqlite>>::encode_by_ref(v, buf),
            SqlxType::Blob(v) => <Vec<u8> as Encode<'_, Sqlite>>::encode_by_ref(v, buf),
            SqlxType::Integer(v) => <i64 as Encode<'_, Sqlite>>::encode_by_ref(v, buf),
            SqlxType::Real(v) => <f64 as Encode<'_, Sqlite>>::encode_by_ref(v, buf),
        }
    }

    fn produces(&self) -> Option<<Sqlite as Database>::TypeInfo> {
        Some(match self {
            SqlxType::Text(_) => <String as Type<Sqlite>>::type_info(),
            SqlxType::Blob(_) => <Vec<u8> as Type<Sqlite>>::type_info(),
            SqlxType::Integer(_) => <i64 as Type<Sqlite>>::type_info(),
            SqlxType::Real(_) => <f64 as Type<Sqlite>>::type_info(),
        })
    }
}

/// This trait can be implemented by any type that can be converted to a database type
/// Typically an `Identifier` (to a `Text`), a `TimestampInSecond` (to an `Integer`) etc...
///
/// This allows a value to be used as a bind parameters in a sqlx query for example:
/// ```
/// use std::str::FromStr;
/// use sqlx::query_as;
/// use ockam_identity::Identifier;
///
/// let identifier = Identifier::from_str("Ie86be15e83d1c93e24dd1967010b01b6df491b45").unwrap();
///
/// let query = query_as("SELECT * FROM identity WHERE identifier=$1").bind(identifier.as_sql());
///```
///
pub trait ToSqlxType {
    // Return the appropriate sql type
    fn to_sql(&self) -> SqlxType;
}

impl ToSqlxType for String {
    fn to_sql(&self) -> SqlxType {
        SqlxType::Text(self.clone())
    }
}

impl ToSqlxType for &str {
    fn to_sql(&self) -> SqlxType {
        self.to_string().to_sql()
    }
}

impl ToSqlxType for bool {
    fn to_sql(&self) -> SqlxType {
        if *self {
            1.to_sql()
        } else {
            0.to_sql()
        }
    }
}

impl ToSqlxType for u64 {
    fn to_sql(&self) -> SqlxType {
        SqlxType::Integer(*self as i64)
    }
}

impl ToSqlxType for OffsetDateTime {
    fn to_sql(&self) -> SqlxType {
        SqlxType::Integer(self.unix_timestamp())
    }
}

impl ToSqlxType for Vec<u8> {
    fn to_sql(&self) -> SqlxType {
        SqlxType::Blob(self.clone())
    }
}

impl ToSqlxType for SocketAddr {
    fn to_sql(&self) -> SqlxType {
        SqlxType::Text(self.to_string())
    }
}
