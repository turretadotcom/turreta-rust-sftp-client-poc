//! Types and functions for the essential domains that closely resembles the database tables they
//! represent.
//!
//!
//!
//! * PostgreSQL version 9.5 or newer
//! * Sqlite3 version 3.24.0 or newer
//!
//! See [the methods on `InsertStatement`](crate::query_builder::InsertStatement#impl-2)
//! for usage examples.
//!
//! Constructing an upsert statement from an existing select statement
//! requires a where clause on sqlite due to a ambiguity in their
//! parser. See [the corresponding documentation](https://www.sqlite.org/lang_UPSERT.html)
//! for details.
pub mod alert;
