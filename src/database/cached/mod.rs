use crate::filesystem::{fs_load, fs_save, fs_delete};
use crate::error::DBError;
use crate::GenericDatabase;

use serde::{Serialize, Deserialize};
use bincode::{deserialize};
use hashbrown::HashMap;

use std::path::PathBuf;

mod cache;

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
    fn save<T>(&mut self, key: &str, value: T) -> Result<(), DBError> 
        where for<'de> T: Deserialize<'de> + Serialize + Clone 
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
        where for<'de> T: Deserialize<'de> + Serialize + Clone 
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
                    Ok(v) => Ok(v)
                    ,
                    Err(e) => Err(e),
                }
            }
            Some(v) => Ok(match deserialize(v) {
                Ok(v) => v,
                Err(e) => {
                    return Err(DBError::load(&format!("Unable to decode {} ({})", key, e)))
                }
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
