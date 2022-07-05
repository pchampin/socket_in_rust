use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use std::io;
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    task::block_on(run())
}

async fn run() -> Result<(), io::Error> {
    let mut state = vec![];
    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("{:?}", stream);
        //task::spawn(async {
            if let Err(e) = handle(stream, &mut state).await {
                eprintln!("=== {}", e);
            }
        //});
    }
    Ok(())
}

async fn handle(mut stream: TcpStream, state: &mut Vec<u16>) -> Result<(), io::Error> {
    let port = stream.peer_addr()?.port();
    for _ in 0..5 {
        task::sleep(Duration::new(1, 0)).await;
        let mid = state.len();
        state.push(port);
        let msg = format!("hello {} ({})\n", port, mid);
        stream.write(msg.as_bytes()).await?;
    }
    Ok(())
}