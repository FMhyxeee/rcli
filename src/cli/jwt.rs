use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::process::{jwt_sign, jwt_verify};
use crate::CmdExector;

// cargo run -- jwt sign --key my-secret --sub 123 --aud 456 --exp 2022-12-12
// eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI0NTYiLCJleHAiOiIyMDIyLTEyLTEyIiwic3ViIjoiMTIzIn0.gpya5zrPk_ry4b_M9bOQb0GQ0LBCVIW6Fj6ZTfIu1IQ
//  cargo run -- jwt verify --key 'my-secret' --token eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI0NTYiLCJleHAiOiIyMDIyLTEyLTEyIiwic3ViIjoiMTIzIn0.gpya5zrPk_ry4b_M9bOQb0GQ0LBCVIW6Fj6ZTfIu1IQ

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Encode a jwt token")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "Verify a jwt token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    sub: String,
    #[arg(short, long)]
    aud: String,
    #[arg(short, long)]
    exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    token: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        jwt_sign(
            self.key.as_str(),
            self.sub.as_str(),
            self.aud.as_str(),
            self.exp.as_str(),
        )
        .await?;
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        jwt_verify(self.key.as_str(), self.token.as_str()).await?;
        Ok(())
    }
}
