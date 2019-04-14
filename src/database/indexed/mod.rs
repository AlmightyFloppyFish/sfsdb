mod index;

use crate::cache::Cache;
use crate::error::DBError;
use crate::filesystem::*;
use crate::GenericDatabase;
use bincode::deserialize;
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
    cache: Cache,
}

impl<I> GenericDatabase for IndexedDB<I>
where
    for<'de> I: Deserialize<'de> + Serialize + Clone,
{
    fn location(&self) -> &str {
        &self.location
    }
    fn exists(&self, key: &str) -> bool {
        if self.cache.limit.is_some() {
            let mut p = PathBuf::new();
            p.push(&self.location);
            p.push(key);
            p.exists()
        } else {
            self.cache.content.contains_key(key)
        }
    }
    fn save<T>(&mut self, key: &str, value: &T) -> Result<(), DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone,
    {
        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        fs_save(&path, &value)?;

        let k = key.to_owned();

        // Keep track of usage for syncing
        self.cache.add_tracker(k.clone());

        // Put everything on the cache untill it's full. And then start to resync from there
        if !self.cache.full {
            match self.cache.limit {
                Some(limit) => {
                    if self.cache.count.len() == limit {
                        self.cache.full = true
                    }
                }
                None => (),
            }
            self.cache.content.insert(
                k.clone(),
                match bincode::serialize(value) {
                    Ok(v) => v,
                    Err(_) => return Err(DBError::save(&format!("Unable to serialize {}", k))),
                },
            );
        }
        Ok(())
    }
    fn load<T>(&mut self, key: &str) -> Result<T, DBError>
    where
        for<'de> T: Deserialize<'de>,
    {
        // Perform resync once ever X amount of loads
        if self.cache.should_resync() {
            self.cache.timer = 0;
            // Prevent possible saving over index
            self.cache.full = true;

            self.resync();
        }
        self.cache.timer += 1;
        self.cache.increase_use(key);
        match self.cache.content.get(key) {
            None => {
                let mut path = PathBuf::new();
                path.push(&self.location());
                path.push(key);
                match fs_load::<T>(&path) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                }
            }
            Some(v) => Ok(match deserialize(v) {
                Ok(v) => v,
                Err(e) => return Err(DBError::load(&format!("Unable to decode {} ({})", key, e))),
            }),
        }
    }
    fn delete(&mut self, key: &str) {
        self.cache.content.remove(key);
        self.cache.del_tracker(key);

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
    // Manually perform a resync of the cache. This will cache the top N most used keys.
    // This is already run automatically on a schedule.
    pub fn resync(&mut self) {
        let mut pairs = Vec::with_capacity(self.cache.count.len());
        let mut i: usize = 0;
        for (key, value) in &self.cache.count {
            pairs.push((key, value));
            i += 1;
        }

        // Sort by one of tuple
        pairs.sort_by(|(_, a_v), (_, b_v)| b_v.cmp(a_v));

        let mut should_exist: HashMap<&str, ()> = HashMap::new();
        // Iter of those that are supposed to be in new cache
        for (c, (k, _)) in pairs.iter().enumerate() {
            // Stop conditionals
            if c > i {
                break;
            } else if self.cache.limit.is_some() {
                if c >= self.cache.limit.unwrap() {
                    break;
                };
            }

            should_exist.insert(k.clone(), ());
            let value_from_fs = if self.cache.content.contains_key(*k) {
                continue;
            } else {
                let mut path = PathBuf::new();
                path.push(&self.location);
                path.push(*k);
                fs::read(path)
                    .map_err(|e| eprintln!("sfsdb: File and Cache mismatch ({}): {}", *k, e))
                    .unwrap_or_default()
            };
            self.cache
                .content
                .insert(k.clone().to_owned(), value_from_fs);
        }

        // Iter over keys and check if key is not in pairs then delete
        let mut in_cache: Vec<String> = Vec::new();
        for key in self.cache.content.keys() {
            in_cache.push(key.clone());
        }
        for key in in_cache {
            if !should_exist.contains_key(key.as_str()) {
                self.cache.content.remove(&key);
            }
        }
    }

    pub fn save_with_index<T>(&mut self, key: &str, data: &T, index: I) -> Result<(), DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone,
    {
        self.save(key, data)?;
        self.index.disk_save(&index, key)?;
        Ok(self.index.attach(key, index))
    }

    pub fn add_index(&mut self, key: &str, index: I) -> Result<(), DBError> {
        if key == "" {
            return Err(DBError::index("Empty key"));
        }
        self.index.disk_save(&index, key)?;
        Ok(self.index.attach(key, index))
    }

    pub fn get_index(&self, key: &str) -> Option<&I> {
        self.index.get(key)
    }

    pub fn edit_index<F>(&mut self, key: &str, with: F) -> Result<(), DBError>
    where
        F: FnMut(I) -> I,
    {
        self.index.update::<F, I>(key, with)?;
        Ok(())
    }

    pub fn delete_index(&mut self, key: &str) {
        if key == "" {
            return;
        }
        self.index.disk_delete(key);
        self.index.delete(key);
    }

    pub fn search_with<F>(&self, apply: F) -> Vec<String>
    where
        F: Fn(&I) -> bool,
    {
        let mut results = Vec::new();
        for (k, v) in self.index.mem.iter() {
            if apply(&v) {
                results.push((*k).clone());
            };
        }
        return results;
    }

    pub fn new(location: &str, cache_limit: Option<usize>, resync_every: u16) -> Self {
        // Load existing fs index
        let mut index = index::Index::new(location);

        if !index.location.exists() {
            fs::create_dir_all(&index.location).unwrap();
        } else {
            // List all in directory
            // Index::disk_load(&self.location, K) for each
            for dir in fs::read_dir(&index.location).unwrap() {
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
            cache: Cache::new(cache_limit, resync_every),
            location: String::from(location),
        }
    }
}
