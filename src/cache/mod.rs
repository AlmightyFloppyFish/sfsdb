use hashbrown::HashMap;

pub struct CacheCount(pub HashMap<String, u64>);

//pub fn pushbackCount(cache_count: &mut HashMap<String, u64>, )

impl CacheCount {
    pub fn increase_use(&mut self, identifier: &str) -> bool {
        match self.0.get_mut(identifier) {
            Some(c) => {
                if *c != std::u64::MAX {
                    *c += 1
                } else {
                    self.reset()
                }
            }
            None => return false,
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
