use cargo_tidy_lints::project_path::WorkspaceDir;

#[test]
fn test_get_dir() {
    let a = WorkspaceDir::workspace_manifest_path();
    let cargo = a.file_name().unwrap();
    assert_eq!(cargo, "Cargo.toml");

    let a = WorkspaceDir::crate_manifest_path();
    let cargo = a.file_name().unwrap();
    assert_eq!(cargo, "Cargo.toml");
}
