use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate, Debug)]
#[garde(transparent)]
pub(crate) struct Email(#[garde(email)] String);

#[derive(Deserialize, Validate, Debug)]
#[garde(transparent)]
pub(crate) struct Password(#[garde(length(min = 15))] String);
