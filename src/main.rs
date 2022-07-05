use std::net::TcpListener;
use std::io;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    for stream in listener.incoming() {
        println!("{:?}", stream);
    }
    Ok(())
}