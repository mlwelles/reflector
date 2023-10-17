// use std::io::Bytes;
// use std::io::BufReader;
// use std::path::PathBuf;

#[allow(dead_code)]
pub struct Gotten {
    mimetype: String,
    resource: String,
    // payload: Bytes,
    // reader: BufReader<Box<Self>>,
    // path: Option<PathBuf>,
}

impl Gotten {
    pub fn new(mimetype: &str, resource: &str) -> Gotten {
        let mimetype = mimetype.to_string();
        let resource = resource.to_string();
        Gotten { mimetype, resource }
    }
}
