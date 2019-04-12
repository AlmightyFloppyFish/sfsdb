extern crate bincode;

use crate::error::DBError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

pub fn fs_load<T>(p: &Path) -> Result<T, DBError>
where
    for<'de> T: Deserialize<'de>,
{
    let f = match File::open(p) {
        Ok(f) => f,
        Err(e) => return Err(DBError::load(&format!("{}", e))),
    };
    match bincode::deserialize_from(f) {
        Ok(d) => Ok(d),
        Err(e) => Err(DBError::load(&format!("{}", e))),
    }
}

pub fn fs_save<'e, T: Serialize>(p: &Path, data: &T) -> Result<(), DBError> {
    let f = match File::create(p) {
        Ok(f) => f,
        Err(_) => return Err(DBError::save(&format!("Could not create {}", p.display()))),
    };
    let r = bincode::serialize_into(f, data);
    if r.is_err() {
        return Err(DBError::save(&format!(
            "Could not encode data to {}",
            p.display()
        )));
    }
    Ok(())
}

pub fn fs_delete(p: &Path) {
    std::fs::remove_file(p)
        .map_err(|_| {
            eprintln!(
                "{}",
                DBError::delete(&format!("Could not delete {}", p.display()))
            )
        })
        .ok();
}
