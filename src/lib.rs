use serde::{Deserialize, Serialize};

pub(crate) mod cache;
pub mod database;
mod error;
mod filesystem;

use crate::error::DBError;
use database::{cached::CachedDB, indexed::IndexedDB, simple::SimpleDB};

/// All databases implement this trait.
pub trait GenericDatabase {
    /// Get the filepath of the database.
    fn location(&self) -> &str;
    /// Check if a key exists in the database.
    fn exists(&self, key: &str) -> bool;
    /// Save a value of type T to the database.
    fn save<T>(&mut self, key: &str, data: &T) -> Result<(), DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone;
    /// Load a value of type T from the database.
    fn load<T>(&mut self, identifier: &str) -> Result<T, DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone;
    /// Remove a key/value from the database.
    fn delete(&mut self, identifier: &str);
}

fn init(dir: &str) {
    std::fs::create_dir_all(dir).ok();
}

/// A simple purely file-system database.
pub fn new(location: &str) -> SimpleDB {
    init(location);
    SimpleDB::new(location)
}

/// A cached database. Used the exact same way as a simple but automaticaly caches the top most
/// used key/value's for faster read access.
pub fn new_cached(location: &str, cache: Option<usize>, resync_every: u16) -> CachedDB {
    init(location);
    CachedDB::new(location, cache, resync_every)
}

/// An indexed+cached database which allows you to bundle any struct along with your data, and
/// then later query it through closures.
pub fn new_indexed<I>(location: &str, cache: Option<usize>, resync_every: u16) -> IndexedDB<I>
where
    for<'de> I: Deserialize<'de> + Serialize + Clone,
{
    init(location);
    IndexedDB::new(location, cache, resync_every)
}
