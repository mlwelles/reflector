//! Display the captures for the provided mirror

use reflector::{Config, Mirror};
use std::env;

fn captures(m: &Mirror) {
    println!("captures in local store:");
    for c in m.local.all_captures().unwrap() {
        println!("{c}");
    }
}

fn main() {
    let cfg = Config::from(env::args());
    for src in cfg.sources {
        match Mirror::new(src.clone()) {
            Ok(m) => captures(&m),
            Err(e) => eprintln!("error with {src}: {:#?}", e),
        }
    }
    println!("done");
}
