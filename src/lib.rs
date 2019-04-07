use serde::{Serialize, Deserialize};

mod error;
pub mod database;
mod filesystem;

#[cfg(test)]
mod tests;

use database::{simple::SimpleDB, cached::CachedDB, indexed::IndexedDB};
use crate::error::DBError;

pub trait GenericDatabase {
    fn location(&self) -> &str;
    fn exists(&self, name: &str) -> bool;
    fn save<T>(&mut self, key: &str, data: T) -> Result<(), DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone;
    fn load<T>(&mut self, identifier: &str) -> Result<T, DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone;
    fn delete(&mut self, identifier: &str);
}

pub fn new(location: &str) -> SimpleDB {
    SimpleDB::new(location)
}

pub fn new_cached(location: &str, cache: Option<usize>) -> CachedDB {
    CachedDB::new(location, cache)
}

pub fn new_indexed<I>(location: &str) -> IndexedDB<I> {
    IndexedDB::new(location)
}
