# node-dirty

## Purpose

Dirty Database is a tiny and fast key-value store with append-only disk log.
Ideal for apps with less than 1 million records.

## Installation

FIXME

## Why dirty?

This database is called dirty because:

* The file format is newline separated JSON
* Your database lives in the same process as your application, they share memory
* There is no query language, you just iterate through all records

So dirty means that you will hit a very hard wall with this database after
~1 million records, but it is a wonderful solution for anything smaller than
that.


## Tests

[![Build Status](https://travis-ci.org/havvy/dirty-db-rs.png)](https://travis-ci.org/havvy/dirty-db-rs)

```
git clone https://github.com/havvy/dirty-db-rs
cd dirty-db-rs
cargo test
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[Original JavaScript implementation](https://github.com/felixge/node-dirty) is
licensed under the MIT license by @felixge.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.