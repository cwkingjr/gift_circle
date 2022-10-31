use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub email_address: String,
    pub group_number: u16,
    pub assigned_person_name: Option<String>,
}
