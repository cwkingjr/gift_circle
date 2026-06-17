use std::collections::HashMap;
use std::ops::Deref;

use crate::error::{GiftCircleError, Result};
use crate::group::Group;
use crate::person::{Participant, Person};

/// A collection of participants loaded from the input CSV.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct People(Vec<Participant>);

/// Participants validated to have a group assignment on every member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupedPeople(People);

impl GroupedPeople {
    pub(crate) fn group_at(people: &People, index: usize) -> u16 {
        match people[index].group_number {
            Some(group) => group,
            None => unreachable!("GroupedPeople validates every participant has a group"),
        }
    }

    fn count_groups(groups: impl Iterator<Item = u16>) -> HashMap<u16, u16> {
        let mut counts = HashMap::new();
        for group in groups {
            *counts.entry(group).or_insert(0) += 1;
        }
        counts
    }

    fn largest_group_from_counts(counts: &HashMap<u16, u16>) -> Option<Group> {
        counts
            .iter()
            .max_by(|(left_num, left_size), (right_num, right_size)| {
                left_size
                    .cmp(right_size)
                    .then_with(|| right_num.cmp(left_num))
            })
            .map(|(&number, &size)| Group::new(number, size))
    }

    pub fn largest_group(&self) -> Group {
        let counts = Self::count_groups(self.0.iter().filter_map(|p| p.group_number));
        Self::largest_group_from_counts(&counts).unwrap_or(Group::new(0, 0))
    }

    pub fn largest_non_prev_group(&self, previous_group: u16, remaining: &[usize]) -> Group {
        let counts = Self::count_groups(
            remaining
                .iter()
                .filter_map(|&index| self.0[index].group_number)
                .filter(|group| *group != previous_group),
        );
        Self::largest_group_from_counts(&counts).unwrap_or(Group::new(0, 0))
    }

    pub fn has_possible_hamiltonian_path(&self) -> bool {
        (self.largest_group().size as usize * 2) <= self.len()
    }

    pub fn first_and_last_groups_are_different(&self, path: &[usize]) -> bool {
        if path.is_empty() {
            return false;
        }
        let first_group = Self::group_at(&self.0, path[0]);
        let last_group = Self::group_at(&self.0, path[path.len() - 1]);
        first_group != last_group
    }

    pub fn has_no_consecutive_group_numbers(&self, path: &[usize]) -> bool {
        let mut previous_group: Option<u16> = None;
        for &index in path {
            let group = Self::group_at(&self.0, index);
            if previous_group == Some(group) {
                return false;
            }
            previous_group = Some(group);
        }
        true
    }

    pub fn is_valid_gift_circle(&self, path: &[usize]) -> bool {
        self.first_and_last_groups_are_different(path)
            && self.has_no_consecutive_group_numbers(path)
    }
}

impl TryFrom<&People> for GroupedPeople {
    type Error = GiftCircleError;

    fn try_from(people: &People) -> Result<Self> {
        if people.has_empty_group() {
            return Err(GiftCircleError::MissingGroup);
        }
        let grouped = Self(people.clone());
        if !grouped.has_possible_hamiltonian_path() {
            return Err(GiftCircleError::ImpossibleGroupLayout);
        }
        Ok(grouped)
    }
}

