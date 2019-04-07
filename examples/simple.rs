use sfsdb::GenericDatabase;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestData {
    pub first_entry: String,
    pub second_entry: u64,
}

#[derive(Debug)]
pub struct Index {
    pub name: String,
    pub id: u32,
}

fn main() {
    let mut db = sfsdb::new_cached("db", Some(30)); // TODO: None should be infinite caching
    let u = TestData{first_entry: "kjehfakwljhfwa eklfhawe fkwhaeflkawhfwaef".to_string(), second_entry: 48328414153};

    // To make CPU chill after compile for more accurate benchmark
    std::thread::sleep(Duration::from_secs(1));

    let t = SystemTime::now();
    for i in  0..200 {
        //db.save_with_index(&i.to_string(), u.clone(), Index { name: "all are this".to_owned(), id: i }).unwrap();
        db.save(&i.to_string(), u.clone()).unwrap();
    }
    println!("200 unique saves took: {:?}", t.elapsed().unwrap());

    std::thread::sleep(Duration::from_secs(1));
    
    let t = SystemTime::now();
    spamload(500, "5", &mut db);
    println!("500 loads took {:?}", t.elapsed().unwrap());

    println!("Integrity check: {:?}\n", db.load::<TestData>("5").unwrap());

    //let t = SystemTime::now();
    //println!("{:?}", db.search_with(|index| index.id == 30));
    //println!("Index search took {:?}", t.elapsed().unwrap());
}

#[no_mangle]
fn spamload<D: sfsdb::GenericDatabase>(times: usize, key: &str, db: &mut D) {
    for i in 0..times {
        let a = db.load::<TestData>(key).unwrap();
    };
}
