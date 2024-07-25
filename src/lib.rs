use std::{path::PathBuf, process::Command, str::FromStr};

pub mod get_lints;
pub mod sorting_lints;
pub mod cargo;

fn workspace_root() -> PathBuf {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()
        .expect("execute cargo failed")
        .stdout;
    let output = String::from_utf8_lossy(&output);

    let cargo_path = &PathBuf::from_str(&output).expect("build path failed");
    cargo_path
        .parent()
        .expect("get parent dir failed")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        let res = workspace_root();
        dbg!(res);
    }
}
