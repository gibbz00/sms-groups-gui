mod core;
pub use core::DbBackend;

mod document;
pub(crate) use document::DbDocument;

mod default_backend;
pub use default_backend::DefaultDbBackend;

#[cfg(feature = "surrealdb")]
mod surrealdb;
#[cfg(feature = "surrealdb")]
pub(crate) use surrealdb::SurrealBackend;

#[cfg(test)]
mod test_suite;
#[cfg(test)]
pub(crate) use test_suite::DbBackendTestSuite;
