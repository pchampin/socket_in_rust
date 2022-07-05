use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::time::Duration;
use std::thread::{sleep, spawn};

fn main() -> Result<(), io::Error> {
    let map = Mutex::new(HashMap::new());
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    for stream in listener.incoming() {
        let stream = stream?;
        println!("{:?}", stream);
        //spawn(||
            if let Err(e) = handle(stream, &map) {
                eprintln!("=== {}", e);
            }
        //);
    }
    Ok(())
}

fn handle(mut stream: TcpStream, map: &Mutex<HashMap<usize, u16>>) -> Result<(), io::Error> {
    let port = stream.peer_addr()?.port();
    for _ in 0..5 {
        sleep(Duration::new(1, 0));
        let mut map = map.lock().unwrap();
        let mid = map.len();
        map.insert(mid, port);
        let msg = format!("hello {} ({})\n", port, mid);
        stream.write(msg.as_bytes())?;
    }
    Ok(())
}