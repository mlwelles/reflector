use reflector::{Config, Mirror};
use std::str::FromStr;

fn main() {
    let mut args = std::env::args();
    let cfg = match args.len() {
        1 => Config::default(),
        2 => {
            if let Some(first) = args.nth(1) {
                match Config::from_str(&first) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("no matches for {}: {:?}", first, e);
                        return;
                    }
                }
            } else {
                eprintln!("arg counting logic fail");
                return;
            }
        }
        _ => {
            eprintln!("unimplemented");
            Config::default()
        }
    };

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
