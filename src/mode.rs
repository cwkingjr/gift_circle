/// Whether gift assignments must respect family group boundaries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GiftMode {
    /// Any participant may be assigned any other participant.
    Plain,
    /// Participants must not give gifts to members of their own group.
    Grouped,
}

impl GiftMode {
    pub(crate) fn uses_groups(self) -> bool {
        matches!(self, Self::Grouped)
    }
}

impl From<bool> for GiftMode {
    fn from(use_groups: bool) -> Self {
        if use_groups {
            Self::Grouped
        } else {
            Self::Plain
        }
    }
}
