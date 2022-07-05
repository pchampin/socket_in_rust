use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::thread::{sleep, spawn};

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    for stream in listener.incoming() {
        let stream = stream?;
        println!("{:?}", stream);
        spawn(||
            if let Err(e) = handle(stream) {
                eprintln!("=== {}", e);
            }
        );
    }
    Ok(())
}

fn handle(mut stream: TcpStream) -> Result<(), io::Error> {
    let port = stream.peer_addr()?.port();
    let msg = format!("hello {}\n", port);
    for _ in 0..5 {
        sleep(Duration::new(1, 0));
        stream.write(msg.as_bytes())?;
    }
    Ok(())
}