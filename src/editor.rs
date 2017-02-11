extern crate buffer;
use buffer::Buffer;

pub struct Editor {
    pub client_id: Option<String>,
    pub buffer: Buffer
}
