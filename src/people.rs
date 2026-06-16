use std::ops::{Deref, DerefMut};

use counter::Counter;

use super::group::Group;
use super::person::Person;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct People(Vec<Person>);

impl People {
    /// Iterates through the contained Persons and sets their `assigned_person_name`
    /// based upon their position in the vec. For example, the person at position 0 is assigned the name of the person
    /// at position 1, person 1 is asssigned person 2, ..., person[last] is assigned the person at position 0.
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

    /// Obtains the group assigned to the most people.
    pub fn largest_group(&self) -> Group {
        let largest = self
            .0
            .iter()
            .map(|p| p.group_number.unwrap())
            .collect::<Counter<_>>()
            .most_common_ordered()[0];
        let g_number = largest.0;
        let g_size = largest.1 as u16;
        Group::new(g_number, g_size)
    }

    /// Obtains the group assigned to the most people, excluding the group used last (previous group).
    pub fn largest_non_prev_group(&self, previous_group: u16) -> Group {
        let largest = self
            .0
            .iter()
            .filter(|p| p.group_number.unwrap() != previous_group)
            .map(|p| p.group_number)
            .collect::<Counter<_>>()
            .most_common_ordered()[0];
        let g_number = largest.0.unwrap();
        let g_size = largest.1 as u16;
        Group::new(g_number, g_size)
    }

    /// Checks that we pass hamiltonian path criteria.
    pub fn has_possible_hamiltonian_path(&self) -> bool {
        (self.largest_group().size as usize * 2) <= self.len()
    }

    /// Determines if any person has an empty group assignment.
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

    pub fn first_and_last_groups_are_different(&self) -> bool {
        let first_group = self.0.first().unwrap().group_number.unwrap();
        let last_group = self.0.last().unwrap().group_number.unwrap();

        first_group != last_group
    }

    pub fn has_no_consecutive_group_numbers(&self) -> bool {
        let mut previous_group: u16 = 0;
        for person in &self.0 {
            if person.group_number.unwrap() == previous_group {
                return false;
            }
            previous_group = person.group_number.unwrap();
        }
        true
    }

    pub fn is_valid_gift_circle(&self) -> bool {
        self.first_and_last_groups_are_different() && self.has_no_consecutive_group_numbers()
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
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
        ]);
        assert_eq!(people.largest_group(), Group::new(1, 2u16));
    }

    #[test]
    fn test_largest_non_prev_group() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ]);
        assert_eq!(people.largest_non_prev_group(2), Group::new(1, 2u16));
    }

    #[test]
    fn test_has_possible_hamiltonian_path_true() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ]);
        assert!(people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_possible_hamiltonian_path_false() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new("Daughter", 2),
        ]);
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
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]);
        assert!(people.first_and_last_groups_are_different());
    }

    #[test]
    fn test_first_and_last_groups_are_different_false() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
        ]);
        assert!(!people.first_and_last_groups_are_different());
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_true() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]);
        assert!(people.has_no_consecutive_group_numbers());
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_false() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ]);
        assert!(!people.has_no_consecutive_group_numbers());
    }

    #[test]
    fn test_is_valid_gift_circle() {
        let people = People(vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ]);
        assert!(people.is_valid_gift_circle());
    }
}
