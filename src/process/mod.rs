mod csv_convert;
mod gen_pass;

use anyhow::Result;

pub use csv_convert::CsvOpts;
pub use gen_pass::GenPassOpts;

pub trait Process {
    fn process(&self) -> Result<()>;
}
