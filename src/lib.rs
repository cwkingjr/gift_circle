//! Generate gift exchange assignments where recipients are randomly picked from other family groups.

pub mod args;
pub mod error;
pub mod gift_circle;
pub mod group;
pub mod people;
pub mod person;

pub use error::GiftCircleError;
pub use gift_circle::{get_gift_circle, get_gift_circle_with_rng, GiftCircleOutput};
pub use people::{GroupedPeople, People};
pub use person::Person;
