use serde::{Deserialize, Serialize};

pub(crate) mod cache;
pub mod database;
mod error;
mod filesystem;

use crate::error::DBError;
use database::{cached::CachedDB, indexed::IndexedDB, simple::SimpleDB};

pub trait GenericDatabase {
    fn location(&self) -> &str;
    fn exists(&self, name: &str) -> bool;
    fn save<T>(&mut self, key: &str, data: &T) -> Result<(), DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone;
    fn load<T>(&mut self, identifier: &str) -> Result<T, DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone;
    fn delete(&mut self, identifier: &str);
}

fn init(dir: &str) {
    std::fs::create_dir_all(dir).ok();
}

pub fn new(location: &str) -> SimpleDB {
    init(location);
    SimpleDB::new(location)
}

pub fn new_cached(location: &str, cache: Option<usize>, resync_every: u16) -> CachedDB {
    init(location);
    CachedDB::new(location, cache, resync_every)
}

pub fn new_indexed<I>(location: &str, cache: Option<usize>, resync_every: u16) -> IndexedDB<I>
where
    for<'de> I: Deserialize<'de> + Serialize + Clone,
{
    init(location);
    IndexedDB::new(location, cache, resync_every)
}
