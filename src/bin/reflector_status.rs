use reflector::{Config, Mirror, MirrorStatus};
use std::env;

fn summarize_status(m: &Mirror, st: &MirrorStatus) {
    println!("{}\t{}", m.name, m.local);
    println!("\tstatus:\t{}", st);
}

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
            Ok(mut m) => match m.status() {
                Ok(st) => {
                    summarize_status(&m, &st);
                    captures(&m);
                }
                Err(e) => eprintln!("{} status error: {:?}", m.name, e),
            },
            Err(e) => eprintln!("error with {src}: {:#?}", e),
        }
    }
    println!("done");
}
