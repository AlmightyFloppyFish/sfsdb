use serde::{Deserialize, Serialize};
use sfsdb::GenericDatabase;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u64,
}

/*
 * A cached database can greatly increase performance when some objects
 * are loaded more than others.
 *
 * For cached database you really don't have to do anything differently.
 * It'll automatically cache the top Some(N) most used objects and pick
 * the cached version instead of it, if it exists when using db.load().
 *
 * It'll resync which objects are cached every 100th load, but you can also
 * force a resync manually throught the db.resync() method
 *
 * For performance comparison with an uncached database run the 'benchmark'
 * example. Just remember to use --release flag to cargo!
 */

fn main() {
    // Second parameter is maximum amount of cached objects
    let mut db = sfsdb::new_cached("db", Some(20));

    let u = User {
        simon: "Justin Evans".to_string(),
        age: 22,
    };

    db.save("some key", &u).unwrap();
    db.save("other key", &u).unwrap();

    assert_eq!(db.exists("some key"), true);
    assert_eq!(u, db.load::<User>("some key").unwrap());
    assert_eq!(u, db.load::<User>("other key").unwrap());
}
