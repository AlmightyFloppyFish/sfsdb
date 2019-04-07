# Simple File-System Database

If you're benchmarking remember to compile your project with `--release` flag!

### Todo
 * Improve caching performance (if possible)
 * CachedIndexedDB
 * Documentation
 * Upload crate

```rust
use sfsdb::GenericDatabase;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestData {
    pub first_entry: String,
    pub second_entry: u64,
}

fn main() {
    let mut db = sfsdb::new("db");
    let u = TestData{first_entry: "kjehfakwljhfwa eklfhawe fkwhaeflkawhfwaef".to_string(), second_entry: 48328414153};

    // Don't unwrap, remember to use proper error handling in actual applications
    db.save("some_key", u.clone()).unwrap();
    let retrieved = db.load::<TestData>("some_key").unwrap();

    assert_eq!(retrieved, u);
}
```
