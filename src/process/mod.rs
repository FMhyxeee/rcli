mod b64;
mod csv_convert;
mod gen_pass;
mod http_server;
mod text;

use anyhow::Result;
#[allow(async_fn_in_trait)]
pub trait Process {
    async fn process(&self) -> Result<()>;
}
