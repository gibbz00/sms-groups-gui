use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

use crate::*;

/// Relative to repo root.
const CONFIG_PATH: &str = "sms_groups_config.yaml";

#[derive(Debug, Deserialize)]
pub struct SmsGroupsConfig {
    pub observability: InstrumentationConfig,
    pub api: ApiConfig,
}

impl SmsGroupsConfig {
    fn config_path() -> PathBuf {
        ProjectPaths::repo_root().join(CONFIG_PATH)
    }

    pub fn read() -> anyhow::Result<Self> {
        let path = Self::config_path();
        let config_string = std::fs::read_to_string(&path).with_context(|| format!("Failed to read config from {}", path.display()))?;
        serde_yaml::from_str(&config_string).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_config() {
        SmsGroupsConfig::read().unwrap();
    }
}
