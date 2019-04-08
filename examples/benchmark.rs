use serde::{Deserialize, Serialize};
use sfsdb::GenericDatabase;
use std::fs;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u64,
}

pub struct MyIndex {
    pub login_attempts: u16,
    pub logged_in: bool,
}

/*
 * For an more in-depth explenation of the different kinds of databases
 * view their respective examples.
 */

fn main() {
    let mut db_uncached = sfsdb::new("db_simple");
    let mut db_indexed = sfsdb::new_indexed::<MyIndex>("db_indexed");
    let mut db_cached = sfsdb::new_cached("db_cached", Some(20));

    let justin = User {
        name: "Justin Evans".to_string(),
        age: 22,
    };
    let keth = User {
        name: "Keth Stone".to_string(),
        age: 31,
    };

    // Creating in an uncached database
    bench(&mut db_uncached, |db| {
        for i in 0..1000 {
            db.save(&i.to_string(), &justin);
        }
        print!("\n(Simple) Saving justin 1000 times took: ");
    });
    // Loading from an uncached database
    bench(&mut db_uncached, |db| {
        for _i in 0..1000 {
            // If I'd instead load i, so it loaded a different one every single time the
            // performance improvements of caching wouldn't show
            let justin = db.load::<User>("400").unwrap();
        }
        print!("(Simple) Loading justin (with key '400') 1000 times took: ");
    });

    // Creating in an cached database
    bench(&mut db_cached, |db| {
        for i in 0..1000 {
            db.save(&i.to_string(), &justin);
        }
        print!("\n(Cached) Saving justin 1000 times took: ");
    });
    // Loading from an cached database
    bench(&mut db_cached, |db| {
        for _i in 0..1000 {
            // If I'd instead load i, so it loaded a different one every single time the
            // performance improvements of caching wouldn't show
            let _justin = db.load::<User>("400").unwrap();
        }
        print!("(Cached) Loading justin (with key '400') 1000 times took: ");
    });

    // Creating in an indexed database
    bench(&mut db_indexed, |db| {
        for i in 0..1000 {
            // For this example, even numbers means he's logged in
            // For this example, key in database is the login attempts
            db.save_with_index(
                &i.to_string(),
                &justin,
                MyIndex {
                    login_attempts: i,
                    logged_in: (i % 2 == 0),
                },
            );
        }
        print!("\n(Indexed) Saving justin 1000 times took: ");
    });
    // Loading from an cached database
    bench(&mut db_indexed, |db| {
        for _i in 0..1000 {
            // If I'd instead load i, so it loaded a different one every single time the
            // performance improvements of caching wouldn't show
            let _justin = db.load::<User>("400").unwrap();
        }
        print!("(Indexed) Loading justin (with key '400') 1000 times took: ");
    });
    // Dispatching index query actions
    bench(&mut db_indexed, |db| {
        let logged_in_users = db.search_with(|i| i.logged_in);
        print!(
            "(Indexed) Querying for all logged-in users (which yielded {} results) took: ",
            logged_in_users.len()
        );
    });
    bench(&mut db_indexed, |db| {
        let locked_out_users = db.search_with(|i| i.login_attempts > 820);
        print!(
            "(Indexed) Querying for all locked-out users (which yielded {} results) took: ",
            locked_out_users.len()
        );
    });

    for db in &["db_simple", "db_cached", "db_indexed"] {
        fs::remove_dir_all(db).ok();
    }
}

fn bench<T, F>(db: &mut T, mut action: F)
where
    F: FnMut(&mut T),
{
    let t = SystemTime::now();
    action(db);
    print!("{:?}\n", t.elapsed().unwrap());
}
