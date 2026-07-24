use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Validate, Debug, Serialize)]
#[garde(transparent)]
pub(crate) struct Email(#[garde(email)] String);

#[derive(Deserialize, Validate, Debug, Serialize)]
#[garde(transparent)]
pub(crate) struct Password(#[garde(length(min = 15))] String);

impl sqlx::Type<sqlx::Sqlite> for Email {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for Email {
    fn decode(
        value: <sqlx::Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
        Ok(Email(s))
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Email(value)
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Email {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&self.0, buf)
    }
}

impl sqlx::Type<sqlx::Sqlite> for Password {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for Password {
    fn decode(
        value: <sqlx::Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
        Ok(Password(s))
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Password {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&self.0, buf)
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Password(value)
    }
}
