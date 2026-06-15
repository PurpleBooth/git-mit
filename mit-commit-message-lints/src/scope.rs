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

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn global_scope_is_global() {
        assert!(
            Scope::Global.is_global(),
            "Expected the Global scope to report as global"
        );
    }

    #[test]
    fn local_scope_is_not_global() {
        assert!(
            !Scope::Local.is_global(),
            "Expected the Local scope to not report as global"
        );
    }
}
