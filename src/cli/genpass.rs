use clap::Parser;
use zxcvbn::zxcvbn;

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, value_parser = least_length)]
    pub length: u8,
    #[arg(long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
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

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", ret);

        // output password strength in stderr
        let estimate = zxcvbn(&ret, &[])?;
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
