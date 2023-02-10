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

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct PersonWithoutGroup {
    pub name: String,
    pub email_address: Option<String>,
    pub assigned_person_name: Option<String>,
}

impl PersonWithoutGroup {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        PersonWithoutGroup {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl From<Person> for PersonWithoutGroup {
    fn from(person: Person) -> Self {
        PersonWithoutGroup {
            name: person.name,
            email_address: person.email_address,
            assigned_person_name: person.assigned_person_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_without_group_from_person() {
        let person = Person::new("Father", 1);
        let person_without_group = PersonWithoutGroup::from(person);
        let expected = PersonWithoutGroup::new("Father");
        assert_eq!(person_without_group, expected);
    }
}
