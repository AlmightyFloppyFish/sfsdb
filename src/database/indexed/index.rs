use hashbrown::HashMap;

use crate::error::DBError;
use crate::filesystem;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const INDEX_FOLDER: &str = "__INDEX__";

// TODO: Make it into a {} struct that contains both path to full and cached copy
pub struct Index<T>(pub HashMap<String, T>);

impl<T> Index<T> {
    pub fn new() -> Self {
        Index(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.0.get(key)
    }

    pub fn disk_save(index: &T, location: &str, key: &str) -> Result<(), DBError>
    where
        T: Serialize,
    {
        let mut path = PathBuf::new();
        path.push(location);
        path.push(INDEX_FOLDER);
        path.push(key);
        filesystem::fs_save(&path, index)
    }

    pub fn disk_load(location: &str, key: &str) -> Result<T, DBError>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut path = PathBuf::new();
        path.push(location);
        path.push(INDEX_FOLDER);
        path.push(key);
        Ok(filesystem::fs_load(&path)?)
    }

    pub fn attach(&mut self, key: &str, index: T) {
        self.0.insert(key.to_owned(), index);
    }

    pub fn update<F, I>(&mut self, key: &str, location: &str, mut apply: F) -> Result<(), DBError>
    where
        T: Serialize,
        for<'de> I: Deserialize<'de> + Serialize + Clone,
        F: FnMut(T) -> T,
    {
        match self.0.remove(key) {
            Some(index) => {
                let new = apply(index);
                Index::disk_save(&new, location, key)?;
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
        self.0.remove(key);
    }
}

pub fn disk_delete(location: &str, key: &str) {
    let mut path = PathBuf::new();
    path.push(location);
    path.push(INDEX_FOLDER);
    path.push(key);
    filesystem::fs_delete(&path);
}
