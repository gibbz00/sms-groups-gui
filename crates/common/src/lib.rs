#![feature(lazy_cell)]
#![allow(async_fn_in_trait)]

mod db_backend;
pub(crate) use db_backend::*;
pub use db_backend::{DbBackend, DefaultDbBackend};

mod structs;
pub use structs::*;

mod config;
pub use config::*;

mod paths;
pub use paths::ProjectPaths;

mod instrumentation;
pub use instrumentation::Instrumentation;
pub(crate) use instrumentation::*;

#[cfg(feature = "test-utils")]
mod test_containers;
#[cfg(feature = "test-utils")]
pub use test_containers::TestContainer;

#[cfg(feature = "test-utils")]
mod test_connection;
#[cfg(feature = "test-utils")]
pub use test_connection::TestConnection;
