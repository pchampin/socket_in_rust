use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use std::io;
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    task::block_on(run())
}

async fn run() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("{:?}", stream);
        if let Err(e) = handle(stream).await {
            eprintln!("=== {}", e);
        }
    }
    Ok(())
}

async fn handle(mut stream: TcpStream) -> Result<(), io::Error> {
    let port = stream.peer_addr()?.port();
    for _ in 0..5 {
        task::sleep(Duration::new(1, 0)).await;
        let msg = format!("hello {}\n", port);
        stream.write(msg.as_bytes()).await?;
    }
    Ok(())
}