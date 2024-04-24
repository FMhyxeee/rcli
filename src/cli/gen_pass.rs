use clap::Parser;

pub const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
pub const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
pub const NUMBER: &[u8] = b"0123456789";
pub const SYMBOL: &[u8] = b"!@#$%^&*_";

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, value_parser = least_length)]
    pub length: usize,
    #[arg(long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
    #[arg(long, default_value_t = false)]
    pub estimate_strength: bool,
}

fn least_length(length: &str) -> Result<usize, &'static str> {
    let length = length
        .parse::<usize>()
        .map_err(|_| "Please input a number!")?;
    if length < 4 {
        Err("The length of password must be greater than 4!")
    } else {
        Ok(length)
    }
}
