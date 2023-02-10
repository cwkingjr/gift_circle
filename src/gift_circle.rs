use anyhow::{anyhow, Result};
use counter::Counter;
use rand::seq::SliceRandom;

use super::group::Group;
use super::person::Person;

fn largest_group(persons: &[Person]) -> Group {
    let group_counter = persons
        .iter()
        .map(|p| p.group_number.unwrap())
        .collect::<Counter<_>>()
        .most_common_ordered();
    Group::new(group_counter[0].0, group_counter[0].1 as u16)
}

fn largest_non_prev_group(persons: &[Person], previous_group: u16) -> Group {
    let group_counter = persons
        .iter()
        .filter(|p| p.group_number.unwrap() != previous_group)
        .map(|p| p.group_number)
        .collect::<Counter<_>>()
        .most_common_ordered();
    Group::new(group_counter[0].0.unwrap(), group_counter[0].1 as u16)
}

fn has_possible_hamiltonian_path(persons: &[Person]) -> bool {
    (largest_group(persons).size as usize * 2) <= persons.len()
}

fn get_duplicate_names(persons: &[Person]) -> Vec<String> {
    let duplicates = persons
        .iter()
        .map(|p| p.name.clone())
        .collect::<Counter<_>>()
        .iter()
        .filter(|(_, v)| **v as u32 > 1)
        .map(|(k, _)| k.clone())
        .collect();
    duplicates
}

fn first_and_last_groups_are_different(persons: &[Person]) -> bool {
    // Last person gives gift to first person so can't be in the same group.

    let first_group = persons.first().unwrap().group_number.unwrap();
    let last_group = persons.last().unwrap().group_number.unwrap();

    first_group != last_group
}

fn has_no_consecutive_group_numbers(persons: &[Person]) -> bool {
    let mut previous_group: u16 = 0;
    for person in persons.iter() {
        if person.group_number.unwrap() == previous_group {
            return false;
        }
        previous_group = person.group_number.unwrap();
    }
    true
}

fn is_gift_circle_valid(persons: &[Person]) -> bool {
    first_and_last_groups_are_different(persons) && has_no_consecutive_group_numbers(persons)
}

fn move_person(from_persons: &mut Vec<Person>, to_persons: &mut Vec<Person>, person: &Person) {
    from_persons.retain(|p| p != person);
    to_persons.push(person.clone());
}

fn generate_path(from_persons: &[Person]) -> Vec<Person> {
    // Go through the list of available participants and generate a gift path
    // where noone gives a gift to anyone in their same group.

    // Preserve the from_persons vec for follow on attempts by working with a cloned vec
    let mut available_persons: Vec<Person> = from_persons.to_owned();

    // Build up the path by adding persons with different group numbers
    let mut persons_path: Vec<Person> = vec![];

    let mut previous_group: u16 = 0;

    while !available_persons.is_empty() {
        // If the largest group is half (or more) than the total remaining, we have
        // to pick someone from that group. Otherwise, pick randomly.

        let largest_np_group = largest_non_prev_group(&available_persons, previous_group);

        let candidates: Vec<Person> =
            if (largest_np_group.size as usize * 2) > available_persons.len() {
                // Build candidates list from the remaining persons in the largest group that is not the previous group
                available_persons
                    .iter()
                    .filter(|&p| p.group_number.unwrap() == largest_np_group.number)
                    .cloned()
                    .collect::<Vec<Person>>()
            } else {
                // Build the candidates list from all remaining persons not in the previous group
                available_persons
                    .iter()
                    .filter(|&p| p.group_number.unwrap() != previous_group)
                    .cloned()
                    .collect::<Vec<Person>>()
            };

        // Randomly select one person from the candidates list
        let choice = candidates.choose(&mut rand::thread_rng()).unwrap();

        // Move the selected person from the available list to the path list
        move_person(&mut available_persons, &mut persons_path, choice);

        previous_group = choice.group_number.unwrap();
    }

    persons_path
}

fn generate_no_group_path(from_persons: &[Person]) -> Vec<Person> {
    // Go through the list of available participants and generate a gift path.

    // Preserve the from_persons vec for follow on attempts by working with a cloned vec
    let mut available_persons: Vec<Person> = from_persons.to_owned();

    // Build up the path by adding random persons
    let mut persons_path: Vec<Person> = vec![];

    while !available_persons.is_empty() {
        // Randomly select one person
        let choice = available_persons
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();

        // Move the selected person from the available list to the path list
        move_person(&mut available_persons, &mut persons_path, &choice);
    }

    persons_path
}

