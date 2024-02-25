use std::{ops::Not, path::Path, time::Duration};

use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    join,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct LintItem {
    #[serde(default)]
    pub id:            String,
    #[serde(default)]
    pub id_span:       IdSpan,
    #[serde(default)]
    pub group:         String,
    #[serde(default)]
    pub level:         String,
    #[serde(default)]
    pub docs:          String,
    #[serde(default)]
    pub version:       String,
    #[serde(default)]
    pub applicability: Applicability,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Applicability {
    pub is_multi_part_suggestion: bool,
    pub applicability:            String,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct IdSpan {
    pub path: String,
    pub line: i64,
}

pub async fn get_lint() -> anyhow::Result<()> {
    let client = ClientBuilder::new()
        .gzip(true)
        .connect_timeout(Duration::from_secs(30))
        .build()?;
    let lint_resp = client
        .get("https://rust-lang.github.io/rust-clippy/master/lints.json")
        .send()
        .await?;

    let mut t = OpenOptions::new();
    t.create(true)
        .read(true)
        .write(true)
        .truncate(true);

    let (lint_items, output_f, snippet_str) = join!(
        lint_resp.json(),
        t.open("./allow.toml"),
        fs::read_to_string(Path::new("./Cargo.toml"))
    );
    let (lint_items, mut output_f, snippet_str): (Vec<LintItem>, fs::File, String) =
        (lint_items?, output_f?, snippet_str?);

    for ele in &lint_items {
        if ele.level == "allow" && snippet_str.contains(&ele.id).not() {
            output_f
                .write_all(format!("{} = \"warn\"\n", ele.id).as_bytes())
                .await?;
        }
    }
    Ok(())
}
