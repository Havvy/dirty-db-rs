// Copyright 2018 The Dirty Database Rust Implementation Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::{Path, PathBuf};

// FIXME: Use actual JSON type.
type Json = ();

pub struct DirtyDb {
    path: PathBuf
}

impl DirtyDb {
    pub fn new<P>(path: P) -> Result<DirtyDb, ()> where P: AsRef<Path> {
        unimplemented!();
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    // The state of the database for further accesses is updated instantly while
    // the change will be written to disk shortly thereafter.
    pub fn set(&mut self, key: String, value: Json) {
        unimplemented!();
    }

    pub fn get(&self, key: String) -> Option<Json> {
        unimplemented!();
    }

    pub fn remove(&self, key: String) {
        unimplemented!();
    }

    // Explicitly closes the database. Same as dropping.
    pub fn close(self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
