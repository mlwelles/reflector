// use std::io::Bytes;
// use std::io::BufReader;

pub struct Gotten {
    // payload: Bytes,
    // reader: BufReader<Box<Self>>,
    mimetype: String,
}

impl Gotten {
    pub fn new(mimetype: &str) -> Gotten {
        let mimetype = mimetype.to_string();
        Gotten { mimetype }
    }
}
