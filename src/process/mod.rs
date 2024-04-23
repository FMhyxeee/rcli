mod b64;
mod csv_convert;
mod gen_pass;
mod text;
mod utils;

use anyhow::Result;

pub use b64::Base64SubCommand;
pub use csv_convert::CsvOpts;
pub use gen_pass::GenPassOpts;
pub use text::TextSubcommand;

pub trait Process {
    fn process(&self) -> Result<()>;
}
