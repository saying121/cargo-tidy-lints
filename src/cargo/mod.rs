use anyhow::Result;
use cargo_toml::{Lints, Manifest, Workspace};

#[derive(Clone, Debug, PartialEq)]
pub struct CargoManifest(Manifest);

impl CargoManifest {
    pub fn load() -> Result<Self> {
        let path = crate::project_path::WorkspaceDir::workspace_manifest_path();

        let manifest: Manifest = Manifest::from_path(path)?;

        Ok(Self(manifest))
    }

    pub const fn is_workspace(&self) -> bool {
        self.0.workspace.is_some()
    }

    pub const fn is_crate(&self) -> bool {
        self.0.package.is_some()
    }

    pub fn crate_locate() -> Result<Self> {
        let path = crate::project_path::WorkspaceDir::crate_manifest_path();

        let manifest: Manifest = Manifest::from_path(path)?;

        Ok(Self(manifest))
    }

    pub fn workspace_contains_lint(&self, lint: &str) -> bool {
        match &self.0.workspace {
            Some(Workspace { lints: Some(ls), .. }) => ls.contains_key(lint),
            _ => false,
        }
    }

    pub fn crate_contains_lint(&self, lint: &str) -> bool {
        match &self.0.lints {
            Some(Lints { workspace, groups }) => {
                if *workspace {
                    self.workspace_contains_lint(lint)
                } else {
                    groups.contains_key(lint)
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_cargo_toml() {
        let a: cargo_toml::Manifest = cargo_toml::Manifest::from_path("./Cargo.toml").unwrap();
        // a.lints.unwrap().workspace
    }
}
