use std::time::Duration;

use anyhow::Result;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
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

    pub fn is_warn(&self) -> bool {
        self.level == "warn"
    }

    pub fn is_deny(&self) -> bool {
        self.level == "deny"
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn level(&self) -> &str {
        &self.level
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn docs(&self) -> &str {
        &self.docs
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub const fn id_span(&self) -> &IdSpan {
        &self.id_span
    }

    pub const fn applicability(&self) -> &Applicability {
        &self.applicability
    }
}

#[derive(Clone, Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Applicability {
    is_multi_part_suggestion: bool,
    applicability: String,
}

impl Applicability {
    pub const fn is_multi_part_suggestion(&self) -> bool {
        self.is_multi_part_suggestion
    }

    pub fn applicability(&self) -> &str {
        &self.applicability
    }
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
