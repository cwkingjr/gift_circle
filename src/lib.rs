//! Generate gift exchange assignments where recipients are randomly picked from other family groups.

#![cfg_attr(
    test,
    allow(clippy::unwrap_used, clippy::expect_used, clippy::unreachable)
)]

pub mod args;
pub mod error;
pub mod gift_circle;
pub mod group;
pub mod mode;
pub mod people;
pub mod person;

pub use error::GiftCircleError;
pub use gift_circle::{generate, generate_with_rng, GiftCircleOutput};
#[allow(deprecated)]
pub use gift_circle::{get_gift_circle, get_gift_circle_with_rng};
pub use mode::GiftMode;
pub use people::{GroupedPeople, People};
pub use person::{Participant, Person};
