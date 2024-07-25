use cargo_tidy_lints::sorting_lints;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sorting_lints::sort_lint().await?;
    Ok(())
}
