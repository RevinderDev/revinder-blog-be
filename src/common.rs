use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate, Debug)]
#[garde(transparent)]
pub struct Email(#[garde(email)] String);

#[derive(Deserialize, Validate, Debug)]
#[garde(transparent)]
pub struct Password(#[garde(length(min = 15))] String);
