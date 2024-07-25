use std::{ffi::OsStr, path::PathBuf, process::Command, str::FromStr, sync::LazyLock};

static WORKSPACE_MANIFEST: LazyLock<PathBuf> = LazyLock::new(|| {
    let output = cargo_command(["locate-project", "--workspace", "--message-format", "plain"]);

    let from_utf8_lossy = String::from_utf8_lossy(&output).to_string();
    let output = from_utf8_lossy.trim();

    let res = PathBuf::from_str(output).expect("build path failed");
    let target = res
        .parent()
        .expect("parent failed")
        .join("target");
    std::fs::create_dir_all(target).expect("create dir failed");

    res
});

static CRATE_MANIFEST: LazyLock<PathBuf> = LazyLock::new(|| {
    let output = cargo_command(["locate-project", "--message-format", "plain"]);

    let from_utf8_lossy = String::from_utf8_lossy(&output).to_string();
    let output = from_utf8_lossy.trim();

    let res = PathBuf::from_str(output).expect("build path failed");
    let target = res
        .parent()
        .expect("parent failed")
        .join("target");
    std::fs::create_dir_all(target).expect("create dir failed");

    res
});

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceDir;

impl WorkspaceDir {
    pub fn workspace_manifest_path() -> &'static PathBuf {
        &WORKSPACE_MANIFEST
    }

    pub fn crate_manifest_path() -> &'static PathBuf {
        &CRATE_MANIFEST
    }

    pub fn workspace_root() -> PathBuf {
        let manifest_path = Self::workspace_manifest_path();

        manifest_path
            .parent()
            .expect("get parent dir failed")
            .to_path_buf()
    }

    pub fn crate_root() -> PathBuf {
        let manifest_path = Self::crate_manifest_path();

        manifest_path
            .parent()
            .expect("get parent dir failed")
            .to_path_buf()
    }
}
