use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, join, task::JoinHandle};

use crate::{cargo::CargoManifest, get_lints::LintItem, project_path::WorkspaceDir};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Output;

impl Output {
    const WORKSPACE_ALLOW: &'static str = "target/workspace_allow.toml";
    const WORKSPACE_UNNECESSARY: &'static str = "target/workspace_unnecessary.toml";

    const CRATE_ALLOW: &'static str = "target/crate_allow.toml";
    const CRATE_UNNECESSARY: &'static str = "target/crate_unnecessary.toml";

    fn worksapce_allow_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::WORKSPACE_ALLOW)
    }

    fn workspace_unnecessary_path() -> PathBuf {
        WorkspaceDir::workspace_root().join(Self::WORKSPACE_UNNECESSARY)
    }

    fn crate_allow_path() -> PathBuf {
        WorkspaceDir::crate_root().join(Self::CRATE_ALLOW)
    }

    fn crate_unnecessary_path() -> PathBuf {
        WorkspaceDir::crate_root().join(Self::CRATE_UNNECESSARY)
    }
}

pub async fn sort_lint() -> Result<()> {
    let lint_items = Arc::new(LintItem::get_lints().await?);

    let mut open_options = OpenOptions::new();
    let options = open_options
        .create(true)
        .read(true)
        .write(true)
        .truncate(true);

    let cargo = Arc::new(CargoManifest::load()?);

    let workspace = cargo.is_workspace().then(|| {
        let options = options.clone();
        let lint_items = Arc::clone(&lint_items);
        let cargo = Arc::clone(&cargo);

        let task: JoinHandle<Result<()>> = tokio::spawn(async move {
            let wk_allow = Output::worksapce_allow_path();
            let wk_ne = Output::workspace_unnecessary_path();

            let (al_f, ne) = join!(options.open(wk_allow), options.open(wk_ne));
            let (mut al_f, mut ne) = (al_f?, ne?);

            for ele in lint_items.iter() {
                if ele.is_allow() && !cargo.workspace_contains_lint(ele.id()) {
                    al_f.write_all(format!("{} = \"{}\"\n", ele.id(), ele.level()).as_bytes())
                        .await?;
                }
                if !ele.is_allow() && cargo.workspace_contains_lint(ele.id()) {
                    ne.write_all(format!("{} = \"{}\"\n", ele.id(), ele.level()).as_bytes())
                        .await?;
                }
            }
            Ok(())
        });

        task
    });

    let crate_ = cargo.is_crate().then(|| {
        let options = options.clone();
        let lint_items = Arc::clone(&lint_items);
        let cargo = Arc::clone(&cargo);

        let task: JoinHandle<Result<()>> = tokio::spawn(async move {
            let crate_allow = Output::crate_allow_path();
            let crate_ne = Output::crate_unnecessary_path();

            let (al_f, ne) = join!(options.open(crate_allow), options.open(crate_ne));
            let (mut al_f, mut ne) = (al_f?, ne?);

            for ele in lint_items.iter() {
                if ele.is_allow() && !cargo.crate_contains_lint(ele.id()) {
                    al_f.write_all(
                        format!(
                            "{} = \"{}\" # {}, {}\n",
                            ele.id(),
                            ele.level(),
                            ele.version(),
                            ele.applicability().applicability()
                        )
                        .as_bytes(),
                    )
                    .await?;
                }
                if !ele.is_allow() && cargo.crate_contains_lint(ele.id()) {
                    ne.write_all(format!("{} = \"{}\"\n", ele.id(), ele.level()).as_bytes())
                        .await?;
                }
            }
            Ok(())
        });

        task
    });

    if let Some(hd) = workspace {
        hd.await??;
    }
    if let Some(cr) = crate_ {
        cr.await??;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let out = Output::worksapce_allow_path();
        dbg!(out);
    }
}
