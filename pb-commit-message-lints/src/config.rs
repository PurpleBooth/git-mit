use git2::ConfigEntries;
use std::{collections::HashMap, error::Error};

pub trait Vcs {
    fn get_bool(&self, name: &str) -> Option<bool>;
}

pub struct InMemoryVcs {
    bool_configs: HashMap<String, bool>,
}

impl InMemoryVcs {
    #[must_use]
    pub fn new(bool_configs: HashMap<String, bool>) -> InMemoryVcs {
        InMemoryVcs { bool_configs }
    }
}

impl Vcs for InMemoryVcs {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.bool_configs.get(name).map(bool::clone)
    }
}

pub struct Git2Vcs {
    git2_config: git2::Config,
}

impl Git2Vcs {
    #[must_use]
    pub fn new(git2_config: git2::Config) -> Git2Vcs {
        Git2Vcs { git2_config }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool, Box<dyn Error>> {
        let at_least_one = |x: ConfigEntries| x.count() > 0;
        self.git2_config
            .entries(Some(lint_name))
            .map(at_least_one)
            .map_err(Box::from)
    }
}

impl Vcs for Git2Vcs {
    fn get_bool(&self, name: &str) -> Option<bool> {
        if let Ok(true) = self.config_defined(name) {
        } else {
            return None;
        }

        self.git2_config.get_bool(name).ok()
    }
}
