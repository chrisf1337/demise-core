use std::net::{TcpListener, TcpStream};

const PORT: i8 = 7654;

fn main() {
    let listener = TcpListener::bind(format!("localhost:{}", PORT).as_str()).unwrap();

    for stream in listener.incoming() {
        match stream {
            _ => {
                println!("Stream accepted!");
            }
        }
    }
}
