mod index;

use serde::{Serialize, Deserialize};
use crate::error::DBError;
use crate::GenericDatabase;
use crate::filesystem::*;
use std::path::PathBuf;

pub struct IndexedDB<T> {
    index: index::Index<T>,
    location: String,
}

impl <I>GenericDatabase for IndexedDB<I> {
    fn location(&self) -> &str {
        &self.location
    }
    fn exists(&self, key: &str) -> bool {
        let mut p = PathBuf::new();
        p.push(&self.location);
        p.push(key);
        p.exists()
    }
    fn save<T>(&mut self, key: &str, value: T) -> Result<(), DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone
    {
        let mut path = PathBuf::new();
        path.push(&self.location);
        path.push(key);
        fs_save(&path, &value)?;
        Ok(())
    }
    fn load<T>(&mut self, key: &str) -> Result<T, DBError> where for<'de> T: Deserialize<'de> {
        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        let v = fs_load::<T>(&path)?;
        Ok(v)
    }
    fn delete(&mut self, key: &str) {
        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        fs_delete(&path);
        self.index.delete(key);
    }
}

impl <I>IndexedDB<I> {
    pub fn save_with_index<T>(&mut self, key: &str, data: T, index: I) -> Result<(), DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone
    {
        self.save(key, data)?;
        Ok(self.index.attach(key, index))
    }

    pub fn add_index(&mut self, key: &str, index: I) {
        self.index.attach(key, index)
    }

    pub fn get_index(&self, key: &str) -> Option<&I> {
        self.index.get(key)
    }

    pub fn del_index(&mut self, key: &str) {
        self.index.delete(key);
    }

    pub fn search_with<F>(&self, apply: F) -> Vec<String> 
        where F: Fn(&I) -> bool 
    {
        let mut results = Vec::new();
        for (k, v) in self.index.0.iter() {
            if apply(&v) {
                results.push((*k).clone());
            };
        };
        return results
    }

    pub fn new(location: &str) -> Self {
        IndexedDB{
            index: index::Index::new(),
            location: String::from(location),
        }
    }
}

