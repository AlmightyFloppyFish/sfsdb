use sfsdb::GenericDatabase;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TestData {
    pub first_entry: String,
    pub second_entry: u64,
}

fn main() {
    let mut db = sfsdb::new("db", Some(6), false); // TODO: Some(4) actually means Some(5). Why? dunno.
    let u = TestData{first_entry: "kjehfakwljhfwa eklfhawe fkwhaeflkawhfwaef".to_string(), second_entry: 48328414153};

    // To make CPU chill after compile for more accurate benchmark
    std::thread::sleep(Duration::from_secs(1));

    let t = SystemTime::now();
    for i in  0..200 {
        db.save(&i.to_string(), u.clone()).unwrap();
    }
    println!("200 unique saves took: {:?}", t.elapsed().unwrap());

    spamload(100, "1", &mut db);
    spamload(200, "2", &mut db);
    spamload(300, "3", &mut db);
    spamload(400, "4", &mut db);
    spamload(500, "5", &mut db);

    std::thread::sleep(Duration::from_secs(1));
    let t = SystemTime::now();
    spamload(40, "3", &mut db);
    println!("Loading \"3\" 40 times, which has been loaded many times before, and is therefore automatically cached, took {:?}", t.elapsed().unwrap());


    std::thread::sleep(Duration::from_secs(1));
    let t = SystemTime::now();
    spamload(40, "120", &mut db);
    println!("Loading \"120\" 40 times, took {:?}", t.elapsed().unwrap());
    
}

fn spamload(times: usize, key: &str, db: &mut sfsdb::Database) {
    for i in 0..times {
        let a = db.load::<TestData>(key).unwrap();
    };
}
