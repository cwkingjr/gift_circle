use rand::prelude::{Rng, RngExt, SliceRandom};

use crate::error::{GiftCircleError, Result};
use crate::mode::GiftMode;
use crate::people::{GroupedPeople, People};
use crate::person::Person;

/// A successfully generated gift circle and metadata about how it was built.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GiftCircleOutput {
    /// Participants in gift order with assigned recipients populated.
    pub people: Vec<Person>,
    /// Number of random generation attempts required.
    pub attempts: u16,
    /// Whether group constraints were applied.
    pub used_groups: bool,
}

enum ValidatedPeople {
    Plain,
    Grouped(GroupedPeople),
}

fn validate_people(from_people: &People, mode: GiftMode) -> Result<ValidatedPeople> {
    if from_people.len() <= 2 {
        return Err(GiftCircleError::TooFewParticipants {
            count: from_people.len(),
        });
    }

    let duplicates = from_people.duplicated_names();
    if !duplicates.is_empty() {
        return Err(GiftCircleError::DuplicateNames(duplicates));
    }

    match mode {
        GiftMode::Plain => Ok(ValidatedPeople::Plain),
        GiftMode::Grouped => GroupedPeople::try_from(from_people).map(ValidatedPeople::Grouped),
    }
}

fn generate_group_path_indices(grouped: &GroupedPeople, rng: &mut impl Rng) -> Vec<usize> {
    let people: &People = grouped;
    let mut remaining: Vec<usize> = (0..people.len()).collect();
    let mut path_indices = Vec::with_capacity(people.len());
    let mut previous_group: u16 = 0;

    while !remaining.is_empty() {
        let largest_np_group = grouped.largest_non_prev_group(previous_group, &remaining);

        let candidate_indices: Vec<usize> = if (largest_np_group.size as usize * 2)
            > remaining.len()
        {
            remaining
                .iter()
                .copied()
                .filter(|&index| GroupedPeople::group_at(people, index) == largest_np_group.number)
                .collect()
        } else {
            remaining
                .iter()
                .copied()
                .filter(|&index| GroupedPeople::group_at(people, index) != previous_group)
                .collect()
        };

        let choice_pos = rng.random_range(0..candidate_indices.len());
        let choice_index = candidate_indices[choice_pos];

        previous_group = GroupedPeople::group_at(people, choice_index);
        if let Some(pos) = remaining.iter().position(|&index| index == choice_index) {
            remaining.swap_remove(pos);
        }
        path_indices.push(choice_index);
    }

    path_indices
}

fn generate_no_group_path_indices(people: &People, rng: &mut impl Rng) -> Vec<usize> {
    let mut path_indices: Vec<usize> = (0..people.len()).collect();
    path_indices.shuffle(rng);
    path_indices
}

/// Generate a gift circle using the process default random number generator.
///
/// # Errors
///
/// Returns [`GiftCircleError`] when validation fails or no valid group circle is found.
pub fn generate(from_people: &People, mode: GiftMode) -> Result<GiftCircleOutput> {
    generate_with_rng(from_people, mode, &mut rand::rng())
}

/// Generate a gift circle using the provided random number generator.
///
/// # Errors
///
/// Returns [`GiftCircleError`] when validation fails or no valid group circle is found.
pub fn generate_with_rng(
    from_people: &People,
    mode: GiftMode,
    rng: &mut impl Rng,
) -> Result<GiftCircleOutput> {
    match validate_people(from_people, mode)? {
        ValidatedPeople::Plain => {
            let path = generate_no_group_path_indices(from_people, rng);
            Ok(GiftCircleOutput {
                people: from_people.assign_from_path(&path),
                attempts: 1,
                used_groups: mode.uses_groups(),
            })
        }
        ValidatedPeople::Grouped(grouped) => {
            const MAX_ATTEMPTS: u16 = 500;
            let mut attempt_count: u16 = 0;
            let mut path = Vec::new();

            while attempt_count < MAX_ATTEMPTS {
                attempt_count += 1;
                let candidate_path = generate_group_path_indices(&grouped, rng);
                if grouped.is_valid_gift_circle(&candidate_path) {
                    path = candidate_path;
                    break;
                }
            }

            if path.is_empty() {
                return Err(GiftCircleError::ExhaustedAttempts {
                    attempts: MAX_ATTEMPTS,
                });
            }

            Ok(GiftCircleOutput {
                people: from_people.assign_from_path(&path),
                attempts: attempt_count,
                used_groups: mode.uses_groups(),
            })
        }
    }
}

#[deprecated(note = "renamed to `generate`")]
pub fn get_gift_circle(from_people: People, use_groups: bool) -> Result<GiftCircleOutput> {
    generate(&from_people, use_groups.into())
}

