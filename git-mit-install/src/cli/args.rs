use clap::ArgMatches;
use mit_commit_message_lints::console::completion::Shell;

pub struct Args {
    matches: ArgMatches,
}

impl Args {
    pub(crate) fn scope(&self) -> Scope {
        if self.matches.value_of("scope") == Some("global") {
            Scope::Global
        } else {
            Scope::Local
        }
    }

    pub fn completion(&self) -> Option<Shell> {
        self.matches.value_of_t::<Shell>("completion").ok()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Scope {
    Global,
    Local,
}

impl Scope {
    pub(crate) fn is_global(&self) -> bool {
        &Self::Global == self
    }
}

impl Scope {}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Self { matches }
    }
}
