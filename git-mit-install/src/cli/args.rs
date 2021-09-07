use clap::ArgMatches;

pub(crate) struct Args {
    matches: ArgMatches,
}

impl Args {
    pub(crate) fn scope(&self) -> Scope {
        if let Some("global") = self.matches.value_of("scope") {
            Scope::Global
        } else {
            Scope::Local
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub(crate) enum Scope {
    Global,
    Local,
}

impl Scope {
    pub(crate) fn is_global(&self) -> bool {
        &Scope::Global == self
    }
}

impl Scope {}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Args { matches }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::args::Scope;

    use super::Args;

    #[test]
    fn can_tell_me_if_its_global() {
        let app = super::super::app::app();
        let matches = app.get_matches_from(vec!["binary", "--scope=global"]);
        let actual = Args::from(matches);

        assert_eq!(actual.scope(), Scope::Global);
        assert!(actual.scope().is_global());
    }

    #[test]
    fn can_tell_me_if_its_local() {
        let app = super::super::app::app();
        let matches = app.get_matches_from(vec!["binary"]);
        let actual = Args::from(matches);

        assert_eq!(actual.scope(), Scope::Local);
    }
}