impl Deref for GroupedPeople {
    type Target = People;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl People {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Participant> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Participant> {
        self.0.get(index)
    }

    pub fn assign_from_path(&self, path: &[usize]) -> Vec<Person> {
        if path.is_empty() {
            return Vec::new();
        }

        path.iter()
            .enumerate()
            .map(|(position, &giver_index)| {
                let recipient_index = path[(position + 1) % path.len()];
                Person::from_assignment(&self.0[giver_index], self.0[recipient_index].name.clone())
            })
            .collect()
    }

    pub fn has_empty_group(&self) -> bool {
        self.0
            .iter()
            .any(|participant| participant.group_number.is_none())
    }

    pub fn duplicated_names(&self) -> Vec<String> {
        let mut counts = HashMap::new();
        for participant in &self.0 {
            *counts.entry(participant.name.clone()).or_insert(0) += 1;
        }

        let mut duplicates: Vec<String> = counts
            .into_iter()
            .filter_map(|(name, count)| (count > 1).then_some(name))
            .collect();
        duplicates.sort_unstable();
        duplicates
    }

    #[deprecated(note = "renamed to `duplicated_names`")]
    pub fn get_duplicated_names(&self) -> Vec<String> {
        self.duplicated_names()
    }
}

impl From<Vec<Participant>> for People {
    fn from(value: Vec<Participant>) -> Self {
        Self(value)
    }
}

impl FromIterator<Participant> for People {
    fn from_iter<I: IntoIterator<Item = Participant>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for People {
    type Item = Participant;
    type IntoIter = std::vec::IntoIter<Participant>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for People {
    type Target = [Participant];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grouped_unchecked(people: People) -> GroupedPeople {
        GroupedPeople(people)
    }

    #[test]
    fn test_assign_from_path() {
        let people = People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 2),
            Participant::new("Son", 1),
            Participant::new("Daughter", 3),
        ]);
        let assigned = people.assign_from_path(&[0, 1, 2, 3]);
        assert_eq!(assigned.last().unwrap().assigned_person_name, "Father");
    }

    #[test]
    fn test_largest_group() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
        ]));
        assert_eq!(people.largest_group(), Group::new(1, 2));
    }

    #[test]
    fn test_largest_non_prev_group() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
            Participant::new("Daughter", 2),
        ]));
        let remaining: Vec<usize> = (0..people.len()).collect();
        assert_eq!(
            people.largest_non_prev_group(2, &remaining),
            Group::new(1, 2)
        );
    }

    #[test]
    fn test_has_possible_hamiltonian_path_true() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
            Participant::new("Daughter", 3),
        ]));
        assert!(people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_possible_hamiltonian_path_false() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 1),
            Participant::new("Daughter", 2),
        ]));
        assert!(!people.has_possible_hamiltonian_path());
    }

    #[test]
    fn test_has_empty_group_false() {
        let people = People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 1),
            Participant::new("Daughter", 2),
        ]);
        assert!(!people.has_empty_group());
    }

    #[test]
    fn test_has_empty_group_true() {
        let people = People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 1),
            Participant::new_no_group("Daughter"),
        ]);
        assert!(people.has_empty_group());
    }

    #[test]
    fn test_duplicated_names_one() {
        let people = People(vec![
            Participant::new("Mother", 1),
            Participant::new("Mother", 1),
        ]);
        assert_eq!(people.duplicated_names().len(), 1);
    }

    #[test]
    fn test_duplicated_names_none() {
        let people = People(vec![
            Participant::new("Mother", 1),
            Participant::new("Father", 1),
        ]);
        assert_eq!(people.duplicated_names().len(), 0);
    }

    #[test]
    fn test_first_and_last_groups_are_different_true() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 2),
            Participant::new("Son", 1),
            Participant::new("Daughter", 3),
        ]));
        assert!(people.first_and_last_groups_are_different(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_first_and_last_groups_are_different_false() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 2),
            Participant::new("Son", 1),
        ]));
        assert!(!people.first_and_last_groups_are_different(&[0, 1, 2]));
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_true() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 2),
            Participant::new("Son", 1),
            Participant::new("Daughter", 3),
        ]));
        assert!(people.has_no_consecutive_group_numbers(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_false() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 2),
            Participant::new("Son", 2),
            Participant::new("Daughter", 3),
        ]));
        assert!(!people.has_no_consecutive_group_numbers(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_is_valid_gift_circle() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 2),
            Participant::new("Son", 1),
            Participant::new("Daughter", 3),
        ]));
        assert!(people.is_valid_gift_circle(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_adjacent_groups_in_valid_circle_differ_including_wraparound() {
        let people = grouped_unchecked(People(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
            Participant::new("Daughter", 2),
        ]));
        let path = vec![0, 2, 1, 3];
        assert!(people.is_valid_gift_circle(&path));
        for window in path.windows(2) {
            assert_ne!(
                GroupedPeople::group_at(&people, window[0]),
                GroupedPeople::group_at(&people, window[1])
            );
        }
        assert_ne!(
            GroupedPeople::group_at(&people, *path.last().unwrap()),
            GroupedPeople::group_at(&people, path[0])
        );
    }
}
