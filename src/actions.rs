extern crate serde;
extern crate serde_json;
extern crate buffer;

use editor::Editor;
use serde_json::{Value};
use self::serde::ser::{Serializer, Serialize, SerializeMap};
use std::sync::{Arc, Mutex};
use std::fmt;
use buffer::{Point, Line, BufErr};

#[derive(Deserialize, Debug)]
pub enum Method {
    #[serde(rename = "connect")]
    Connect,
    #[serde(rename = "insertAtPt")]
    InsertAtPt
}

/* === Requests === */

#[derive(Deserialize, Debug)]
pub struct ConnectReq {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub method: Method
}

#[derive(Deserialize, Debug)]
pub struct InsertAtPtReq {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub method: Method,
    pub point: Point,
    pub string: String
}

pub trait Req {
    fn exec(&self, editor: &mut Arc<Mutex<Editor>>) -> Resp;
}

impl Req for ConnectReq {
    fn exec(&self, editor: &mut Arc<Mutex<Editor>>) -> Resp {
        debug!("Calling Message exec() for ConnectReq {:?}", self);
        let mut ed = editor.lock().unwrap();
        match ed.client_id {
            Some(_) => {
                Resp(Err(RespErr::ClientAlreadyConnected))
            }
            None => {
                ed.client_id = Some(self.client_id.clone());
                Resp(Ok(RespOk::ConnectResp(ConnRespStruct {
                    server_id: ed.server_id
                })))
            }
        }
    }
}

impl Req for InsertAtPtReq {
    fn exec(&self, editor: &mut Arc<Mutex<Editor>>) -> Resp {
        debug!("Calling Message exec() for InsertAtPtReq {:?}", self);
        let ref mut buffer = editor.lock().unwrap().buffer;
        match buffer.insert_at_pt(&self.string, &self.point) {
            Ok(lines_changed) => {
                Resp(Ok(RespOk::InsertAtPtOk(lines_changed)))
            }
            Err(err) => {
                Resp(Err(RespErr::InsertAtPtErr(err)))
            }
        }
    }
}

/* === Responses === */

pub enum RespErr {
    MalformedInput,
    InvalidMethod,
    MissingMethod,
    TestError,
    DeserializationError,
    ClientAlreadyConnected,
    InsertAtPtErr(BufErr)
}

pub enum RespOk {
    ConnectResp(ConnRespStruct),
    InsertAtPtOk(Vec<Line>),
    Ok
}

fn resp_err_code(resp_err: &RespErr) -> i32 {
    match resp_err {
        &RespErr::MalformedInput => 0,
        &RespErr::InvalidMethod => 1,
        &RespErr::MissingMethod => 2,
        &RespErr::TestError => 3,
        &RespErr::DeserializationError => 4,
        &RespErr::ClientAlreadyConnected => 5,
        &RespErr::InsertAtPtErr(_) => 6
    }
}

#[derive(Serialize)]
pub struct ConnRespStruct {
    #[serde(rename = "serverId")]
    pub server_id: String
};

impl fmt::Display for RespErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RespErr::MalformedInput => { write!(f, "malformed input") }
            &RespErr::InvalidMethod => { write!(f, "invalid method") }
            &RespErr::MissingMethod => { write!(f, "missing method") }
            &RespErr::TestError => { write!(f, "test error") },
            &RespErr::DeserializationError => { write!(f, "deserialization error") },
            &RespErr::ClientAlreadyConnected => { write!(f, "client already connected") },
            &RespErr::InsertAtPtErr(ref buf_err) => {
                write!(f, "insert at point error: {}", buf_err.to_string())
            }
        }
    }
}

pub struct Resp(pub Result<RespOk, RespErr>);

impl Serialize for RespErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Serialize as objection with two fields, an error code and an error message
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("code", &resp_err_code(self))?;
        map.serialize_entry("message", &self.to_string())?;
        map.end()
    }
}

impl Serialize for RespOk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            &RespOk::Ok => {
                // Serialize generic Ok result as null
                serializer.serialize_none()

                // ...or maybe as an empty object {}?
                // let map = serializer.serialize_map(Some(0))?;
                // map.end()
            }
            &RespOk::ConnectResp(ref s) => {
                // serde_json serializes structs with no fields as null
                s.serialize(serializer)
            }
            &RespOk::InsertAtPtOk(ref l) => {
                l.serialize(serializer)
            }
        }
    }
}

impl Serialize for Resp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self.0 {
            Ok(ref s) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("error", &false)?;
                map.serialize_entry("payload", &s)?;
                map.end()
            }
            Err(ref err) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("error", &true)?;
                map.serialize_entry("payload", &err)?;
                map.end()
            }
        }
    }
}
