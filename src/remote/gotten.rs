// use std::io::Bytes;
// use std::io::BufReader;
// use std::path::PathBuf;
use url::Url;

#[allow(dead_code)]
pub struct Gotten {
    pub mimetype: String,
    pub resource: String,
    pub source: Url,
    // reader: BufReader<Box<Self>>,
    // remote: SocketAddr,
}

impl Gotten {
    pub fn new(mimetype: &str, resource: &str, source: Url) -> Gotten {
        let mimetype = mimetype.to_string();
        let resource = resource.to_string();
        Gotten {
            mimetype,
            resource,
            source,
        }
    }
}
