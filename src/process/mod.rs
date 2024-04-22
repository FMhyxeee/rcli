mod csv_convert;
mod gen_pass;
mod b64;

use std::path::Path;

use anyhow::Result;

pub use csv_convert::CsvOpts;
pub use gen_pass::GenPassOpts;
pub use b64::Base64SubCommand;

pub trait Process {
    fn process(&self) -> Result<()>;
}


fn varify_input_file(filename: &str) -> Result<String, &'static str> {
    // if the input is "-", we should read from stdin
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("The input file does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varify_input_file() {
        assert_eq!(varify_input_file("-"), Ok("-".to_string()));
        assert_eq!(varify_input_file("input.csv"), Err("The input file does not exist"));
        assert_eq!(varify_input_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
    }
}
