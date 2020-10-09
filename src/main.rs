use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
    let response;

    stream.read(&mut buffer).unwrap();
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    response = format!(
        "{} {}",
        status_line,
        contents);

    println!("Requests {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
