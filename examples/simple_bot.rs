use anyhow::*;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let (_bot, con) = miraie::Bot::new(
        "127.0.0.1:18418".parse().unwrap(),
        "dZujVWpnxxXXE5b",
        2394345431u64.into(),
    )
    .await?;
    con.run().await?;
    Ok(())
}
