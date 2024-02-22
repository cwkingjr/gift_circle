use counter::Counter;

use super::group::Group;
use super::person::Person;

#[allow(dead_code)]
pub type People = Vec<Person>;

pub trait PeopleCycle {
    fn assign_gift_recipients(&mut self);
    fn get_duplicated_names(&self) -> Vec<String>;
    fn first_and_last_groups_are_different(&self) -> bool;
    fn has_empty_group(&self) -> bool;
    fn has_no_consecutive_group_numbers(&self) -> bool;
    fn has_possible_hamiltonian_path(&self) -> bool;
    fn is_valid_gift_circle(&self) -> bool;
    fn largest_group(&self) -> Group;
    fn largest_non_prev_group(&self, previous_group: u16) -> Group;
}

impl PeopleCycle for People {
    /// Iterates through the contained Persons and sets their `assigned_person_name`
    /// based upon their position in the vec. For example, the person at position 0 is assigned the name of the person
    /// at position 1, person 1 is asssigned person 2, ..., person[last] is assigned the person at position 0.
    fn assign_gift_recipients(&mut self) {
        let last_person_name = self.last().unwrap().name.clone();

        for (i, person) in self.clone().iter().enumerate() {
            if person.name == last_person_name {
                self[i].assigned_person_name = Some(self[0].name.clone());
            } else {
                self[i].assigned_person_name = Some(self[i + 1].name.clone());
            }
        }
    }

    ///Obtains the group assigned to the most people.
    fn largest_group(&self) -> Group {
        let largest = self
            .iter()
            .map(|p| p.group_number.unwrap())
            .collect::<Counter<_>>() // gets the count per group number
            .most_common_ordered()[0]; // grab the largest group by count
        let g_number = largest.0;
        let g_size = largest.1 as u16;
        Group::new(g_number, g_size)
    }

    ///Obtains the group assigned to the most people, excluding the group used last (previous group).
    fn largest_non_prev_group(&self, previous_group: u16) -> Group {
        let largest = self
            .iter()
            .filter(|p| p.group_number.unwrap() != previous_group)
            .map(|p| p.group_number)
            .collect::<Counter<_>>()
            .most_common_ordered()[0];
        let g_number = largest.0.unwrap();
        let g_size = largest.1 as u16;
        Group::new(g_number, g_size)
    }

    ///Checks that we pass hamiltonian path criteria.
    fn has_possible_hamiltonian_path(&self) -> bool {
        (self.largest_group().size as usize * 2) <= self.len()
    }

    ///Determines if any person has an empty group assignment.
    fn has_empty_group(&self) -> bool {
        self.iter().any(|p| p.group_number.is_none())
    }

    fn get_duplicated_names(&self) -> Vec<String> {
        let duplicated_names = self
            .iter()
            .map(|p| p.name.clone())
            .collect::<Counter<_>>()
            .iter()
            .filter(|(_, the_name_count)| **the_name_count > 1)
            .map(|(the_name, _)| the_name.clone())
            .collect();
        duplicated_names
    }

    fn first_and_last_groups_are_different(&self) -> bool {
        // Last person gives gift to first person so can't be in the same group.

        let first_group = self.first().unwrap().group_number.unwrap();
        let last_group = self.last().unwrap().group_number.unwrap();

        first_group != last_group
    }

    fn has_no_consecutive_group_numbers(&self) -> bool {
        let mut previous_group: u16 = 0;
        for person in self.iter() {
            if person.group_number.unwrap() == previous_group {
                return false;
            }
            previous_group = person.group_number.unwrap();
        }
        true
    }

    fn is_valid_gift_circle(&self) -> bool {
        self.first_and_last_groups_are_different() && self.has_no_consecutive_group_numbers()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_assign_gift_recipients() {
        let mut people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
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
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
        ];
        assert_eq!(people.largest_group(), Group::new(1, 2u16));
    }

    #[test]
    fn test_largest_non_prev_group() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ];
        assert_eq!(people.largest_non_prev_group(2), Group::new(1, 2u16));
    }

    #[test]
    fn test_has_possible_hamiltonian_path_true() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ];
        assert!(people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_possible_hamiltonian_path_false() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new("Daughter", 2),
        ];
        assert!(!people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_empty_group_false() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new("Daughter", 2),
        ];
        assert!(!people.has_empty_group());
    }

    #[test]
    fn test_has_empty_group_true() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new_no_group("Daughter"),
        ];
        assert!(people.has_empty_group());
    }

    #[test]
    fn test_get_duplicate_names_one() {
        let people = vec![Person::new("Mother", 1), Person::new("Mother", 1)];
        assert_eq!(people.get_duplicated_names().len(), 1);
    }

    #[test]
    fn test_get_duplicate_names_none() {
        let people = vec![Person::new("Mother", 1), Person::new("Father", 1)];
        assert_eq!(people.get_duplicated_names().len(), 0);
    }

    #[test]
    fn test_first_and_last_groups_are_different_true() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
        assert!(people.first_and_last_groups_are_different());
    }

    #[test]
    fn test_first_and_last_groups_are_different_false() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
        ];
        assert!(!people.first_and_last_groups_are_different());
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_true() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
        assert!(people.has_no_consecutive_group_numbers());
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_false() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ];
        assert!(!people.has_no_consecutive_group_numbers());
    }

    #[test]
    fn test_is_valid_gift_circle() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
        assert!(people.is_valid_gift_circle());
    }
}
