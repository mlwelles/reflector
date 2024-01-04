use reflector::{Config, Mirror};
use std::env;

fn main() {
    let cfg = Config::from(env::args());
    for src in cfg.sources {
        match Mirror::new(src.clone()) {
            Ok(mut m) => match m.status() {
                Ok(st) => {
                    println!("{}\t{}", m.name, m.local);
                    println!("{}\tstatus: {}", m.name, st);
                }
                Err(e) => eprintln!("{} status error: {:?}", m.name, e),
            },
            Err(e) => eprintln!("error with {src}: {:#?}", e),
        }
    }
    println!("done");
}
