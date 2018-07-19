// Copyright 2018 The Dirty Database Rust Implementation Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Result as IoResult, BufReader, BufRead, BufWriter};
use std::path::{Path, PathBuf};

use serde_json::Value as Json;

pub struct DirtyDb {
    path: PathBuf,
    docs: HashMap<String, Json>,
    database_file: BufWriter<File>,
}

#[derive(Serialize, Deserialize)]
struct Row {
    key: String,
    value: Option<Json>
}

impl DirtyDb {
    pub fn new<P>(path: P) -> IoResult<DirtyDb>
    where P: AsRef<Path> + Into<PathBuf> + Clone
    {
        let mut docs = HashMap::new();
        let database_file = std::fs::OpenOptions::new().read(true).append(true).create(true).open(&path)?;

        for line in BufReader::new(&database_file).lines() {
            let row: Row = serde_json::from_str(&line?)?;
            let key = row.key;

            match row.value {
                None => {
                    docs.remove(&key);
                },

                Some(value) => {
                    docs.insert(key, value);
                }
            }
        }

        let database_file = BufWriter::new(database_file);

        Ok(DirtyDb { path: path.into(), docs, database_file })
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn insert(&mut self, key: String, value: Json) {
        self.docs.insert(key.clone(), value.clone());
        self.write(Row{key, value: Some(value)});
    }

    pub fn get(&self, key: &str) -> Option<&Json> {
        self.docs.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.docs.remove(key);
        self.write(Row{key: key.to_string(), value: None});
    }

    // Explicitly closes the database. Same as dropping.
    pub fn close(self) {}

    fn write(&mut self, row: Row) {
        serde_json::to_writer(&mut self.database_file, &row).expect("Dirty DB's Row Serialize impl cannot fail.");
    }
}

#[cfg(test)]
mod tests {
    use ::DirtyDb;

    use std::path::PathBuf;

    fn database_location(loc: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(file!());  // proj/src/lib.rs
        path.pop(); // proj/src/
        path.pop(); // proj/
        path.push("databases");
        path.push(loc);
        path
    }

    #[test]
    fn read_example_success() {
        let db = DirtyDb::new(database_location("example-success.db")).unwrap();
        let value = db.get("example").unwrap();
        assert_eq!(&json!("success"), value);
    }

    #[test]
    fn read_example_success_rewritten() {
        let db = DirtyDb::new(database_location("example-success-rewritten.db")).unwrap();
        let value = db.get("example").unwrap();
        assert_eq!(&json!("success"), value);
    }

    #[test]
    fn read_example_deleted() {
        let db = DirtyDb::new(database_location("example-deleted.db")).unwrap();
        assert_eq!(None, db.get("example"));
    }
}
