mod core;
pub(crate) use core::DbBackend;

#[cfg(test)]
mod test_suite;
#[cfg(test)]
pub(crate) use test_suite::DbBackendTestSuite;

pub(crate) use default_backend::DefaultDbBackend;
mod default_backend {
    use crate::*;

    #[cfg(feature = "surrealdb")]
    pub type DefaultDbBackend = SurrealBackend;
}

#[cfg(feature = "surrealdb")]
mod surrealdb;
#[cfg(feature = "surrealdb")]
pub(crate) use surrealdb::SurrealBackend;
