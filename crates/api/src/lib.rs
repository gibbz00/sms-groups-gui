#![feature(lazy_cell)]

mod rest_service;
pub(crate) use rest_service::RestService;

mod rest_server;
pub use rest_server::RestServer;
