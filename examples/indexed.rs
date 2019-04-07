use sfsdb::GenericDatabase;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestData {
    pub first_entry: String,
    pub second_entry: u64,
}

pub struct MyIndex {
    pub categories: Vec<String>,
    pub date: SystemTime,
}

/*
 * When using an indexed database you can dispatch search actions and get results.
 * Save and loads still work the same but to save with an index use the save_with_index()
 * method or use the add_index() method after saving normally
 */

fn main() {
    // Second parameter is maximum amount of cached objects
    let mut db = sfsdb::new_cached("db", Some(20));

    let u = TestData {
        first_entry: "some string".to_string(),
        second_entry: 1234,
    };

    db.save_with_index("some key", &u, MyIndex {}).unwrap();
    db.save_with_index("other key", &u).unwrap();

    assert_eq!(db.exists("some key"), true);
    assert_eq!(u, db.load::<TestData>("some key").unwrap());
    assert_eq!(u, db.load::<TestData>("other key").unwrap());
}
