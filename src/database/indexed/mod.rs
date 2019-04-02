mod index;

use crate::error::DBError;
use serde::{Serialize, Deserialize};

pub struct IndexedDB {
    // Saves an index of the database in memory.
    // Improves performance of checking if things exist or not.
    // In the future i could also add search feature for this
    index_root: index::File,

    location: String,
}

impl IndexedDB {
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn exists(&self, name: &str) -> bool {
        false
    }
    pub fn save<T: Serialize>(&mut self,key: &str, data: T) -> Result<(), DBError> {
        Ok(())
    }
    pub fn load<T>(&self, identifier: &str) -> Result<T, DBError> where for<'de> T: Deserialize<'de> {
        Err(DBError::load(""))
    }

    pub fn new(location: &str) -> Self {
        IndexedDB{
            index_root: index::File::Folder(Box::new([])),
            location: String::from(location),
        }
    }
}

