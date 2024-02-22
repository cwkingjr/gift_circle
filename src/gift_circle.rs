use std::u16;

use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;

use super::people::{People, PeopleCycle};
use super::person::Person;

/// Move the person from the available/from list to the path/to list
fn move_person(from_people: &mut People, to_people: &mut People, person: &Person) {
    from_people.retain(|p| p != person);
    to_people.push(person.clone());
}

fn generate_group_path(from_people: &mut People) -> People {
    // Go through the list of available people and generate a gift path
    // where noone gives a gift to anyone in their same group.

    // Build up the path by adding persons with different group numbers
    let mut people_path: People = vec![];

    let mut previous_group: u16 = 0;

    while !from_people.is_empty() {
        // If the largest group is half (or more) than the total remaining, we have
        // to pick someone from that group. Otherwise, pick randomly.

        let largest_np_group = from_people.largest_non_prev_group(previous_group);

        let candidates: People = if (largest_np_group.size as usize * 2) > from_people.len() {
            // Build candidates list from the remaining persons in the largest group that is not the previous group
            from_people
                .iter()
                .filter(|&p| p.group_number.unwrap() == largest_np_group.number)
                .cloned()
                .collect::<People>()
        } else {
            // Build the candidates list from all remaining persons not in the previous group
            from_people
                .iter()
                .filter(|&p| p.group_number.unwrap() != previous_group)
                .cloned()
                .collect::<People>()
        };

        // Randomly select one person from the candidates list
        let choice = candidates.choose(&mut rand::thread_rng()).unwrap();

        move_person(from_people, &mut people_path, choice);

        previous_group = choice.group_number.unwrap();
    }

    people_path
}

fn generate_no_group_path(from_people: &mut People) -> People {
    // Go through the list of available people and generate a gift path.

    let mut people_path: People = vec![];

    while !from_people.is_empty() {
        // Randomly select one person
        let choice = from_people.choose(&mut rand::thread_rng()).unwrap().clone();

        // Move the selected person from the available list to the path list
        move_person(from_people, &mut people_path, &choice);
    }

    people_path
}

/// Runs several validation checks on `from_people`, makes numerous attempts to generate
/// a valid group or no group gift circle, returns a result with a valid gift circle
/// (`People` with `assigned_person_name`'s populated) or an error,
/// and stderr prints the number of attempts taken.
pub fn get_gift_circle(from_people: People, use_groups: bool) -> Result<People> {
    if from_people.len() <= 2 {
        return Err(anyhow!(
            "You must submit at least three people in order to form a gift circle."
        ));
    }

    let duplicates: Vec<String> = from_people.get_duplicated_names();
    if !duplicates.is_empty() {
        return Err(anyhow!("Found duplicate names: {:#?}", duplicates));
    }

    if use_groups {
        if from_people.has_empty_group() {
            return Err(anyhow!(
                "When using groups each participant must have a group assigned!"
            ));
        }

        if !from_people.has_possible_hamiltonian_path() {
            return Err(anyhow!(
                "Sorry, no possible hamiltonian path with this set of groups."
            ));
        }
    }

    const MAX_ATTEMPTS: u16 = 500;
    let mut attempt_count: u16 = 0;

    let mut have_valid_circle = false;
    let mut gift_path: People = vec![];

    while !have_valid_circle && attempt_count < MAX_ATTEMPTS {
        // Preserve the from_people vec for follow on attempts by working with a cloned vec
        let mut available_people: People = from_people.to_owned();

        if use_groups {
            gift_path = generate_group_path(&mut available_people);
            if gift_path.is_valid_gift_circle() {
                have_valid_circle = true;
            }
        } else {
            gift_path = generate_no_group_path(&mut available_people);
            if from_people.len() == gift_path.len() {
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

    gift_path.assign_gift_recipients();

    if use_groups {
        eprintln!("#INFO: Found valid gift circle USING groups in {attempt_count} attempts");
    } else {
        eprintln!("#INFO: Found valid gift circle NOT USING groups in {attempt_count} attempts");
    }

    Ok(gift_path)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_person() {
        let person1 = Person::new("Father", 1);
        let person_to_move = person1.clone();
        let mut move_from = vec![person1];
        let mut move_to = vec![];
        move_person(&mut move_from, &mut move_to, &person_to_move);
        assert_eq!(move_from.len(), 0);
        assert_eq!(move_to.len(), 1);
    }

    #[test]
    fn test_get_gift_circle_using_groups() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ];
        if let Ok(gift_circle) = get_gift_circle(people, true) {
            assert_eq!(gift_circle.len(), 4);
        }
    }

    #[test]
    fn test_get_gift_circle_not_using_groups() {
        let people = vec![
            Person::new_no_group("Father"),
            Person::new_no_group("Mother"),
            Person::new_no_group("Son"),
            Person::new_no_group("Daughter"),
        ];
        if let Ok(gift_circle) = get_gift_circle(people, false) {
            assert_eq!(gift_circle.len(), 4);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_gift_circle_errors_with_too_few_entries() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
        ];
        get_gift_circle(people, true).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_gift_circle_errors_with_duplicate_names() {
        let people = vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Father", 3),
        ];
        get_gift_circle(people, true).unwrap();
    }
}
