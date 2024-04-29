use anyhow::Result;
use rcli::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    // println!("{:?}", opts);
    opts.cmd.process().await?;
    Ok(())
}
