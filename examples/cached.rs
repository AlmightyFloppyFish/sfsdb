use sfsdb::GenericDatabase;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestData {
    pub first_entry: String,
    pub second_entry: u64,
}

/*
 * For cached database you really don't have to do anything differently.
 * It'll automatically cache the top Some(N) most used objects and pick
 * the cached version instead of it exists when using db.load().
 *
 * It'll resync which objects are cached every 100th load, but you can also
 * force a resync manually throught the db.resync() method
 */

fn main() {
    // Second parameter is maximum amount of cached objects
    let mut db = sfsdb::new_cached("db", Some(20));

    let u = TestData {
        first_entry: "some string".to_string(),
        second_entry: 1234,
    };

    db.save("some key", &u).unwrap();
    db.save("other key", &u).unwrap();

    assert_eq!(db.exists("some key"), true);
    assert_eq!(u, db.load::<TestData>("some key").unwrap());
    assert_eq!(u, db.load::<TestData>("other key").unwrap());
}
