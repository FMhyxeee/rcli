use crate::{
    cli::{CsvOpts, OutputFormat},
    Process,
};

use std::fs;

use anyhow::Result;
use csv::Reader;

impl Process for CsvOpts {
    async fn process(&self) -> Result<()> {
        let mut reader = Reader::from_path(&self.input)?;
        // check if the csv file is empty
        if reader.records().next().is_none() {
            return Err(anyhow::Error::msg("[NOTICE] The input file is empty"));
        }
        let header = if self.header && reader.has_headers() {
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

        match self.format {
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(&ret)?;
                let filename = format!("{}.json", self.output);
                fs::write(filename, json)?;
                Ok(())
            }
            OutputFormat::Yaml => {
                let yaml = serde_yaml::to_string(&ret)?;
                let filename = format!("{}.yaml", self.output);
                fs::write(filename, yaml)?;
                Ok(())
            }
            OutputFormat::Toml => {
                // TODO: TOML does not support tuple
                // let toml = toml::to_string(&ret)?;
                // let filename = format!("{}.toml", opts.output);
                // fs::write(filename, toml)?;
                // Ok(())
                Err(anyhow::Error::msg("[ERROR] TOML does not support tuple"))
            }
            OutputFormat::Unknown => Err(anyhow::Error::msg(
                "[ERRPR] Unknown output format. You can only choose json, yaml or toml.",
            )),
        }
    }
}
