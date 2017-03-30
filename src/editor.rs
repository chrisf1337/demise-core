extern crate buffer;
extern crate uuid;
use buffer::Buffer;

pub struct Editor {
    pub client_id: Option<String>,
    pub server_id: uuid::Uuid,
    pub buffer: Buffer
}
