use std::path::Path;

use anyhow::Result;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    join,
};

use crate::get_lints::LintItem;

pub async fn sort_lint() -> Result<()> {
    let lint_items = LintItem::get_lints().await?;

    let mut allow_f = OpenOptions::new();
    allow_f
        .create(true)
        .read(true)
        .write(true)
        .truncate(true);

    let mut not_allow_f = OpenOptions::new();
    not_allow_f
        .create(true)
        .read(true)
        .write(true)
        .truncate(true);

    let cargo = crate::cargo::CargoManifest::build_workspace()?;



    let (all_file, not_allow, snippet_str) = join!(
        allow_f.open("./default_allow.toml"),
        not_allow_f.open("./default_warn_or_deny.toml"),
        fs::read_to_string(Path::new("./Cargo.toml"))
    );
    let (mut allo_f, mut not_allow_f, snippet_str): (fs::File, fs::File, String) =
        (all_file?, not_allow?, snippet_str?);

    for ele in &lint_items {
        let contain = snippet_str.contains(ele.id());

        if ele.is_allow() && !contain {
            allo_f
                .write_all(format!("{} = \"{}\"\n", ele.id(), ele.level()).as_bytes())
                .await?;
        }
        if !ele.is_allow() && contain {
            not_allow_f
                .write_all(format!("{} = \"{}\"\n", ele.id(), ele.level()).as_bytes())
                .await?;
        }
    }

    Ok(())
}
