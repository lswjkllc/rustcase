use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use simkvs::{CommandRequest, CommandResponse};
use tokio::net::TcpListener;
use tracing::info;

use simkvs::{Service, MemTable};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // 初始化 service
    let service: Service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    let svc2 = service.clone();
    tokio::spawn(async move {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            svc2.print_elems("table1");
        }
    });

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        // 复制一份 service
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(cmd)) = stream.next().await {
                let resp = svc.execute(cmd);
                stream.send(resp).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}