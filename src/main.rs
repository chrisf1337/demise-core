#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate log;
extern crate env_logger;


extern crate buffer;


use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, ErrorKind};

use buffer::Buffer;
use serde_json::Value;

mod actions;
use actions::*;

mod editor;
use editor::Editor;

const IN_PORT: i16 = 8765;
const OUT_PORT: i16 = 8766;

fn in_handle_client(mut editor: Arc<Mutex<Editor>>, mut stream: TcpStream) {
    debug!("Inbound connection from {}", stream.peer_addr().unwrap().ip());
    loop {
        let input: serde_json::Result<Value> = serde_json::from_reader(&stream);
        debug!("deserialized input: {:?}", input);
        let response = match input {
            Ok(input) => {
                debug!("{:?}", input);
                match input["method"].as_str() {
                    Some("connect") => {
                        let connect_input: Result<ConnectRequest, serde_json::error::Error> =
                            serde_json::from_value(input);
                        match connect_input {
                            Ok(cinp) => cinp.exec(&mut editor),
                            Err(_) => Response(Err(ResponseErr::DeserializationError))
                        }
                    }
                    Some(&_) => {
                        warn!("Invalid method: {}", input);
                        Response(Err(ResponseErr::InvalidMethod))
                    }
                    None => {
                        error!("Missing method: {}", input);
                        Response(Err(ResponseErr::MissingMethod))
                    }
                }
            }
            Err(err) => {
                error!("Illegal input: {}", err);
                Response(Err(ResponseErr::MalformedInput))
            }
        };

        // Send response back to client
        let response_str = serde_json::to_string(&response);
        if response_str.is_err() {
            error!("Serialization error: {}", response_str.unwrap_err());
            continue;
        }
        let s = response_str.unwrap();
        match stream.write(s.as_bytes()) {
            Ok(_) => debug!("Sent response to client: {}", s),
            Err(err) => {
                match err.kind() {
                    ErrorKind::BrokenPipe => {
                        error!("Client disconnected");
                        break;
                    }
                    _ => error!("Stream write error: {}", err)
                }
            }
        }
    }
}

fn out_handle_client(stream: TcpStream) {
    println!("Outbound connection to {}", stream.peer_addr().unwrap().ip());
}

fn main() {
    env_logger::init().unwrap();

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
                    stream.set_read_timeout(None).unwrap();
                    let editor = editor.clone();
                    thread::spawn(move || {
                        in_handle_client(editor, stream);
                        debug!("Client thread exiting");
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
