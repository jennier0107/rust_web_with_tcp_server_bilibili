use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
#[allow(dead_code)]

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(&mut buffer).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Serving on 3000");

    for stream in listener.incoming() {
        handle_stream(stream?);
        println!("Connection established!")
    }
    Ok(())
}
