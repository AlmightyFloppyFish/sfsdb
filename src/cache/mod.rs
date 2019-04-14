use hashbrown::HashMap;

pub struct Cache {
    pub count: HashMap<String, u64>,
    pub when_to_sync: u16,
    pub timer: u16,
    pub limit: Option<usize>,
    pub full: bool,
    pub content: HashMap<String, Vec<u8>>,
}

//pub struct CacheCount(pub HashMap<String, u64>);

//pub fn pushbackCount(cache_count: &mut HashMap<String, u64>, )

impl Cache {
    pub fn increase_use(&mut self, identifier: &str) -> bool {
        match self.count.get_mut(identifier) {
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
        self.count.insert(key, 0);
    }

    pub fn del_tracker(&mut self, key: &str) {
        self.count.remove(key);
    }

    pub fn should_resync(&self) -> bool {
        self.timer > self.when_to_sync
    }

    // This prevents integer overflows by pushing back all values, maintaining the percentual
    // difference between the counts.
    pub fn reset(&mut self) {
        for count in self.count.values_mut() {
            *count = *count / 5;
        }
    }

    pub fn new(limit: Option<usize>, resync_every: u16) -> Self {
        Cache {
            count: HashMap::new(),
            limit: limit,
            when_to_sync: resync_every,
            full: false,
            timer: 0,
            content: HashMap::new(),
        }
    }
}
