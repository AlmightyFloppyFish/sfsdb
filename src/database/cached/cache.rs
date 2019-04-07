use hashbrown::HashMap;
use std::path::PathBuf;
use std::fs::read;

pub struct CacheCount(HashMap<String, u64>);

//pub fn pushbackCount(cache_count: &mut HashMap<String, u64>, )

impl CacheCount {
    pub fn increase_use(&mut self, identifier: &str) -> bool {
        match self.0.get_mut(identifier) {
            Some(c) => 
                if *c != std::u64::MAX { *c += 1 } 
                else { self.reset() },
            None => return false
        }
        true
    }

    pub fn add_tracker(&mut self, key: String) {
        self.0.insert(key, 0);
    }

    pub fn del_tracker(&mut self, key: &str) {
        self.0.remove(key);
    }

    // This prevents integer overflows by pushing back all values, maintaining the percentual
    // difference between the counts.
    pub fn reset(&mut self) {
        for count in self.0.values_mut() {
            *count = *count / 5;
        }
    }

    pub fn new() -> Self {
        CacheCount(HashMap::new())
    }
}

impl crate::database::cached::CachedDB {
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
                break
            } else if self.cache_limit.is_some() {
                if c >= self.cache_limit.unwrap() { break };
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
}
