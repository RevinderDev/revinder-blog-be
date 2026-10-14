use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Deserialize, Validate, Debug, Serialize, Type)]
#[garde(transparent)]
#[sqlx(transparent)]
pub(crate) struct Email(#[garde(email)] String);

#[derive(Deserialize, Validate, Debug, Serialize)]
#[garde(transparent)]
pub(crate) struct Password(#[garde(length(min = 15))] String);

#[derive(Deserialize, Serialize, Debug, Type)]
#[sqlx(transparent)]
pub(crate) struct PasswordHash(String);

impl Password {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Email(value)
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Password(value)
    }
}
impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        PasswordHash(value)
    }
}
