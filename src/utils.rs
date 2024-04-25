use anyhow::Result;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if the input is "-", we should read from stdin
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("The input file does not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("The path does not exist or is not a directory")
    }
}

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use crate::utils::verify_file;

    #[test]
    fn test_varify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_file("input.csv"),
            Err("The input file does not exist")
        );
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
    }
}
