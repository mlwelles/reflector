use reflector::{Config, Mirror};
use std::env;

fn main() {
    let cfg = Config::from(env::args());
    for src in cfg.sources {
        println!("{:#?}", src);
        match Mirror::new(src) {
            Ok(r) => {
                println!("ok, {}", r)
            }
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }
}
