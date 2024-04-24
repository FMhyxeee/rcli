mod b64;
mod csv_convert;
mod gen_pass;
mod text;

use anyhow::Result;

pub trait Process {
    fn process(&self) -> Result<()>;
}
