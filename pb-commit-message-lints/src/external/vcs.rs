use std::{clone::Clone, collections::HashMap, error::Error};

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
    bools: &'a HashMap<String, bool>,
    strs: &'a mut HashMap<String, String>,
    i64s: &'a mut HashMap<String, i64>,
}

impl InMemory<'_> {
    #[must_use]
    pub fn new<'a>(
        bools: &'a HashMap<String, bool>,
        strs: &'a mut HashMap<String, String>,
        i64s: &'a mut HashMap<String, i64>,
    ) -> InMemory<'a> {
        InMemory { bools, strs, i64s }
    }
}

impl Vcs for InMemory<'_> {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.bools.get(name).map(bool::clone)
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.strs.get(name).map(std::string::String::as_str)
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.i64s.get(name).map(i64::clone)
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.strs.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>> {
        self.i64s.insert(name.into(), value);
        Ok(())
    }
}

pub struct Git2 {
    config: git2::Config,
}

impl Git2 {
    #[must_use]
    pub fn new(config: git2::Config) -> Git2 {
        Git2 { config }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool, Box<dyn Error>> {
        self.config
            .entries(Some(lint_name))
            .map(|entries| entries.count() > 0)
            .map_err(Box::from)
    }
}

impl Vcs for Git2 {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.config.get_bool(name).ok())
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.config.get_str(name).ok())
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.config.get_i64(name).ok())
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.config.set_str(name, value).map_err(Box::from)
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>> {
        self.config.set_i64(name, value).map_err(Box::from)
    }
}
