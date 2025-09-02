//! Display the captures for the provided mirror

use reflector::{Config, Mirror};
use std::env;

fn captures(cfg: &Config, m: &Mirror) {
    match m.loop_period_timerange(&cfg.loops) {
        Ok(per) => {
            let cc = m.captures_in_range(&per);
            println!("{} captures in {} periods:", cc.len(), cfg.loops);
            for c in cc {
                println!("{c}");
            }
        }
        Err(e) => eprintln!("error time range for {} loop: {:?}", cfg.loops, e),
    }
}

fn main() {
    let cfg = Config::try_from(env::args()).unwrap();
    for src in cfg.sources.inner().iter() {
        match Mirror::new(src.clone()) {
            Ok(m) => captures(&cfg, &m),
            Err(e) => eprintln!("error with {src}: {:#?}", e),
        }
    }
    println!("done");
}
