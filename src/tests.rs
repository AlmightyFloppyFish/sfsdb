use crate::GenericDatabase;
use std::time::{Duration, SystemTime};

#[test]
fn integrity() {
    let mut db = crate::new("test_db", None, false);
    db.save::<String>("some_key", "some_data".to_owned())
        .unwrap();

    let loaded = db.load::<String>("some_key").unwrap();

    assert_eq!(loaded, "some_data".to_owned());
}

#[test]
fn cache() {
    let mut db = crate::new("test_db", Some(6), false);
    let u: u64 = 41423141;

    let t = SystemTime::now();
    for i in 0..200 {
        db.save(&i.to_string(), u.clone()).unwrap();
    }
    println!("200 unique saves took: {:?}", t.elapsed().unwrap());

    spamload::<u64>(500, "3", &mut db);

    let t = SystemTime::now();
    spamload::<u64>(40, "3", &mut db);
    let cached = t.elapsed().unwrap();
    println!(
        "Loading \"3\" 40 times, (should be cached), took {:?}",
        cached
    );

    let t = SystemTime::now();
    spamload::<u64>(40, "120", &mut db);
    let uncached = t.elapsed().unwrap();
    println!("Loading \"120\" 40 times, took {:?}", uncached);

    // Cached version should be faster
    assert!(uncached.as_nanos() > cached.as_nanos());
}

fn spamload<T>(times: usize, key: &str, db: &mut crate::Database)
where
    T: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone,
{
    for i in 0..times {
        let a = db.load::<T>(key).map_err(|e| eprintln!("{}", e)).unwrap();
    }
}
