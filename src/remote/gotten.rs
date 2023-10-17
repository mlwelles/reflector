// use std::io::Bytes;
// use std::io::BufReader;
// use std::path::PathBuf;

#[allow(dead_code)]
pub struct Gotten {
    pub mimetype: String,
    pub resource: String,
    // support: ureq
    // reader: BufReader<Box<Self>>,
    // path: Option<PathBuf>,
    // payload: Bytes,
    // remote: SocketAddr,
}

impl Gotten {
    pub fn new(mimetype: &str, resource: &str) -> Gotten {
        let mimetype = mimetype.to_string();
        let resource = resource.to_string();
        Gotten { mimetype, resource }
    }
}
