//! Indicate the status of the provided mirror

use reflector::{Config, Mirror, MirrorStatus};
use std::env;

fn summarize_status(m: &Mirror, st: &MirrorStatus) {
    println!("{}\t{}", m.name, m.local);
    println!("\tstatus:\t{}", st);
}

fn main() {
    let cfg = Config::try_from(env::args()).unwrap();
    for src in cfg.sources {
        match Mirror::new(src.clone()) {
            Ok(mut m) => match m.status() {
                Ok(st) => {
                    summarize_status(&m, &st);
                }
                Err(e) => eprintln!("{} status error: {:?}", m.name, e),
            },
            Err(e) => eprintln!("error with {src}: {:#?}", e),
        }
    }
    println!("done");
}
