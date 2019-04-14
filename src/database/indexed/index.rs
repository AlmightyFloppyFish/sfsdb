use hashbrown::HashMap;

use crate::error::DBError;
use crate::filesystem;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

const INDEX_FOLDER: &str = "__INDEX__";

// TODO: Make it into a {} struct that contains both path to full and cached copy
pub struct Index<T> {
    pub location: PathBuf,
    pub mem: HashMap<String, T>,
}

impl<T> Index<T> {
    pub fn new(db_root: &str) -> Self {
        Index {
            location: PathBuf::from_str(&format!("{}/{}/", db_root, INDEX_FOLDER)).unwrap(),
            mem: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.mem.get(key)
    }

    pub fn disk_save(&mut self, index: &T, key: &str) -> Result<(), DBError>
    where
        T: Serialize,
    {
        self.location.push(key);
        filesystem::fs_save(&self.location, index)?;
        self.location.pop();
        Ok(())
    }

    pub fn attach(&mut self, key: &str, index: T) {
        self.mem.insert(key.to_owned(), index);
    }

    pub fn update<F, I>(&mut self, key: &str, mut apply: F) -> Result<(), DBError>
    where
        T: Serialize,
        for<'de> I: Deserialize<'de> + Serialize + Clone,
        F: FnMut(T) -> T,
    {
        match self.mem.remove(key) {
            Some(index) => {
                let new = apply(index);
                self.disk_save(&new, key)?;
                Ok(self.attach(key, new))
            }
            None => {
                return Err(DBError::index(&format!(
                    "key {} not found when attempting to edit its index",
                    key
                )));
            }
        }
    }

    pub fn delete(&mut self, key: &str) {
        self.mem.remove(key);
    }

    pub fn disk_delete(&mut self, key: &str) {
        self.location.push(key);
        filesystem::fs_delete(&self.location)
    }
}
