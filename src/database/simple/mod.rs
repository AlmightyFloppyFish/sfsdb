use crate::filesystem::{fs_load, fs_save, fs_delete};
use serde::{Serialize, Deserialize};

use std::fs;
use std::path::{PathBuf, Path};
use crate::error::DBError;

pub struct SimpleDB {
    location: String,
}

impl SimpleDB {
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn exists(&self, identifier: &str) -> bool {
        let mut p = PathBuf::new();
        p.push(self.location());
        p.push(identifier);
        p.exists()
    }
    pub fn save<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), DBError> {
        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        fs_save(&path, &value)?;
        Ok(())
    }
    pub fn load<T>(&self, key: &str) -> Result<T, DBError> where for<'de> T: Deserialize<'de> {
        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        let v = fs_load::<T>(&path)?;
        Ok(v)
    }
    pub fn delete(&self, key: &str) {
        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        fs_delete(&path);
    }

    pub fn new(location: &str) -> Self {
        if !Path::new(location).exists() {
            if fs::create_dir(location).is_err() {
                panic!("sfsdb: Could not create database at {}", location)
            }
        }
        SimpleDB{
            location: String::from(location),
        }
    }
}
