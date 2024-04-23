use anyhow::Result;
use clap::Parser;
use rcli::{CliOpts, Process, SubCommand};

// 构想的命令 rcli csv -i input.csv -o output.csv --header -d ','
fn main() -> Result<()> {
    let opts = CliOpts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            opts.process()?;
        }
        SubCommand::GenPass(opts) => {
            opts.process()?;
        }
        SubCommand::Base64(opts) => {
            opts.process()?;
        }
        SubCommand::Text(opts) => {
            opts.process()?;
        }
    }
    Ok(())
}
