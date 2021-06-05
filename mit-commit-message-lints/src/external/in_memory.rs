use crate::external::{Error, Vcs};
use glob::Pattern;
use std::{collections::BTreeMap, string::String};

pub struct InMemory<'a> {
    store: &'a mut BTreeMap<String, String>,
}

impl InMemory<'_> {
    #[must_use]
    pub fn new(store: &mut BTreeMap<String, String>) -> InMemory {
        InMemory { store }
    }
}

impl Vcs for InMemory<'_> {
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>, Error> {
        let mut keys: Vec<String> = self.store.keys().map(String::from).collect();

        if let Some(pattern) = glob {
            let compiled_glob = glob::Pattern::new(pattern)?;

            keys = keys
                .into_iter()
                .filter(|value| Pattern::matches(&compiled_glob, value))
                .collect();
        }

        Ok(keys)
    }

    fn get_bool(&self, name: &str) -> Result<Option<bool>, Error> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse().map_err(Error::from)?)),
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>, Error> {
        Ok(self.store.get(name).map(String::as_str))
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, Error> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse().map_err(Error::from)?)),
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
        self.store.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Error> {
        self.store.insert(name.into(), format!("{}", value));
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), Error> {
        self.store.remove(name);
        Ok(())
    }
}
