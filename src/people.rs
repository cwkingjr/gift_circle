use std::ops::{Deref, DerefMut};

use counter::Counter;

use crate::error::{GiftCircleError, Result};
use crate::group::Group;

use super::person::Person;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct People(Vec<Person>);

/// Participants validated to have a group assignment on every member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupedPeople(People);

impl GroupedPeople {
    pub fn try_new(people: People) -> Result<Self> {
        if people.has_empty_group() {
            return Err(GiftCircleError::MissingGroup);
        }
        Ok(Self(people))
    }

    pub(crate) fn person_group(person: &Person) -> u16 {
        person
            .group_number
            .expect("GroupedPeople invariant: every participant has a group")
    }

    pub fn largest_group(&self) -> Group {
        let largest = self
            .0
            .iter()
            .map(Self::person_group)
            .collect::<Counter<_>>()
            .most_common_ordered()[0];
        Group::new(largest.0, largest.1 as u16)
    }

    pub fn largest_non_prev_group(&self, previous_group: u16, remaining: &[usize]) -> Group {
        let largest = remaining
            .iter()
            .map(|&i| Self::person_group(&self.0[i]))
            .filter(|&g| g != previous_group)
            .collect::<Counter<_>>()
            .most_common_ordered()[0];
        Group::new(largest.0, largest.1 as u16)
    }

    pub fn has_possible_hamiltonian_path(&self) -> bool {
        (self.largest_group().size as usize * 2) <= self.len()
    }

    pub fn first_and_last_groups_are_different(&self, path: &[usize]) -> bool {
        let first_group = Self::person_group(&self.0[path[0]]);
        let last_group = Self::person_group(&self.0[*path.last().expect("non-empty path")]);
        first_group != last_group
    }

    pub fn has_no_consecutive_group_numbers(&self, path: &[usize]) -> bool {
        let mut previous_group: u16 = 0;
        for &index in path {
            let group = Self::person_group(&self.0[index]);
            if group == previous_group {
                return false;
            }
            previous_group = group;
        }
        true
    }

    pub fn is_valid_gift_circle(&self, path: &[usize]) -> bool {
        self.first_and_last_groups_are_different(path)
            && self.has_no_consecutive_group_numbers(path)
    }
}

impl Deref for GroupedPeople {
    type Target = People;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GroupedPeople {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl People {
    pub fn assign_gift_recipients(&mut self) {
        if self.0.is_empty() {
            return;
        }
        let first = self.0[0].name.clone();
        for i in 0..self.0.len() - 1 {
            self.0[i].assigned_person_name = Some(self.0[i + 1].name.clone());
        }
        self.0.last_mut().unwrap().assigned_person_name = Some(first);
    }

    pub fn has_empty_group(&self) -> bool {
        self.0.iter().any(|p| p.group_number.is_none())
    }

    pub fn get_duplicated_names(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|p| p.name.clone())
            .collect::<Counter<_>>()
            .iter()
            .filter(|(_, the_name_count)| **the_name_count > 1)
            .map(|(the_name, _)| the_name.clone())
            .collect()
    }
}

impl From<Vec<Person>> for People {
    fn from(value: Vec<Person>) -> Self {
        Self(value)
    }
}

impl FromIterator<Person> for People {
    fn from_iter<I: IntoIterator<Item = Person>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for People {
    type Item = Person;
    type IntoIter = std::vec::IntoIter<Person>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for People {
    type Target = Vec<Person>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for People {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn grouped(people: People) -> GroupedPeople {
        GroupedPeople::try_new(people).expect("valid grouped fixture")
    }

    #[test]
    fn test_assign_gift_recipients() {
        let mut people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]);
        people.assign_gift_recipients();
        let last_persons_assigned_name = people
            .last()
            .unwrap()
            .to_owned()
            .assigned_person_name
            .unwrap();

        let first_persons_name = people[0].name.clone();
        assert!(last_persons_assigned_name == first_persons_name);
    }

    #[test]
    fn test_largest_group() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
        ]));
        assert_eq!(people.largest_group(), Group::new(1, 2u16));
    }

    #[test]
    fn test_largest_non_prev_group() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ]));
        let remaining: Vec<usize> = (0..people.len()).collect();
        assert_eq!(
            people.largest_non_prev_group(2, &remaining),
            Group::new(1, 2u16)
        );
    }

    #[test]
    fn test_has_possible_hamiltonian_path_true() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ]));
        assert!(people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_possible_hamiltonian_path_false() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new("Daughter", 2),
        ]));
        assert!(!people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_empty_group_false() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new("Daughter", 2),
        ]);
        assert!(!people.has_empty_group());
    }

    #[test]
    fn test_has_empty_group_true() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new_no_group("Daughter"),
        ]);
        assert!(people.has_empty_group());
    }

    #[test]
    fn test_get_duplicate_names_one() {
        let people = People(vec![Person::new("Mother", 1), Person::new("Mother", 1)]);
        assert_eq!(people.get_duplicated_names().len(), 1);
    }

    #[test]
    fn test_get_duplicate_names_none() {
        let people = People(vec![Person::new("Mother", 1), Person::new("Father", 1)]);
        assert_eq!(people.get_duplicated_names().len(), 0);
    }

    #[test]
    fn test_first_and_last_groups_are_different_true() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]));
        assert!(people.first_and_last_groups_are_different(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_first_and_last_groups_are_different_false() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
        ]));
        assert!(!people.first_and_last_groups_are_different(&[0, 1, 2]));
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_true() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]));
        assert!(people.has_no_consecutive_group_numbers(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_false() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ]));
        assert!(!people.has_no_consecutive_group_numbers(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_is_valid_gift_circle() {
        let people = grouped(People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]));
        assert!(people.is_valid_gift_circle(&[0, 1, 2, 3]));
    }
}
