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
                MirrorStatus::Empty(_) => true,
                MirrorStatus::Partial(_) => true,
                MirrorStatus::Full(_) => false,
                MirrorStatus::Unimplemented => return Err(Unimplemented),
            };
            if do_get {
                match m.fill_loop() {
                    Ok(l) => Ok(GetMirrorResult { captures: Some(l) }),
                    Err(e) => Err(RealGetError(e)),
                }
            } else {
                // already full
                Ok(GetMirrorResult {
                    captures: Some(m.loop_captures()),
                })
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
                    Ok(r) if r.captures.is_none() => {
                        println!("no captures in our loop period");
                    }
                    Ok(r) => {
                        println!("now we have this capturelist: {}", r.captures.unwrap());
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
