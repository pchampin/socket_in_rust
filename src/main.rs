use async_std::prelude::*;
use async_std::net::TcpListener;
use async_std::task;
use std::io;

fn main() -> Result<(), io::Error> {
    task::block_on(run())
}

async fn run() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        println!("{:?}", stream);
        stream.write("hello world\n".as_bytes()).await?;
    }
    Ok(())
}