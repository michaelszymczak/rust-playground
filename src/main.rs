use std::io::Read;
use std::net::{TcpListener, TcpStream};

// mod guessgame;
//
// fn main() {
//     guessgame::main();
// }
// New prject vvvvvvvv
fn main() {
    if 2 > 1 {
        sanbox();
    } else {
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
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handling {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

/////////////////////////////// sandbox //////////////////////////////////////////////

fn sanbox() {
    let immutable_foo = Foo { value: 0 };
    let immutable_bar = Foo { value: 10 };
    // immutable_foo.mutate(); // impossible
    // immutable_bar.mutate(); // impossible
    println!("immutable_foo: {:?}", immutable_foo);
    println!("immutable_bar: {:?}", immutable_bar);

    // 1. mutability can change after passing to a function

    let returned_immutable_foo = fn_taking_mutable_foo(immutable_foo); // weird but possible

    // I guess the above is safe as this is impossible
    // println!("immutable_foo: {:?}", immutable_foo);

    println!("returned_immutable_foo: {:?}", returned_immutable_foo); // possible

    // 2. mutability can change after reassignment

    let mut mutable_bar = immutable_bar;
    mutable_bar.mutate();
    println!("mutable_bar: {:?}", mutable_bar);
}

fn fn_taking_mutable_foo(mut sth: Foo) -> Foo {
    sth.mutate();
    sth
}

#[derive(Debug)]
struct Foo {
    value: i32,
}

impl Foo {
    fn mutate(&mut self) -> () {
        println!("mutating {}", self.value);
        self.value = self.value + 1;
    }
}
