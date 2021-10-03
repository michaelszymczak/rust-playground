use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{fs, thread};

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    println!("Handling {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..size]));
    stream.write(response(&buffer).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn response(request_buffer: &[u8]) -> String {
    return match response_body(request_buffer) {
        Some(body) => {
            let headers = format!("Content-Length:{}", body.len());
            let response_result = "HTTP/1.1 200 OK";
            format!("{}\r\n{}\r\n\r\n{}", response_result, headers, body)
        }
        None => {
            let body = fs::read_to_string("404.html").unwrap();
            let headers = format!("Content-Length:{}", body.len());
            let response_result = "HTTP/1.1 404 NOT FOUND";
            format!("{}\r\n{}\r\n\r\n{}", response_result, headers, body)
        }
    };
}

fn response_body(request_buffer: &[u8]) -> Option<String> {
    if request_buffer.starts_with(b"GET / HTTP") {
        Some(fs::read_to_string("hello.html").unwrap())
    } else if request_buffer.starts_with(b"GET /sleep HTTP") {
        thread::sleep(Duration::from_secs(5));
        Some(fs::read_to_string("hello.html").unwrap())
    } else if request_buffer.starts_with(b"GET /favicon.ico HTTP") {
        Some(String::from(""))
    } else {
        None
    }
}
