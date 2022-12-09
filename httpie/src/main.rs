mod command;
mod request;

use clap::Parser;
use reqwest::Client;
use anyhow::{Result, Ok};

use command::{HttpieCli, Action};
use request::{get, post};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = HttpieCli::parse();
    let client = Client::new();

    match cli.subcmd {
        Action::Get(ref args) => get(client, args).await?,
        Action::Post(ref args) => post(client, args).await?,
    };
    
    Ok(())
}
