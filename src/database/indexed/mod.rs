mod index;

use crate::cache;
use crate::error::DBError;
use crate::filesystem::*;
use crate::GenericDatabase;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Index will always be loaded in ram
// but i might want to add a flag to make
// it purely disk-saved
pub struct IndexedDB<T>
where
    for<'de> T: Deserialize<'de> + Serialize + Clone,
{
    location: String,
    index: index::Index<T>,
    cache_limit: Option<usize>,
    cache_timer: u8,
    cache_count: cache::CacheCount,
    cache: HashMap<String, Vec<u8>>, // Key -> bincode
}

impl<I> GenericDatabase for IndexedDB<I>
where
    for<'de> I: Deserialize<'de> + Serialize + Clone,
{
    fn location(&self) -> &str {
        &self.location
    }
    fn exists(&self, key: &str) -> bool {
        let mut p = PathBuf::new();
        p.push(&self.location);
        p.push(key);
        p.exists()
    }
    fn save<T>(&mut self, key: &str, value: &T) -> Result<(), DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone,
    {
        let mut path = PathBuf::new();
        path.push(&self.location);
        path.push(key);
        fs_save(&path, &value)?;
        Ok(())
    }
    fn load<T>(&mut self, key: &str) -> Result<T, DBError>
    where
        for<'de> T: Deserialize<'de>,
    {
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

impl<I> IndexedDB<I>
where
    for<'de> I: Deserialize<'de> + Serialize + Clone,
{
    pub fn save_with_index<T>(&mut self, key: &str, data: &T, index: I) -> Result<(), DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone,
    {
        self.save(key, data)?;
        index::Index::disk_save(&index, &self.location, key)?;
        Ok(self.index.attach(key, index))
    }

    pub fn add_index(&mut self, key: &str, index: I) -> Result<(), DBError> {
        index::Index::disk_save(&index, &self.location, key)?;
        Ok(self.index.attach(key, index))
    }

    pub fn get_index(&self, key: &str) -> Option<&I> {
        self.index.get(key)
    }

    pub fn edit_index<F>(&mut self, key: &str, with: F) -> Result<(), DBError>
    where
        F: FnMut(I) -> I,
    {
        self.index.update::<F, I>(key, &self.location, with)?;
        Ok(())
    }

    pub fn delete_index(&mut self, key: &str) {
        index::disk_delete(&self.location, key);
        self.index.delete(key);
    }

    pub fn search_with<F>(&self, apply: F) -> Vec<String>
    where
        F: Fn(&I) -> bool,
    {
        let mut results = Vec::new();
        for (k, v) in self.index.0.iter() {
            if apply(&v) {
                results.push((*k).clone());
            };
        }
        return results;
    }

    pub fn new(location: &str, cache: Option<usize>) -> Self {
        // Load existing fs index
        let mut index = index::Index::new();
        let mut index_path = std::path::PathBuf::new();
        index_path.push(location);
        index_path.push("__INDEX__");

        if !index_path.exists() {
            fs::create_dir_all(&index_path).unwrap();
        } else {
            // List all in directory
            // Index::disk_load(&self.location, K) for each
            for dir in fs::read_dir(index_path).unwrap() {
                let p = dir.unwrap();
                let v = match fs_load(&p.path()) {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Skipping invalid index {:?}", &p.path());
                        continue;
                    }
                };
                index.attach(p.file_name().to_str().unwrap(), v)
            }
        }

        IndexedDB {
            index: index,
            cache_limit: cache,
            cache_timer: 0,
            cache_count: cache::CacheCount::new(),
            cache: HashMap::new(),
            location: String::from(location),
        }
    }
}
