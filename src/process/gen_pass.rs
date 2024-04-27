use crate::{
    cli::{GenPassOpts, LOWER, NUMBER, SYMBOL, UPPER},
    Process,
};
use anyhow::Result;
use rand::seq::SliceRandom;

impl Process for GenPassOpts {
    async fn process(&self) -> Result<()> {
        let result = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", result);
        Ok(())
    }
}

pub fn process_genpass(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();

    // In order to get all of the characters that the user wants to use in the password at least once,
    // we need to get at least one of each type of character.
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPER);
        if let Some(c) = UPPER.choose(&mut rng) {
            password.push(*c);
        }
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        if let Some(c) = LOWER.choose(&mut rng) {
            password.push(*c);
        }
    }
    if number {
        chars.extend_from_slice(NUMBER);
        if let Some(c) = NUMBER.choose(&mut rng) {
            password.push(*c);
        }
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        if let Some(c) = SYMBOL.choose(&mut rng) {
            password.push(*c);
        }
    }

    for _ in 0..(length - password.len()) {
        let c = chars.choose(&mut rng).expect(
            "chars won't be empty, please varify your input! DO NOT make all options false!",
        );
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    let strength = zxcvbn::zxcvbn(&password, &[])?;
    eprintln!("{:?}", strength);

    Ok(password)
}
