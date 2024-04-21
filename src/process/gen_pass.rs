use crate::Process;

use anyhow::Result;
use clap::Parser;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

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

impl Process for GenPassOpts {
    fn process(&self) -> Result<()> {
        let mut rng = rand::thread_rng();
        let mut password = Vec::new();

        // In order to get all of the characters that the user wants to use in the password at least once,
        // we need to get at least one of each type of character.
        let mut chars = Vec::new();

        if self.uppercase {
            chars.extend_from_slice(UPPER);
            if let Some(c) = UPPER.choose(&mut rng) {
                password.push(*c);
            }
        }
        if self.lowercase {
            chars.extend_from_slice(LOWER);
            if let Some(c) = LOWER.choose(&mut rng) {
                password.push(*c);
            }
        }
        if self.number {
            chars.extend_from_slice(NUMBER);
            if let Some(c) = NUMBER.choose(&mut rng) {
                password.push(*c);
            }
        }
        if self.symbol {
            chars.extend_from_slice(SYMBOL);
            if let Some(c) = SYMBOL.choose(&mut rng) {
                password.push(*c);
            }
        }

        for _ in 0..(self.length - password.len()) {
            let c = chars.choose(&mut rng).expect(
                "chars won't be empty, please varify your input! DO NOT make all options false!",
            );
            password.push(*c);
        }

        password.shuffle(&mut rng);

        let password = String::from_utf8(password)?;

        println!("{}", password);

        if self.estimate_strength {
            let strength = zxcvbn::zxcvbn(&password, &[])?;
            println!("{:?}", strength);
        }

        Ok(())
    }
}
