use log::{debug, info};
use reflector::{
    display_systime, CaptureList, Config, GetError, Mirror, MirrorStatus, StatusError,
};
use std::env;

// why ?
#[allow(dead_code)]
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
                println!("fetching mirror {}", m.name);
                match m.fill_loop() {
                    Ok(l) => Ok(GetMirrorResult { captures: Some(l) }),
                    Err(e) => Err(RealGetError(e)),
                }
            } else {
                println!(
                    "mirror {} is already full for the default loop period",
                    m.name
                );
                Ok(GetMirrorResult {
                    captures: Some(m.loop_captures()),
                })
            }
        }
        Err(e) => Err(RealStatusError(e)),
    }
}

fn main() {
    std_logger::Config::logfmt().init();
    let cfg = Config::try_from(env::args()).expect("error with args");
    for src in cfg.sources {
        debug!("{:#?}", src);
        match Mirror::new(src) {
            Ok(m) => {
                info!("got mirror {m}");
                match get_mirror(m) {
                    Ok(r) if r.captures.is_none() => {
                        println!("no captures in our loop period");
                    }
                    Ok(r) => {
                        let cap = r.captures.unwrap();
                        println!("mirror has {}", cap);
                        let l = cap.latest().unwrap();
                        println!(
                            "latest stamped {} file {}",
                            display_systime(&l.time),
                            l.path.display(),
                        );
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
