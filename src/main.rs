use std::io::Read;
use std::net::{TcpListener, TcpStream};

// mod guessgame;
//
// fn main() {
//     guessgame::main();
// }
// New prject vvvvvvvv
fn main() {
    let port = "7878";
    println!("listening on {}", port);
    let listener = TcpListener::bind(format!("{}:{}", "127.0.0.1", port)).unwrap();
    for tcp_accept_result in listener.incoming() {
        let stream = tcp_accept_result.unwrap();
        println!(
            "Connection {} <-> {} established",
            stream.local_addr().unwrap(),
            stream.peer_addr().unwrap()
        );

        handle_connection(stream);
        // println!("Done handling {}", stream.peer_addr().unwrap());
    }
    println!("done")
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handling {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
