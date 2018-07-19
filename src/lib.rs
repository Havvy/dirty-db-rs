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

#[cfg(test)]
extern crate tempfile;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Result as IoResult, BufReader, BufRead, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use serde_json::Value as Json;

pub mod empty_sink {
    use std::io;

    pub struct EmptySink {
        empty: io::Empty,
        sink: io::Sink
    }

    impl io::Read for EmptySink {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.empty.read(buffer)
        }
    }

    impl io::Write for EmptySink {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            self.sink.write(buffer)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.sink.flush()
        }
    }

    impl Default for EmptySink {
        fn default() -> EmptySink {
            EmptySink {
                empty: io::empty(),
                sink: io::sink()
            }
        }
    }

    impl EmptySink {
        pub fn new() -> EmptySink {
            Default::default()
        }
    }
}

pub struct DirtyDb<RW> where RW: Read + Write {
    docs: HashMap<String, Json>,
    writer: BufWriter<RW>,
}

#[derive(Serialize, Deserialize)]
struct Row {
    key: String,
    value: Option<Json>
}

impl DirtyDb<File> {
    // Open a file at the specified path and interpret it as a DirtyDB.
    //
    // If there is no file at the specified path, this function creates a new file.
    pub fn open<P>(path: P) -> IoResult<DirtyDb<File>>
    where P: AsRef<Path> + Into<PathBuf>
    {
        let file = std::fs::OpenOptions::new().read(true).append(true).create(true).open(&path)?;
        DirtyDb::new(file).into()
    }

    // Opens a DirtyDB from an already open File handle.
    //
    // The database gets read from the beginning of the file.
    pub fn from_file(mut file: File) -> IoResult<DirtyDb<File>> {
        use std::io::{Seek, SeekFrom};
        file.seek(SeekFrom::Start(0))?;
        DirtyDb::new(file)
    }
}

impl DirtyDb<empty_sink::EmptySink> {
    pub fn in_memory() -> DirtyDb<empty_sink::EmptySink> {
        DirtyDb::new(Default::default()).expect("EmptySink doesn't perform IO so cannot fail.")
    }
}

impl<RW> DirtyDb<RW> where RW: Read + Write, for<'a> &'a mut RW: Read {
    /// Opens a DirtyDB from an arbitrary type that implements Read and Write.
    pub fn new(mut buffer: RW) -> IoResult<DirtyDb<RW>> {
        let mut docs = HashMap::new();

        for line in BufReader::new(&mut buffer).lines() {
            let row: Row = serde_json::from_str(&line?)?;

            match row.value {
                None => {
                    docs.remove(&row.key);
                },

                Some(value) => {
                    docs.insert(row.key, value);
                }
            }
        }

        let writer = BufWriter::new(buffer);

        Ok(DirtyDb { docs, writer })
    }

    /// Insert a new key/value pair into the database.
    pub fn insert(&mut self, key: String, value: Json) {
        self.docs.insert(key.clone(), value.clone());
        self.write(Row{key, value: Some(value)});
    }

    /// Get a specific value from the database.
    pub fn get(&self, key: &str) -> Option<&Json> {
        self.docs.get(key)
    }

    /// Remove a specific value from the database.
    pub fn remove(&mut self, key: &str) {
        self.docs.remove(key);
        self.write(Row{key: key.to_string(), value: None});
    }

    /// Explicitly closes the database. Same as dropping.
    pub fn close(self) {}

    fn write(&mut self, row: Row) {
        serde_json::to_writer(&mut self.writer, &row).expect("Dirty DB's Row Serialize impl cannot fail.");
    }

    /// Access to the underlying HashMap.
    pub fn as_hashmap(&self) -> &HashMap<String, Json> {
        &self.docs
    }

    /// Number of non-deleted elements in the database.
    pub fn len(&self) -> usize {
        self.docs.len()
    }

    // pub fn keys(&self) -> impl Iterator<Item=&String> {
    //     self.docs.keys()
    // }
}

#[cfg(test)]
mod tests {
    use ::DirtyDb;

    use std::fs::File;
    use std::io::{self, Read, Seek, SeekFrom};
    use std::path::PathBuf;

    fn database_location(loc: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(file!());  // $proj/src/lib.rs
        path.pop(); // $proj/src/
        path.pop(); // $proj/
        path.push("databases"); // $proj/databases/
        path.push(loc); //$proj/databases/$loc
        path
    }

    fn temp_file_copy(loc: &str) -> File {
        let mut original = File::open(database_location(loc)).unwrap();
        let mut copy = ::tempfile::tempfile().unwrap();
        io::copy(&mut original, &mut copy).unwrap();
        copy
    }

    #[test]
    fn read_example_success() {
        let db = DirtyDb::from_file(temp_file_copy("example-success.db")).unwrap();
        let value = db.get("example").unwrap();
        assert_eq!(&json!("success"), value);
    }

    #[test]
    fn read_example_success_rewritten() {
        let db = DirtyDb::from_file(temp_file_copy("example-success-rewritten.db")).unwrap();
        let value = db.get("example").unwrap();
        assert_eq!(&json!("success"), value);
    }

    #[test]
    fn read_example_deleted() {
        let db = DirtyDb::from_file(temp_file_copy("example-deleted.db")).unwrap();
        assert_eq!(None, db.get("example"));
    }

    #[test]
    fn write_example_success() {
        let mut db_file = ::tempfile::tempfile().unwrap();
        let mut db = DirtyDb::from_file(db_file.try_clone().unwrap()).unwrap();
        db.insert("example".to_string(), json!("success"));
        db.close();
        db_file.seek(SeekFrom::Start(0)).unwrap();
        let mut db_string = String::new();
        db_file.read_to_string(&mut db_string).unwrap();
        assert_eq!(r#"{"key":"example","value":"success"}"#, &db_string);
    }
}