#[deprecated(note = "renamed to `generate_with_rng`")]
pub fn get_gift_circle_with_rng(
    from_people: &People,
    use_groups: bool,
    rng: &mut impl Rng,
) -> Result<GiftCircleOutput> {
    generate_with_rng(from_people, use_groups.into(), rng)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;
    use crate::person::Participant;

    fn assert_valid_assignments(output: &GiftCircleOutput) {
        let names: Vec<&str> = output.people.iter().map(|p| p.name.as_str()).collect();
        let recipients: Vec<&str> = output
            .people
            .iter()
            .map(|p| p.assigned_person_name.as_str())
            .collect();

        let mut sorted_names = names.clone();
        let mut sorted_recipients = recipients;
        sorted_names.sort_unstable();
        sorted_recipients.sort_unstable();
        assert_eq!(sorted_names, sorted_recipients);

        for person in &output.people {
            assert_ne!(person.assigned_person_name, person.name);
        }
    }

    #[test]
    fn generate_using_groups() {
        let people = People::from(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
            Participant::new("Daughter", 2),
        ]);
        let mut rng = StdRng::seed_from_u64(42);
        let output =
            generate_with_rng(&people, GiftMode::Grouped, &mut rng).expect("valid fixture");
        assert_eq!(output.people.len(), 4);
        assert!(output.used_groups);
        assert_valid_assignments(&output);
    }

    #[test]
    fn generate_not_using_groups() {
        let people = People::from(vec![
            Participant::new_no_group("Father"),
            Participant::new_no_group("Mother"),
            Participant::new_no_group("Son"),
            Participant::new_no_group("Daughter"),
        ]);
        let mut rng = StdRng::seed_from_u64(7);
        let output = generate_with_rng(&people, GiftMode::Plain, &mut rng).expect("valid fixture");
        assert_eq!(output.people.len(), 4);
        assert_eq!(output.attempts, 1);
        assert!(!output.used_groups);
        assert_valid_assignments(&output);
    }

    #[test]
    fn generate_is_reproducible_with_seeded_rng() {
        let people = People::from(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
            Participant::new("Daughter", 2),
        ]);
        let mut rng_a = StdRng::seed_from_u64(99);
        let mut rng_b = StdRng::seed_from_u64(99);
        let output_a = generate_with_rng(&people, GiftMode::Grouped, &mut rng_a).unwrap();
        let output_b = generate_with_rng(&people, GiftMode::Grouped, &mut rng_b).unwrap();
        assert_eq!(output_a.people, output_b.people);
    }

    #[test]
    fn generate_errors_with_too_few_entries() {
        let people = People::from(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
        ]);
        let err = generate(&people, GiftMode::Grouped).unwrap_err();
        assert!(matches!(
            err,
            GiftCircleError::TooFewParticipants { count: 2 }
        ));
    }

    #[test]
    fn generate_errors_with_duplicate_names() {
        let people = People::from(vec![
            Participant::new("Father", 1),
            Participant::new("Mother", 1),
            Participant::new("Son", 2),
            Participant::new("Father", 3),
        ]);
        let err = generate(&people, GiftMode::Grouped).unwrap_err();
        assert!(matches!(err, GiftCircleError::DuplicateNames(_)));
    }
}

#[cfg(test)]
mod proptests {
    use proptest::prelude::*;
    use proptest::test_runner::TestCaseError;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;
    use crate::person::Participant;

    fn grouped_people_from_counts(counts: &[(u16, usize)]) -> People {
        let mut participants = Vec::new();
        for &(group, size) in counts {
            for index in 0..size {
                participants.push(Participant {
                    name: format!("g{group}-p{index}"),
                    email_address: None,
                    group_number: Some(group),
                });
            }
        }
        People::from(participants)
    }

    proptest! {
        #[test]
        fn feasible_group_layouts_generate_within_attempt_limit(
            g1 in 2u16..=4,
            g2 in 2u16..=4,
            seed in any::<u64>(),
        ) {
            let people = grouped_people_from_counts(&[(1, g1 as usize), (2, g2 as usize)]);
            prop_assume!(GroupedPeople::try_from(&people).is_ok());

            let mut rng = StdRng::seed_from_u64(seed);
            let output = generate_with_rng(&people, GiftMode::Grouped, &mut rng)
                .map_err(|err| TestCaseError::fail(format!("{err:?}")))?;
            prop_assert!(output.used_groups);
            prop_assert_eq!(output.people.len(), people.len());
            prop_assert!(output.attempts <= 500);
        }
    }
}
