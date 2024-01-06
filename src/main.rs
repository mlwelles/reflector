use reflector::time_util::*;
use reflector::{Config, Mirror, MirrorStatus, StatusError};
use std::env;

fn get_mirror(mut m: Mirror) -> Result<MirrorStatus, StatusError> {
    match m.status() {
        Ok(s) => {
            match s {
                MirrorStatus::Empty => eprintln!("empty, we should get"),
                MirrorStatus::Partial(t) => {
                    eprintln!("partial since {}, we should get", display_systime(t))
                }
                MirrorStatus::Full(t) => eprintln!(
                    "full, our latest time is {}, nothing to get",
                    display_systime(t)
                ),
                MirrorStatus::Unimplemented => return Err(StatusError::Unimplemented),
            }
            Ok(s)
        }
        Err(e) => Err(e),
    }
}

fn main() {
    let cfg = Config::from(env::args());
    for src in cfg.sources {
        println!("{:#?}", src);
        match Mirror::new(src) {
            Ok(m) => {
                println!("ok, {}", m);
                get_mirror(m).unwrap();
            }
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }
}
