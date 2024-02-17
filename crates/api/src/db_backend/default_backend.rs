use crate::*;

#[cfg(feature = "surrealdb")]
pub type DefaultDbBackend = SurrealBackend;
