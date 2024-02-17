use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstrumentationConfig {
    pub log_dir: PathBuf,
}
