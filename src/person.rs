use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub email_address: Option<String>,
    pub group_number: Option<u16>,
    pub assigned_person_name: Option<String>,
}

impl Person {
    #[allow(dead_code)]
    pub fn new(name: &str, group_number: u16) -> Self {
        Person {
            name: name.to_string(),
            group_number: Some(group_number),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn new_no_group(name: &str) -> Self {
        Person {
            name: name.to_string(),
            ..Default::default()
        }
    }
}
