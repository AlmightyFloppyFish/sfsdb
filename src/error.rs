use chrono::{Local, DateTime};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DBError {
    kind: String,
    cause: String,
    when: DateTime<Local>,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &format!("[{}] sfsdb {} error: {}", self.when.time().format("%H:%M:%S"), self.kind, self.cause)
        )
    }
}

impl DBError {
    fn new(kind: &str, cause: &str) -> Self {
        DBError {
            kind: kind.to_owned(),
            cause: cause.to_owned(),
            when: Local::now(),
        }
    }
    pub fn save(cause: &str) -> Self {
        Self::new("save", cause)
    }
    pub fn load(cause: &str) -> Self {
        Self::new("load", cause)
    }
    pub fn delete(cause: &str) -> Self {
        Self::new("delete", cause)
    }
}

impl Error for DBError {}
