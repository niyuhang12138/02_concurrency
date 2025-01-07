use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listener on: {addr}");
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {raddr}");
        tokio::spawn(async move {
            if let Err(e) = process_redis(stream, raddr).await {
                warn!("Error processing conn with {raddr}: {e:?}");
            };
        });
    }
}

async fn process_redis(stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {n} bytes");
                let line = String::from_utf8_lossy(&buf);
                info!("{line}");
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {raddr} closed");
    Ok(())
}
