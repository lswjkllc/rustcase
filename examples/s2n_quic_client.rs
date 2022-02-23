use anyhow::{anyhow, Result};
use s2n_quic::{Client, client::Connect};
use std::net::SocketAddr;

const CERT_PEM: &str = include_str!("../fixtures/cert.pem");

#[tokio::main]
async fn main() -> Result<()> {
    // create client
    let client = Client::builder()
        .with_tls(CERT_PEM)?
        .with_io("0.0.0.0:0")?
        .start()
        .map_err(|e| anyhow!("Failed to start client. Error: {e}"))?;

    // create connection
    let server_addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let connect = Connect::new(server_addr).with_server_name("localhost");
    let mut conn = client.connect(connect).await?;

    // keep alive
    conn.keep_alive(true)?;

    // open a new bidirectional stream
    let stream = conn.open_bidirectional_stream().await?;
    let (mut rx, mut tx) = stream.split();

    // spawn a tokio task to copy server data to stdout
    tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        if let Err(e) = tokio::io::copy(&mut rx, &mut stdout).await {
            println!("Failed to copy data from server. Error: {e}");
        }
    });

    // copy stdin to server
    let mut stdin = tokio::io::stdin();
    tokio::io::copy(&mut stdin, &mut tx).await?;

    Ok(())
}