pub fn get_gift_circle(from_persons: Vec<Person>, use_groups: bool) -> Result<Vec<Person>> {
    if from_persons.len() <= 2 {
        return Err(anyhow!(
            "You must submit at least three people in order to form a gift circle."
        ));
    }

    let duplicates: Vec<String> = get_duplicate_names(&from_persons);
    if !duplicates.is_empty() {
        return Err(anyhow!("Found duplicate names: {:#?}", duplicates));
    }

    if use_groups {
        let possible_path = has_possible_hamiltonian_path(&from_persons);
        if !possible_path {
            return Err(anyhow!(
                "Sorry, no possible hamiltonian path with this set of groups."
            ));
        }
    }

    const MAX_ATTEMPTS: u16 = 100;
    let mut attempt_count: u16 = 0;

    let mut have_valid_circle = false;
    let mut gift_circle: Vec<Person> = vec![];

    while !have_valid_circle && attempt_count < MAX_ATTEMPTS {
        if use_groups {
            gift_circle = generate_path(&from_persons);
            if is_gift_circle_valid(&gift_circle) {
                have_valid_circle = true;
            }
        } else {
            gift_circle = generate_no_group_path(&from_persons);
            if from_persons.len() == gift_circle.len() {
                have_valid_circle = true;
            }
        }
        attempt_count += 1;
    }

    if attempt_count == MAX_ATTEMPTS {
        return Err(anyhow!(
            "Sorry, could not find gift circle in {} attempts",
            MAX_ATTEMPTS
        ));
    }

    let last_person_name = gift_circle.last().unwrap().name.clone();

    for (i, person) in gift_circle.clone().iter().enumerate() {
        if person.name == last_person_name {
            gift_circle[i].assigned_person_name = Some(gift_circle[0].name.clone());
        } else {
            gift_circle[i].assigned_person_name = Some(gift_circle[i + 1].name.clone());
        }
    }

    if use_groups {
        eprintln!("#INFO: Found valid gift circle USING groups in {attempt_count} attempts");
    } else {
        eprintln!("#INFO: Found valid gift circle NOT USING groups in {attempt_count} attempts");
    }

    Ok(gift_circle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_group() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
        ];
        assert_eq!(largest_group(&participants), Group::new(1, 2u16));
    }

    #[test]
    fn test_largest_non_prev_group() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ];
        assert_eq!(
            largest_non_prev_group(&participants, 2),
            Group::new(1, 2u16)
        );
    }

    #[test]
    fn test_has_possible_hamiltonian_path_true() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ];
        assert!(has_possible_hamiltonian_path(&participants));
    }

    #[test]
    fn test_has_possible_hamiltonian_path_false() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 1),
            Person::new("Daughter", 2),
        ];
        assert!(!has_possible_hamiltonian_path(&participants));
    }

    #[test]
    fn test_get_duplicate_names() {
        let participants = vec![Person::new("Mother", 1), Person::new("Mother", 1)];
        assert_eq!(get_duplicate_names(&participants).len(), 1);
    }

    #[test]
    fn test_first_and_last_groups_are_different_true() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
        assert!(first_and_last_groups_are_different(&participants));
    }

    #[test]
    fn test_first_and_last_groups_are_different_false() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
        ];
        assert!(!first_and_last_groups_are_different(&participants));
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_true() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
        assert!(has_no_consecutive_group_numbers(&participants));
    }

    #[test]
    fn test_has_no_consecutive_group_numbers_false() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 2),
            Person::new("Daughter", 3),
        ];
        assert!(!has_no_consecutive_group_numbers(&participants));
    }

    #[test]
    fn test_is_gift_circle_valid_true() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 2),
            Person::new("Son", 1),
            Person::new("Daughter", 3),
        ];
        assert!(is_gift_circle_valid(&participants));
    }

    #[test]
    fn test_move_person() {
        let person1 = Person::new("Father", 1);
        let person_to_move = person1.clone();
        let mut move_from = vec![person1];
        let mut move_to: Vec<Person> = vec![];
        move_person(&mut move_from, &mut move_to, &person_to_move);
        assert_eq!(move_from.len(), 0);
        assert_eq!(move_to.len(), 1);
    }

    #[test]
    fn test_get_gift_circle_using_groups() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ];
        if let Ok(gift_circle) = get_gift_circle(participants, true) {
            assert_eq!(gift_circle.len(), 4);
        }
    }

    #[test]
    fn test_get_gift_circle_not_using_groups() {
        let participants = vec![
            Person::new_no_group("Father"),
            Person::new_no_group("Mother"),
            Person::new_no_group("Son"),
            Person::new_no_group("Daughter"),
        ];
        if let Ok(gift_circle) = get_gift_circle(participants, false) {
            assert_eq!(gift_circle.len(), 4);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_gift_circle_errors_with_too_few_entries() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
        ];
        get_gift_circle(participants, true).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_gift_circle_errors_with_duplicate_names() {
        let participants = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Father", 3),
        ];
        get_gift_circle(participants, true).unwrap();
    }
}
