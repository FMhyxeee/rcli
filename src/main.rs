use anyhow::Result;
use clap::Parser;
use rcli::{process, CliOpts, SubCommand};

// 构想的命令 rcli csv -i input.csv -o output.csv --header -d ','
fn main() -> Result<()> {
    let opts = CliOpts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process(opts)?;
        }
    }
    Ok(())
}
