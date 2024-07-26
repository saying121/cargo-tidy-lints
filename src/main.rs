use anyhow::Result;
use cargo_tidy_lints::{lint_item::LintItem, sorting_lints};

#[tokio::main]
async fn main() -> Result<()> {
    let lints = LintItem::get_lints(None).await?;
    sorting_lints::tidy_lint(lints, false).await?;

    Ok(())
}
