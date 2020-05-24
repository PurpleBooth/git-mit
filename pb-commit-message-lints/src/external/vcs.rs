use std::{clone::Clone, collections::HashMap, error::Error};

use git2::ConfigEntries;

pub trait Vcs {
    fn get_bool(&self, name: &str) -> Option<bool>;
    fn get_str(&self, name: &str) -> Option<&str>;
    fn get_i64(&self, name: &str) -> Option<i64>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>>;
}

pub struct InMemory<'a> {
    bool_configs: &'a HashMap<String, bool>,
    str_configs: &'a mut HashMap<String, String>,
    i64_configs: &'a mut HashMap<String, i64>,
}

impl InMemory<'_> {
    #[must_use]
    pub fn new<'a>(
        bool_configs: &'a HashMap<String, bool>,
        str_configs: &'a mut HashMap<String, String>,
        i64_configs: &'a mut HashMap<String, i64>,
    ) -> InMemory<'a> {
        InMemory {
            bool_configs,
            str_configs,
            i64_configs,
        }
    }
}

impl Vcs for InMemory<'_> {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.bool_configs.get(name).map(bool::clone)
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.str_configs.get(name).map(std::string::String::as_str)
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.i64_configs.get(name).map(i64::clone)
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.str_configs.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>> {
        self.i64_configs.insert(name.into(), value);
        Ok(())
    }
}

pub struct Git2 {
    git2_config: git2::Config,
}

impl Git2 {
    #[must_use]
    pub fn new(git2_config: git2::Config) -> Git2 {
        Git2 { git2_config }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool, Box<dyn Error>> {
        let at_least_one = |x: ConfigEntries| x.count() > 0;
        self.git2_config
            .entries(Some(lint_name))
            .map(at_least_one)
            .map_err(Box::from)
    }
}

impl Vcs for Git2 {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.git2_config.get_bool(name).ok())
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.git2_config.get_str(name).ok())
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.git2_config.get_i64(name).ok())
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.git2_config.set_str(name, value).map_err(Box::from)
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>> {
        self.git2_config.set_i64(name, value).map_err(Box::from)
    }
}
