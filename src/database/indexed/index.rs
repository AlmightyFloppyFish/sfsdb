use hashbrown::HashMap;

use crate::error::DBError;

pub struct FileData {
    name: String,
    location: String,
    last_modified: String,
}

pub struct Index<T>(pub HashMap<String, T>);

impl<T> Index<T> {
    pub fn new() -> Self {
        Index(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        self.0.get_mut(key)
    }

    // I should make the user just attach a closure were it returns true or false
    // through DOP, where I give the user the saved object as a parameter
    pub fn attach(&mut self, key: &str, index: T) {
        self.0.insert(key.to_owned(), index);
    }

    pub fn update<F>(&mut self, key: &str, mut apply: F) -> Result<(), DBError>
    where
        F: FnMut(&mut T),
    {
        match self.0.get_mut(key) {
            Some(mut index) => Ok(apply(&mut index)),
            None => return Err(DBError::index("")),
        }
    }

    pub fn delete(&mut self, key: &str) {
        self.0.remove(key);
    }
}
