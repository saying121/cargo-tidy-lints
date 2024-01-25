use my_lints::get_lints;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    get_lints::get_lint().await
}
