use std::time::Duration;

use anyhow::Result;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LintItem {
    #[serde(default)]
    id: String,
    #[serde(default)]
    id_span: IdSpan,
    #[serde(default)]
    group: String,
    #[serde(default)]
    level: String,
    #[serde(default)]
    docs: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    applicability: Applicability,
}

impl LintItem {
    pub async fn get_lints() -> Result<Vec<Self>> {
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()?;
        let lint_resp = client
            .get("https://rust-lang.github.io/rust-clippy/master/lints.json")
            .send()
            .await?;

        let lint_items: Vec<Self> = lint_resp.json().await?;
        Ok(lint_items)
    }

    pub fn is_allow(&self) -> bool {
        self.level == "allow"
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn level(&self) -> &str {
        &self.level
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Applicability {
    pub is_multi_part_suggestion: bool,
    pub applicability: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdSpan {
    pub path: String,
    pub line: i64,
}
