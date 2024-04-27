use anyhow::Result;
use clap::Parser;
use rcli::{CliOpts, Process, SubCommand};

// 构想的命令 rcli csv -i input.csv -o output.csv --header -d ','
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts = CliOpts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            opts.process().await?;
        }
        SubCommand::GenPass(opts) => {
            opts.process().await?;
        }
        SubCommand::Base64(opts) => {
            opts.process().await?;
        }
        SubCommand::Text(opts) => {
            opts.process().await?;
        }
        SubCommand::Http(opts) => {
            opts.process().await?;
        }
    }
    Ok(())
}
