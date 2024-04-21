use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;
use csv::Reader;

// 构想的命令 rcli csv -i input.csv -o output.csv --header -d ','
fn main() -> Result<()> {
    let opts = CliOpts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            // check if the csv file is empty
            if reader.records().next().is_none() {
                return Err(anyhow::Error::msg("[NOTICE] The input file is empty"));
            }
            let header = if opts.header {
                reader
                    .headers()?
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            } else {
                // if no header, we should create some default value such as col1, col2, col3...
                // read the first record to get the column count
                let first_record = reader.records().next().unwrap()?;
                first_record
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("col{}", i + 1))
                    .collect::<Vec<String>>()
            };

            // zip the header with the records
            let records = reader.records();

            let mut ret = Vec::with_capacity(128);
            for record in records {
                let record = record?
                    .iter()
                    .zip(header.iter())
                    .map(|(v, k)| (k.clone(), v.to_string()))
                    .collect::<Vec<(String, String)>>();
                ret.push(record);
            }

            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, author = "hyx", long_about = None)]
struct CliOpts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert csv to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = varify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.csv")]
    output: String,
    #[arg(long, default_value_t = true)]
    header: bool,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn varify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found, please check your file if exist!")
    }
}
