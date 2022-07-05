use std::net::TcpListener;
use std::io;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("{:?}", stream);
        stream.write("hello world\n".as_bytes())?;
    }
    Ok(())
}