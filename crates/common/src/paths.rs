use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub struct ProjectPaths;

impl ProjectPaths {
    pub fn crate_root() -> &'static Path {
        Path::new(CRATE_ROOT)
    }
    pub fn repo_root() -> &'static Path {
        REPO_ROOT.as_ref()
    }
}

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
// WORKAROUND: until https://github.com/rust-lang/cargo/issues/3946 gets resolved.
static REPO_ROOT: LazyLock<PathBuf> = LazyLock::new(|| Path::new(CRATE_ROOT).parent().expect("/crates").parent().expect("/").to_path_buf());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_repo_root() {
        let manifest_path_str = std::env::var("CARGO_MANIFEST_DIR").expect("Set when running Cargo");
        let diff = pathdiff::diff_paths(manifest_path_str, REPO_ROOT.as_path()).unwrap();
        assert_eq!("crates/common", diff.to_str().unwrap());
    }
}
