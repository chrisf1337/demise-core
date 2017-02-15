#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate buffer;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

use buffer::Buffer;
use serde_json::Value;

mod actions;
use actions::*;

mod editor;
use editor::Editor;

const IN_PORT: i16 = 8765;
const OUT_PORT: i16 = 8766;

fn send_response(stream: &mut TcpStream, response: Response) {
    match response.0 {
        Ok(ResponseOk::ConnectResponse(resp)) => {

        }
        Ok(_) => {

        }
        Err(err) => {

        }
    }
}

fn in_handle_client(mut editor: Arc<Mutex<Editor>>, stream: TcpStream) -> Result<(), serde_json::Error> {
    println!("Inbound connection from {}", stream.peer_addr().unwrap().ip());
    loop {
        let input: serde_json::Result<Value> = serde_json::from_reader(&stream);
        match input {
            Ok(input) => {
                println!("{:?}", input);
                match input["method"].as_str() {
                    Some("connect") => {
                        let connect_input: ConnectRequest = serde_json::from_value(input)?;
                        match connect_input.exec(&mut editor).0 {
                            Ok(_) => {
                                println!("connect method ok");
                            }
                            Err(err) => {
                                println!("connect method err");
                            }
                        }
                    }
                    Some(&_) => {
                        println!("Illegal input: {}", input);
                    }
                    None => {
                        println!("Illegal input: {}", input);
                    }
                }
            }
            Err(err) => {
                println!("Illegal input: {}", err);
                continue;
            }
        }
    }
}

fn out_handle_client(stream: TcpStream) {
    println!("Outbound connection to {}", stream.peer_addr().unwrap().ip());
}

fn main() {
    let editor = Arc::new(Mutex::new(Editor {
        client_id: None,
        buffer: Buffer::new()
    }));

    let in_listener = TcpListener::bind(format!("127.0.0.1:{}", IN_PORT).as_str()).unwrap();
    let out_listener = TcpListener::bind(format!("127.0.0.1:{}", OUT_PORT).as_str()).unwrap();

    println!("Listening for input on port {}", IN_PORT);
    println!("Sending output on port {}", OUT_PORT);

    let in_thread = thread::spawn(move || {
        for stream in in_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let editor = editor.clone();
                    thread::spawn(move || {
                        match in_handle_client(editor, stream) {
                            Ok(_) => {
                                println!("in_handle_client() exited successfully");
                            }
                            Err(err) => {
                                println!("in_handle_client() exited with error: {:?}", err);
                            }
                        }
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

    let v = vec![
        Response(Ok(ResponseOk::ConnectResponse(ConnRespStruct {
            test_field: 214
        }))),
        Response(Ok(ResponseOk::Ok)),
        Response(Err(ResponseErr::TestError))
    ];

    for elem in &v {
        println!("{}", serde_json::to_string(elem).unwrap());
    }

    in_thread.join().unwrap();
    out_thread.join().unwrap();
}
