use rand::prelude::{Rng, SliceRandom};

use crate::error::{GiftCircleError, Result};
use crate::people::{GroupedPeople, People};

/// A successfully generated gift circle and metadata about how it was built.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GiftCircleOutput {
    /// Participants in gift order with `assigned_person_name` populated.
    pub people: People,
    /// Number of random generation attempts required.
    pub attempts: u16,
    /// Whether group constraints were applied.
    pub used_groups: bool,
}

fn path_from_indices(people: &People, path_indices: &[usize]) -> People {
    path_indices
        .iter()
        .map(|&index| people[index].clone())
        .collect::<People>()
}

fn generate_group_path(grouped: &GroupedPeople, rng: &mut impl Rng) -> (People, Vec<usize>) {
    let people: &People = grouped;
    let mut remaining: Vec<usize> = (0..people.len()).collect();
    let mut path_indices = Vec::with_capacity(people.len());
    let mut previous_group: u16 = 0;

    while !remaining.is_empty() {
        let largest_np_group = grouped.largest_non_prev_group(previous_group, &remaining);

        let candidate_indices: Vec<usize> =
            if (largest_np_group.size as usize * 2) > remaining.len() {
                remaining
                    .iter()
                    .copied()
                    .filter(|&index| {
                        GroupedPeople::person_group(&people[index]) == largest_np_group.number
                    })
                    .collect()
            } else {
                remaining
                    .iter()
                    .copied()
                    .filter(|&index| GroupedPeople::person_group(&people[index]) != previous_group)
                    .collect()
            };

        let choice_index =
            candidate_indices[rng.random_range(0..candidate_indices.len())];

        previous_group = GroupedPeople::person_group(&people[choice_index]);
        remaining.retain(|&index| index != choice_index);
        path_indices.push(choice_index);
    }

    (path_from_indices(people, &path_indices), path_indices)
}

fn generate_no_group_path(people: &People, rng: &mut impl Rng) -> People {
    let mut path_indices: Vec<usize> = (0..people.len()).collect();
    path_indices.shuffle(rng);
    path_from_indices(people, &path_indices)
}

fn validate_people(from_people: &People, use_groups: bool) -> Result<Option<GroupedPeople>> {
    if from_people.len() <= 2 {
        return Err(GiftCircleError::TooFewParticipants {
            count: from_people.len(),
        });
    }

    let duplicates = from_people.get_duplicated_names();
    if !duplicates.is_empty() {
        return Err(GiftCircleError::DuplicateNames(duplicates));
    }

    if !use_groups {
        return Ok(None);
    }

    let grouped = GroupedPeople::try_new(from_people.clone())?;
    if !grouped.has_possible_hamiltonian_path() {
        return Err(GiftCircleError::ImpossibleGroupLayout);
    }

    Ok(Some(grouped))
}

/// Generate a gift circle using the process default random number generator.
///
/// # Errors
///
/// Returns [`GiftCircleError`] when validation fails or no valid group circle is found.
pub fn get_gift_circle(from_people: People, use_groups: bool) -> Result<GiftCircleOutput> {
    get_gift_circle_with_rng(&from_people, use_groups, &mut rand::rng())
}

/// Generate a gift circle using the provided random number generator.
///
/// # Errors
///
/// Returns [`GiftCircleError`] when validation fails or no valid group circle is found.
pub fn get_gift_circle_with_rng(
    from_people: &People,
    use_groups: bool,
    rng: &mut impl Rng,
) -> Result<GiftCircleOutput> {
    let grouped = validate_people(from_people, use_groups)?;

    if use_groups {
        const MAX_ATTEMPTS: u16 = 500;
        let grouped = grouped.ok_or(GiftCircleError::MissingGroup)?;
        let mut attempt_count: u16 = 0;
        let mut gift_path = People::default();

        while attempt_count < MAX_ATTEMPTS {
            attempt_count += 1;
            let (path, path_indices) = generate_group_path(&grouped, rng);
            if grouped.is_valid_gift_circle(&path_indices) {
                gift_path = path;
                break;
            }
        }

        if gift_path.is_empty() {
            return Err(GiftCircleError::ExhaustedAttempts {
                attempts: MAX_ATTEMPTS,
            });
        }

        gift_path.assign_gift_recipients();
        return Ok(GiftCircleOutput {
            people: gift_path,
            attempts: attempt_count,
            used_groups: true,
        });
    }

    let mut gift_path = generate_no_group_path(from_people, rng);
    gift_path.assign_gift_recipients();
    Ok(GiftCircleOutput {
        people: gift_path,
        attempts: 1,
        used_groups: false,
    })
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;
    use crate::person::Person;

    fn assert_valid_assignments(output: &GiftCircleOutput) {
        let names: Vec<&str> = output.people.iter().map(|p| p.name.as_str()).collect();
        let recipients: Vec<&str> = output
            .people
            .iter()
            .map(|p| p.assigned_person_name.as_deref().unwrap())
            .collect();

        let mut sorted_names = names.clone();
        let mut sorted_recipients = recipients;
        sorted_names.sort_unstable();
        sorted_recipients.sort_unstable();
        assert_eq!(sorted_names, sorted_recipients);

        for person in output.people.iter() {
            assert_ne!(
                person.assigned_person_name.as_deref(),
                Some(person.name.as_str())
            );
        }
    }

    #[test]
    fn test_get_gift_circle_using_groups() {
        let people = People::from(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ]);
        let mut rng = StdRng::seed_from_u64(42);
        let output = get_gift_circle_with_rng(&people, true, &mut rng).expect("valid fixture");
        assert_eq!(output.people.len(), 4);
        assert!(output.used_groups);
        assert_valid_assignments(&output);
    }

    #[test]
    fn test_get_gift_circle_not_using_groups() {
        let people = People::from(vec![
            Person::new_no_group("Father"),
            Person::new_no_group("Mother"),
            Person::new_no_group("Son"),
            Person::new_no_group("Daughter"),
        ]);
        let mut rng = StdRng::seed_from_u64(7);
        let output = get_gift_circle_with_rng(&people, false, &mut rng).expect("valid fixture");
        assert_eq!(output.people.len(), 4);
        assert_eq!(output.attempts, 1);
        assert!(!output.used_groups);
        assert_valid_assignments(&output);
    }

    #[test]
    fn test_get_gift_circle_is_reproducible_with_seeded_rng() {
        let people = People::from(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Daughter", 2),
        ]);
        let mut rng_a = StdRng::seed_from_u64(99);
        let mut rng_b = StdRng::seed_from_u64(99);
        let output_a = get_gift_circle_with_rng(&people, true, &mut rng_a).unwrap();
        let output_b = get_gift_circle_with_rng(&people, true, &mut rng_b).unwrap();
        assert_eq!(output_a.people, output_b.people);
    }

    #[test]
    fn test_get_gift_circle_errors_with_too_few_entries() {
        let people = People::from(vec![Person::new("Father", 1), Person::new("Mother", 1)]);
        let err = get_gift_circle(people, true).unwrap_err();
        assert!(matches!(
            err,
            GiftCircleError::TooFewParticipants { count: 2 }
        ));
    }

    #[test]
    fn test_get_gift_circle_errors_with_duplicate_names() {
        let people = People::from(vec![
            Person::new("Father", 1),
            Person::new("Mother", 1),
            Person::new("Son", 2),
            Person::new("Father", 3),
        ]);
        let err = get_gift_circle(people, true).unwrap_err();
        assert!(matches!(err, GiftCircleError::DuplicateNames(_)));
    }
}
