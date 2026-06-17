use serde::{Deserialize, Serialize};

/// A participant read from the input CSV before gift assignments are generated.
#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize)]
pub struct Participant {
    /// Unique participant name.
    pub name: String,
    /// Optional email address passed through to output.
    pub email_address: Option<String>,
    /// Family group number when `--use-groups` is enabled.
    pub group_number: Option<u16>,
}

/// A participant row written to the output CSV with an assigned recipient.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Person {
    /// Unique participant name.
    pub name: String,
    /// Optional email address from the input CSV.
    pub email_address: Option<String>,
    /// Family group number from the input CSV, if any.
    pub group_number: Option<u16>,
    /// Name of the person this participant gives a gift to.
    pub assigned_person_name: String,
}

impl Person {
    pub(crate) fn from_assignment(participant: &Participant, assigned_person_name: String) -> Self {
        Self {
            name: participant.name.clone(),
            email_address: participant.email_address.clone(),
            group_number: participant.group_number,
            assigned_person_name,
        }
    }
}

#[cfg(test)]
impl Participant {
    pub fn new(name: &str, group_number: u16) -> Self {
        Self {
            name: name.to_string(),
            group_number: Some(group_number),
            ..Default::default()
        }
    }

    pub fn new_no_group(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}
