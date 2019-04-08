use serde::{Deserialize, Serialize};
use sfsdb::GenericDatabase;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u64,
}

fn main() {
    let mut db = sfsdb::new("db");

    let u = User {
        name: "Justin Evans".to_string(),
        age: 22,
    };

    // Remember to use proper error handling and not unwrap()
    // in a real world application.
    db.save("some key", &u).unwrap();
    db.save("other key", &u).unwrap();

    assert_eq!(db.exists("some key"), true);
    assert_eq!(u, db.load::<User>("some key").unwrap());
    assert_eq!(u, db.load::<User>("other key").unwrap());
}
