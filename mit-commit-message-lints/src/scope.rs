//! A module representing the scope

/// The scopes we might read the config for
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, clap::ValueEnum, Copy)]
pub enum Scope {
    /// The home directory
    Global,
    /// The local folder
    Local,
}
impl Scope {
    /// If this scope is global or not
    #[must_use]
    pub fn is_global(&self) -> bool {
        &Self::Global == self
    }
}

impl Scope {}
