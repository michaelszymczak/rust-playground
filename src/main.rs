use hello::ThreadPool;
use std::net::TcpListener;

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
    let pool = ThreadPool::new(4);
    for tcp_accept_result in listener.incoming() {
        let stream = tcp_accept_result.unwrap();
        pool.execute(|| {
            println!(
                "Connection {} <-> {} established",
                stream.local_addr().unwrap(),
                stream.peer_addr().unwrap()
            );

            hello::handle_connection(stream);
        });
    }
    println!("done")
}
