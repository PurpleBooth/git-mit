use git2::ConfigEntries;
use std::{collections::HashMap, error::Error};

pub trait Vcs {
    fn get_bool(&self, name: &str) -> Option<bool>;
    fn get_str(&self, name: &str) -> Option<&str>;
    fn get_i64(&self, name: &str) -> Option<i64>;
}

pub struct InMemoryVcs {
    bool_configs: HashMap<String, bool>,
    str_configs: HashMap<String, String>,
    i64_configs: HashMap<String, i64>,
}

impl InMemoryVcs {
    #[must_use]
    pub fn new(
        bool_configs: HashMap<String, bool>,
        str_configs: HashMap<String, String>,
        i64_configs: HashMap<String, i64>,
    ) -> InMemoryVcs {
        InMemoryVcs {
            bool_configs,
            str_configs,
            i64_configs,
        }
    }
}

impl Vcs for InMemoryVcs {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.bool_configs.get(name).map(bool::clone)
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.str_configs.get(name).map(std::string::String::as_str)
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.i64_configs.get(name).map(i64::clone)
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

    fn get_str(&self, name: &str) -> Option<&str> {
        if let Ok(true) = self.config_defined(name) {
        } else {
            return None;
        }

        self.git2_config.get_str(name).ok()
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        if let Ok(true) = self.config_defined(name) {
        } else {
            return None;
        }

        self.git2_config.get_i64(name).ok()
    }
}
