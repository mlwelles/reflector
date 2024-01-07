use reflector::time_util::*;
use reflector::{CaptureList, Config, GetError, Mirror, MirrorStatus, StatusError};
use std::env;

#[derive(Debug)]
enum GetMirrorError {
    Unimplemented,
    RealStatusError(StatusError),
    RealGetError(GetError),
}
use GetMirrorError::*;

#[derive(Debug)]
struct GetMirrorResult {
    captures: Option<CaptureList>,
}

fn get_mirror(mut m: Mirror) -> Result<GetMirrorResult, GetMirrorError> {
    match m.status() {
        Ok(s) => {
            let do_get = match s {
                MirrorStatus::Empty => {
                    eprintln!("empty, we should get");
                    true
                }
                MirrorStatus::Partial(t) => {
                    eprintln!("partial since {}, we should get", display_systime(t));
                    true
                }
                MirrorStatus::Full(t) => {
                    eprintln!(
                        "full, our latest time is {}, nothing to get",
                        display_systime(t)
                    );
                    false
                }
                MirrorStatus::Unimplemented => return Err(Unimplemented),
            };
            if do_get {
                match m.fill_loop() {
                    Ok(l) => Ok(GetMirrorResult { captures: Some(l) }),
                    Err(e) => Err(RealGetError(e)),
                }
            } else {
                Ok(GetMirrorResult { captures: None })
            }
        }
        Err(e) => Err(RealStatusError(e)),
    }
}

fn main() {
    let cfg = Config::from(env::args());
    for src in cfg.sources {
        println!("{:#?}", src);
        match Mirror::new(src) {
            Ok(m) => {
                println!("ok, {}", m);
                match get_mirror(m) {
                    Ok(c) => {
                        println!("now we have this capturelist: {:?}", c);
                    }
                    Err(e) => {
                        eprintln!("filling loop captures failed: {:?}", e);
                    }
                }
            }
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }
}
