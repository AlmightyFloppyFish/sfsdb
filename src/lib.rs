use serde::{Serialize, Deserialize};

pub mod error;
mod database;

#[cfg(test)]
mod tests;
mod filesystem;

use database::{simple::SimpleDB, cached::CachedDB};
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

pub enum Database {
    Simple(SimpleDB),
    Cached(CachedDB),
    //Indexed(IndexedDB),
    //IndexedCached(IndexedCachedDB),
    //Async(AsyncDB) // Doesn't halt program after calls
}

use Database::*;

impl GenericDatabase for Database {
    fn location(&self) -> &str {
        match self {
            Simple(db) => db.location(),
            Cached(db) => db.location(),
        }
    }
    fn exists(&self, name: &str) -> bool {
        match self {
            Simple(db) => db.exists(name),
            Cached(db) => db.exists(name),
        }
    }
    fn save<T>(&mut self, key: &str, data: T) -> Result<(), DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone 
    {
        match self {
            Simple(db) => db.save(key, data),
            Cached(db) => db.save(key, data),
        }
    }
    fn load<T>(&mut self, identifier: &str) -> Result<T, DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone 
    {
        match self {
            Simple(db) => db.load(identifier),
            Cached(db) => db.load(identifier),
        }
    }
    fn delete(&mut self, identifier: &str) {
        match self {
            Simple(db) => db.delete(identifier),
            Cached(db) => db.delete(identifier),
        }
    }
}

// Wrapper function to decide which database implementation to use
pub fn new(location: &str, cached: Option<usize>, indexed: bool) -> Database {
    if indexed {
        panic!("Indexing not implemented");
    } else if cached.is_some() {
        Database::Cached(CachedDB::new(location, if cached.unwrap() == 0 { None } else { cached }))
    } else {
        Database::Simple(SimpleDB::new(location))
    }
}
