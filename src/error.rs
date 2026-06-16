use thiserror::Error;

pub type Result<T> = std::result::Result<T, GiftCircleError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GiftCircleError {
    #[error("You must submit at least three people in order to form a gift circle.")]
    TooFewParticipants { count: usize },
    #[error("Found duplicate names: {0:?}")]
    DuplicateNames(Vec<String>),
    #[error("When using groups each participant must have a group assigned!")]
    MissingGroup,
    #[error("Sorry, no possible hamiltonian path with this set of groups.")]
    ImpossibleGroupLayout,
    #[error("Sorry, could not find gift circle in {attempts} attempts")]
    ExhaustedAttempts { attempts: u16 },
}
