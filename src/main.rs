extern crate gapbuffer;
extern crate byteorder;

use gapbuffer::GapBuffer;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Cursor, Read, Write};
use byteorder::{LittleEndian, ReadBytesExt};

const IN_PORT: i16 = 8765;
const OUT_PORT: i16 = 8766;

const PACKET_SIZE_BYTES: usize = 4;

struct Editor {
    buffer: GapBuffer
}

fn in_handle_client(mut stream: TcpStream) {
    println!("Inbound connection from {}", stream.peer_addr().unwrap().ip());

    loop {
        let mut recv_buf: Vec<u8>;
        let mut recv_size_buf = [0u8; PACKET_SIZE_BYTES];

        let _ = match stream.read(&mut recv_size_buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };

        println!("{:?}", recv_size_buf);
        let recv_size = match Cursor::new(&recv_size_buf).read_u32::<LittleEndian>() {
            Ok(size) => size,
            Err(e) => {
                panic!("Size conversion error: {}", e);
            }
        };
        println!("Size received: {}", recv_size);
    }
}

fn out_handle_client(stream: TcpStream) {
    println!("Outbound connection to {}", stream.peer_addr().unwrap().ip());
}

fn main() {
    let editor = Arc::new(Mutex::new(Editor {
        buffer: GapBuffer::new()
    }));

    let in_listener = TcpListener::bind(format!("127.0.0.1:{}", IN_PORT).as_str()).unwrap();
    let out_listener = TcpListener::bind(format!("127.0.0.1:{}", OUT_PORT).as_str()).unwrap();

    println!("Listening for input on port {}", IN_PORT);
    println!("Sending output on port {}", OUT_PORT);

    let in_thread = thread::spawn(move || {
        for stream in in_listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        in_handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("In listener error: {}", e);
                }
            }
        }
    });

    let out_thread = thread::spawn(move || {
        for stream in out_listener.incoming() {
            match stream {
                Ok(stream) => {

                }
                Err(e) => {
                    println!("Out listener error: {}", e);
                }
            }
        }
    });

    in_thread.join().unwrap();
    out_thread.join().unwrap();
}
