use std::sync::Arc;

use anyhow::Result;

use crate::{cargo::CargoManifest, lint_item::LintItem};

pub async fn tidy_lint(lints: Vec<LintItem>, with_docs: bool) -> Result<()> {
    let lints = Arc::new(lints);

    let cargo = Arc::new(CargoManifest::load_workspace()?);
    let wk_task = cargo.is_workspace().then(|| {
        let cargo = Arc::clone(&cargo);
        let lints = Arc::clone(&lints);

        tokio::spawn(async move {
            cargo
                .tidy_workspace(&lints, with_docs)
                .await
        })
    });
    let crate_task = (cargo.is_crate() && !cargo.crate_inherit()).then(|| {
        let cargo = Arc::clone(&cargo);
        let lints = Arc::clone(&lints);

        tokio::spawn(async move {
            cargo
                .tidy_crate(&lints, with_docs)
                .await
        })
    });

    if let Some(task) = wk_task {
        task.await??;
    }
    if let Some(task) = crate_task {
        task.await??;
    }

    Ok(())
}
