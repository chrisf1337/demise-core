#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate log;
extern crate buffer;
extern crate byteorder;
extern crate color_logger;
extern crate uuid;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, ErrorKind, Cursor};
use log::LogLevelFilter;

use buffer::{Buffer, IntoLine};
use serde_json::Value;

mod actions;
use actions::*;

mod editor;
use editor::Editor;

const IN_PORT: i16 = 8765;
const OUT_PORT: i16 = 8766;
const PACKET_SIZE_BYTES: usize = 4;

fn in_handle_client(mut editor: Arc<Mutex<Editor>>, mut stream: TcpStream) {
    debug!("Inbound connection from {}", stream.peer_addr().unwrap().ip());
    loop {
        let mut recv_buf: Vec<u8>;
        let mut recv_size_buf = [0u8; PACKET_SIZE_BYTES];

        let _ = match stream.read(&mut recv_size_buf) {
            Err(e) => {
                error!("Stream read error: {}", e);
                continue;
            }
            Ok(m) => {
                if m == 0 {
                    info!("Client disconnected (stream read 0 bytes)");
                    break;
                }
                m
            }
        };
        let recv_size = match Cursor::new(&recv_size_buf).read_u32::<LittleEndian>() {
            Ok(size) => size,
            Err(e) => {
                panic!("Size conversion error: {}", e);
            }
        };
        debug!("Size received: {}", recv_size);

        recv_buf = vec![0; recv_size as usize];
        let _ = match stream.read(&mut recv_buf[..]) {
            Err(e) => {
                error!("Stream read error: {}", e);
                continue;
            }
            Ok(m) => {
                if m == 0 {
                    info!("Client disconnected (stream read 0 bytes)");
                    break;
                }
                m
            }
        };

        let input: serde_json::Result<Value> = serde_json::from_slice(&recv_buf[..]);

        debug!("deserialized input: {:?}", input);
        let resp = match input {
            Ok(input) => {
                debug!("{:?}", input);
                match input["method"].as_str() {
                    Some("connect") => {
                        let connect_input: Result<ConnectReq, serde_json::error::Error> =
                            serde_json::from_value(input);
                        match connect_input {
                            Ok(inp) => inp.exec(&mut editor),
                            Err(_) => Resp(Err(RespErr::DeserializationError))
                        }
                    }
                    Some("insertAtPt") => {
                        let insert_input: Result<InsertAtPtReq, serde_json::error::Error> =
                            serde_json::from_value(input);
                        match insert_input {
                            Ok(inp) => inp.exec(&mut editor),
                            Err(_) => Resp(Err(RespErr::DeserializationError))
                        }
                    }
                    Some(&_) => {
                        warn!("Invalid method: {}", input);
                        Resp(Err(RespErr::InvalidMethod))
                    }
                    None => {
                        error!("Missing method: {}", input);
                        Resp(Err(RespErr::MissingMethod))
                    }
                }
            }
            Err(err) => {
                error!("Illegal input: {}", err);
                Resp(Err(RespErr::MalformedInput))
            }
        };

        // Send resp back to client
        let resp_str = serde_json::to_string(&resp);
        if resp_str.is_err() {
            error!("Serialization error: {}", resp_str.unwrap_err());
            continue;
        }
        let s = resp_str.unwrap();
        let sb = s.as_bytes();
        let sblen = sb.len();
        let mut send_buf = vec![];
        match send_buf.write_u32::<LittleEndian>(sblen as u32) {
            Ok(_) => {}
            Err(err) => {
                panic!("Size conversion error: {}", err);
            }
        }
        send_buf.extend_from_slice(sb);
        match stream.write(&send_buf[..]) {
            Ok(_) => debug!("Sent {}-byte resp to client: {}", sblen, s),
            Err(err) => {
                match err.kind() {
                    ErrorKind::BrokenPipe => {
                        info!("Client disconnected");
                        break;
                    }
                    _ => error!("Stream write error: {}", err)
                }
            }
        }
    }
}

fn out_handle_client(stream: TcpStream) {
    debug!("Outbound connection to {}", stream.peer_addr().unwrap().ip());
}

fn main() {
    color_logger::init(LogLevelFilter::Debug).unwrap();

    let editor = Arc::new(Mutex::new(Editor {
        client_id: None,
        server_id = Uuid::new_v4(),
        buffer: Buffer::new()
    }));

    let in_listener = TcpListener::bind(format!("127.0.0.1:{}", IN_PORT).as_str()).unwrap();
    let out_listener = TcpListener::bind(format!("127.0.0.1:{}", OUT_PORT).as_str()).unwrap();

    debug!("Listening for input on port {}", IN_PORT);
    debug!("Sending output on port {}", OUT_PORT);

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
                    error!("In listener error: {}", e);
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
                    error!("Out listener error: {}", e);
                }
            }
        }
    });

    let v = vec![
        Resp(Ok(RespOk::ConnectResp(ConnRespStruct {}))),
        Resp(Ok(RespOk::Ok)),
        Resp(Err(RespErr::TestError)),
        Resp(Ok(RespOk::InsertAtPtOk(vec![
            "ab".into_line(0),
            "cd".into_line(1)
        ])))
    ];

    for elem in &v {
        debug!("{}", serde_json::to_string(elem).unwrap());
    }

    in_thread.join().unwrap();
    out_thread.join().unwrap();
}
