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
    pub fn show(&self, with_docs: bool) -> String {
        let mut res = format!(
            "{} = \"{}\" # version: {}, applicability: {}, group: {}\n",
            self.id, self.level, self.version, self.applicability.applicability, self.group
        );

        if with_docs {
            let docs_str: String = self
                .docs
                .lines()
                .fold(String::new(), |output, line| {
                    format!("{}# {}\n", output, line)
                });

            res.push_str(&docs_str);
        }

        res
    }
    pub async fn get_lints(version: Option<&str>) -> Result<Vec<Self>> {
        let client = ClientBuilder::new()
            .brotli(true)
            .connect_timeout(Duration::from_secs(30))
            .build()?;
        let version = version.unwrap_or("stable");
        let url = format!(
            "https://rust-lang.github.io/rust-clippy/{}/lints.json",
            version
        );
        let lint_resp = client.get(url).send().await?;

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

#[test]
fn feature() {
    let item = LintItem {
        id: "absolute_paths".to_owned(),
        id_span: IdSpan::default(),
        group: "restriction".to_owned(),
        level: "allow".to_owned(),
        docs: r#"
### What it does
Checks for usage of items through absolute paths, like `std::env::current_dir`.

### Why restrict this?
Many codebases have their own style when it comes to importing, but one that is seldom used
is using absolute paths *everywhere*. This is generally considered unidiomatic, and you
should add a `use` statement.

The default maximum segments (2) is pretty strict, you may want to increase this in
`clippy.toml`.

Note: One exception to this is code from macro expansion - this does not lint such cases, as
using absolute paths is the proper way of referencing items in one.

### Example
```rust
let x = std::f64::consts::PI;
```
Use any of the below instead, or anything else:
```rust
use std::f64;
use std::f64::consts;
use std::f64::consts::PI;
let x = f64::consts::PI;
let x = consts::PI;
let x = PI;
use std::f64::consts as f64_consts;
let x = f64_consts::PI;
```

### Configuration
This lint has the following configuration variables:

- `absolute-paths-allowed-crates`: Which crates to allow absolute paths from (default: `[]`)- `absolute-paths-max-segments`: The maximum number of segments a path can have before being linted, anything above this will
   be linted. (default: `2`)"#.to_owned(),
        version: "1.73.0".to_owned(),
        applicability: Applicability {
            is_multi_part_suggestion: false,
            applicability: "Unresolved".to_owned()
        }
    };

    let s = item.show(true);
    println!("{s}");
    println!("====");
}
