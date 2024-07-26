use std::path::PathBuf;

use crate::project_path::WorkspaceDir;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Output;

impl Output {
    const WORKSPACE_ALLOW: &'static str = "target/workspace_allow.toml";
    const WORKSPACE_UNNECESSARY: &'static str = "target/workspace_unnecessary.toml";
    const WORKSPACE_DUPLICATE: &'static str = "target/workspace_duplicate.toml";
    const WORKSPACE_DEPRECATED: &'static str = "target/workspace_deprecated.toml";

    const CRATE_ALLOW: &'static str = "target/crate_allow.toml";
    const CRATE_UNNECESSARY: &'static str = "target/crate_unnecessary.toml";
    const CRATE_DUPLICATE: &'static str = "target/crate_duplicate.toml";
    const CRATE_DEPRECATED: &'static str = "target/crate_deprecated.toml";

    pub fn worksapce_allow_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::WORKSPACE_ALLOW)
    }

    pub fn worksapce_deprecated_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::WORKSPACE_DEPRECATED)
    }

    pub fn workspace_unnecessary_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::WORKSPACE_UNNECESSARY)
    }

    pub fn workspace_duplicate_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::WORKSPACE_DUPLICATE)
    }

    pub fn crate_allow_path() -> PathBuf {
        WorkspaceDir::crate_root().join(Self::CRATE_ALLOW)
    }
    pub fn crate_deprecated_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::CRATE_DEPRECATED)
    }

    pub fn crate_unnecessary_path() -> PathBuf {
        WorkspaceDir::crate_root().join(Self::CRATE_UNNECESSARY)
    }

    pub fn crate_duplicate_path() -> PathBuf {
        WorkspaceDir::crate_root().join(Self::CRATE_DUPLICATE)
    }
}
