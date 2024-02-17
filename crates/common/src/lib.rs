#![feature(lazy_cell)]

mod config;
pub use config::SmsGroupsConfig;
pub(crate) use config::*;

mod paths;
pub use paths::ProjectPaths;
