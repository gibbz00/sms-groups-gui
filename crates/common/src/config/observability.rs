use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ObservabilityConfig {
    pub log_dir: PathBuf,
}
