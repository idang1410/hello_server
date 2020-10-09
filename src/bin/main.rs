extern crate ctrlc;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use hello_server::ThreadPool;

fn main() {
    let exit_handle = Arc::new(AtomicBool::new(false));
    let exit_dup = exit_handle.clone();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    ctrlc::set_handler(move || {
        exit_dup.store(true, Ordering::SeqCst);
    }).expect("Error setting ctrc");

    for stream in listener.incoming() {
        if exit_handle.load(Ordering::SeqCst) {
            break;
        }
        let stream = stream.unwrap();

        pool.execute(|| {
        handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let response;

    stream.read(&mut buffer).unwrap();
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
