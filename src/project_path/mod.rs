use std::{ffi::OsStr, path::PathBuf, process::Command, str::FromStr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceDir;

impl WorkspaceDir {
    fn cargo_command<S, I>(args: I) -> Vec<u8>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new("cargo")
            .args(args)
            .output()
            .expect("execute cargo failed")
            .stdout
    }

    pub fn workspace_manifest_path() -> PathBuf {
        let output =
            Self::cargo_command(["locate-project", "--workspace", "--message-format", "plain"]);

        let from_utf8_lossy = String::from_utf8_lossy(&output).to_string();
        let output = from_utf8_lossy.trim();

        PathBuf::from_str(output).expect("build path failed")
    }

    pub fn workspace_root() -> PathBuf {
        let manifest_path = Self::workspace_manifest_path();

        manifest_path
            .parent()
            .expect("get parent dir failed")
            .to_path_buf()
    }

    pub fn crate_manifest_path() -> PathBuf {
        let output = Self::cargo_command(["locate-project", "--message-format", "plain"]);

        let from_utf8_lossy = String::from_utf8_lossy(&output).to_string();
        let output = from_utf8_lossy.trim();

        PathBuf::from_str(output).expect("build path failed")
    }

    pub fn crate_root() -> PathBuf {
        let manifest_path = Self::crate_manifest_path();

        manifest_path
            .parent()
            .expect("get parent dir failed")
            .to_path_buf()
    }
}
