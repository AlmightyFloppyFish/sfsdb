use crate::cache;
use crate::error::DBError;
use crate::filesystem::{fs_delete, fs_load, fs_save};
use crate::GenericDatabase;
use std::fs::read;

use bincode::deserialize;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

const CACHE_RESYNC_EVERY: u8 = 100;

pub struct CachedDB {
    location: String,
    cache_limit: Option<usize>,
    cache_timer: u8,
    cache_count: cache::CacheCount,
    cache: HashMap<String, Vec<u8>>, // Key -> bincode
}

impl GenericDatabase for CachedDB {
    fn location(&self) -> &str {
        &self.location
    }
    fn exists(&self, key: &str) -> bool {
        if self.cache_limit.is_some() {
            let mut p = PathBuf::new();
            p.push(&self.location);
            p.push(key);
            p.exists()
        } else {
            self.cache.contains_key(key)
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
        self.cache_count.add_tracker(k);
        Ok(())
    }
    fn load<T>(&mut self, key: &str) -> Result<T, DBError>
    where
        for<'de> T: Deserialize<'de> + Serialize + Clone,
    {
        // Perform resync once ever X amount of loads
        if self.cache_timer > CACHE_RESYNC_EVERY {
            self.cache_timer = 0;
            self.resync();
        }
        self.cache_timer += 1;
        self.cache_count.increase_use(key);
        match self.cache.get(key) {
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
        self.cache.remove(key);
        self.cache_count.del_tracker(key);

        let mut path = PathBuf::new();
        path.push(&self.location());
        path.push(key);
        fs_delete(&path);
    }
}

impl CachedDB {
    // Manually perform a resync of the cache. This will cache the top N most used keys.
    // This is already run automatically on a schedule.
    pub fn resync(&mut self) {
        let mut pairs = Vec::with_capacity(self.cache_count.0.len());
        let mut i: usize = 0;
        for (key, value) in &self.cache_count.0 {
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
            } else if self.cache_limit.is_some() {
                if c >= self.cache_limit.unwrap() {
                    break;
                };
            }

            should_exist.insert(k.clone(), ());
            let value_from_fs = if self.cache.contains_key(*k) {
                continue;
            } else {
                let mut path = PathBuf::new();
                path.push(&self.location);
                path.push(*k);
                read(path)
                    .map_err(|e| eprintln!("sfsdb: File and Cache mismatch ({}): {}", *k, e))
                    .unwrap_or_default()
            };
            self.cache.insert(k.clone().to_owned(), value_from_fs);
        }

        // Iter over keys and check if key is not in pairs then delete
        let mut in_cache: Vec<String> = Vec::new();
        for key in self.cache.keys() {
            in_cache.push(key.clone());
        }
        for key in in_cache {
            if !should_exist.contains_key(key.as_str()) {
                self.cache.remove(&key);
            }
        }
    }
    pub fn new(location: &str, cache_limit: Option<usize>) -> Self {
        CachedDB {
            location: String::from(location),
            cache_limit: cache_limit,
            cache_timer: 0,
            cache_count: cache::CacheCount::new(),
            cache: HashMap::new(),
        }
    }
}
