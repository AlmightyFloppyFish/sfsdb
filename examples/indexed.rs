use serde::{Deserialize, Serialize};
use std::ops::Sub;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MyIndex {
    pub categories: Vec<String>,
    pub starting_date: SystemTime,
}

/*
 * When using an indexed database you can dispatch search actions and get results.
 * Save and loads still work the same but to save with an index use the save_with_index()
 * method or use the add_index() method after saving normally
 */

fn main() {
    // Second parameter is maximum amount of cached objects
    let mut db = sfsdb::new_indexed("db");

    db.save_with_index(
        "justin",
        User {
            name: String::from("Justin Evens"),
            age: 22,
        },
        MyIndex {
            categories: vec!["employee".into(), "programmers".into()],
            starting_date: SystemTime::now(),
        },
    )
    .unwrap();
    db.save_with_index(
        "keth",
        User {
            name: String::from("Keth Stone"),
            age: 31,
        },
        MyIndex {
            categories: vec!["employee".into(), "support team".into()],
            starting_date: SystemTime::now().sub(Duration::from_secs(400)),
        },
    )
    .unwrap();

    // Of course this is only very basic queries but, since it's a closure you have the
    // power of the entire Rust programming language at your hands.

    let programmers = db.search_with(|index| index.categories.contains(&"programmers".to_owned()));
    println!("All programmers: {:?}", programmers);

    let recent =
        db.search_with(|index| index.starting_date.elapsed().unwrap() > Duration::from_secs(300));
    println!("These have existed for more than 300 seconds: {:?}", recent);
}
