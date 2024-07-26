use anyhow::Result;
use cargo_toml::{Lints, Manifest, Workspace};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, join};

use crate::{lint_item::LintItem, output::Output};

#[derive(Clone, Debug, PartialEq)]
pub struct CargoManifest(Manifest);

impl CargoManifest {
    pub fn load_workspace() -> Result<Self> {
        let path = crate::project_path::WorkspaceDir::workspace_manifest_path();

        let manifest: Manifest = Manifest::from_path(path)?;

        Ok(Self(manifest))
    }

    pub fn load_crate() -> Result<Self> {
        let path = crate::project_path::WorkspaceDir::crate_manifest_path();

        let manifest: Manifest = Manifest::from_path(path)?;

        Ok(Self(manifest))
    }

    pub const fn is_workspace(&self) -> bool {
        self.0.workspace.is_some()
    }

    pub const fn is_crate(&self) -> bool {
        self.0.package.is_some()
    }

    pub fn workspace_contains_lint(&self, lint: &LintItem) -> bool {
        match &self.0.workspace {
            Some(Workspace { lints: Some(lints), .. }) => lints
                .get("clippy")
                .map_or(false, |clippys| {
                    clippys.contains_key(lint.id()) || clippys.contains_key(lint.group())
                }),
            _ => false,
        }
    }

    pub fn workspace_lint_deprecated(&self, lint: &LintItem) -> bool {
        match &self.0.workspace {
            Some(Workspace { lints: Some(lints), .. }) => lints
                .get("clippy")
                .map_or(false, |clippys| {
                    clippys.contains_key(lint.id()) && lint.group() == "deprecated"
                }),
            _ => false,
        }
    }

    pub fn crate_lint_deprecated(&self, lint: &LintItem) -> bool {
        match &self.0.lints {
            Some(Lints { workspace, groups }) => {
                // if inherit workspace
                if *workspace {
                    self.workspace_contains_lint(lint)
                } else {
                    groups
                        .get("clippy")
                        .map_or(false, |lints| {
                            lints.contains_key(lint.id()) && lint.group() == "deprecated"
                        })
                }
            }
            _ => false,
        }
    }

    /// lint in group but set it again
    pub fn workspace_group_duplicate(&self, lint: &LintItem) -> bool {
        match &self.0.workspace {
            Some(Workspace { lints: Some(lints), .. }) => lints
                .get("clippy")
                .map_or(false, |clippys| {
                    clippys.contains_key(lint.group()) && clippys.contains_key(lint.id())
                }),
            _ => false,
        }
    }

    /// lint in group but set it again
    pub fn crate_group_duplicate(&self, lint: &LintItem) -> bool {
        match &self.0.lints {
            Some(Lints { workspace, groups }) => {
                // if inherit workspace
                if *workspace {
                    self.workspace_group_duplicate(lint)
                } else {
                    groups
                        .get("clippy")
                        .map_or(false, |lints| {
                            lints.contains_key(lint.group()) && lints.contains_key(lint.id())
                        })
                }
            }
            _ => false,
        }
    }

    pub fn crate_contains_lint(&self, lint: &LintItem) -> bool {
        match &self.0.lints {
            Some(Lints { workspace, groups }) => {
                // if inherit workspace
                if *workspace {
                    self.workspace_contains_lint(lint)
                } else {
                    groups
                        .get("clippy")
                        .map_or(false, |lints| {
                            lints.contains_key(lint.id()) || lints.contains_key(lint.group())
                        })
                }
            }
            _ => false,
        }
    }

    /// is crate lints rules inherit from the workspace
    pub const fn crate_inherit(&self) -> bool {
        match self.0.lints {
            Some(Lints { workspace, .. }) => workspace,
            _ => false,
        }
    }

    pub async fn tidy_workspace(&self, lints: &[LintItem], with_docs: bool) -> Result<()> {
        let options = opt();

        let (allow, unne, dup, dep) = (
            Output::worksapce_allow_path(),
            Output::workspace_unnecessary_path(),
            Output::workspace_duplicate_path(),
            Output::worksapce_deprecated_path(),
        );

        let (al_f, unne, dup, dep) = join!(
            options.open(allow),
            options.open(unne),
            options.open(dup),
            options.open(dep)
        );
        let (mut al_f, mut ne, mut dup, mut dep) = (al_f?, unne?, dup?, dep?);

        for lint in lints {
            if lint.is_allow() && !self.workspace_contains_lint(lint) {
                al_f.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
            if !lint.is_allow() && self.workspace_contains_lint(lint) {
                ne.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
            if self.workspace_group_duplicate(lint) {
                dup.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
            if self.workspace_lint_deprecated(lint) {
                dep.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn tidy_crate(&self, lints: &[LintItem], with_docs: bool) -> Result<()> {
        let options = opt();

        let (allow, unne, dup, dep) = (
            Output::crate_allow_path(),
            Output::crate_unnecessary_path(),
            Output::crate_duplicate_path(),
            Output::crate_deprecated_path(),
        );

        let (al_f, unne, dup, dep) = join!(
            options.open(allow),
            options.open(unne),
            options.open(dup),
            options.open(dep)
        );
        let (mut al_f, mut ne, mut dup, mut dep) = (al_f?, unne?, dup?, dep?);

        for lint in lints {
            if lint.is_allow() && !self.crate_contains_lint(lint) {
                al_f.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
            if !lint.is_allow() && self.crate_contains_lint(lint) {
                ne.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
            if self.crate_group_duplicate(lint) {
                dup.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
            if self.crate_lint_deprecated(lint) {
                dep.write_all(lint.show(with_docs).as_bytes())
                    .await?;
            }
        }

        Ok(())
    }
}

fn opt() -> OpenOptions {
    let mut open_options = OpenOptions::new();
    let options = open_options
        .create(true)
        .read(true)
        .write(true)
        .truncate(true);

    options.clone()
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_cargo_toml() {
        let a: cargo_toml::Manifest = cargo_toml::Manifest::from_path("./Cargo.toml").unwrap();
        let a = a.lints.unwrap();
        dbg!(a.groups.get("rust"));
        dbg!(a.groups.get("clippy"));
    }
}
