// use std::io::Bytes;
use std::io::BufReader;

pub struct Gotten {
    // payload: Bytes,
    reader: BufReader<Box<Self>>,
    mimetype: String,
}
