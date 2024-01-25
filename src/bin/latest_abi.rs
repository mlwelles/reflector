use reflector::{Mirror, MirrorStatus, SourceConfig};
// use std::env;
use std::os;

fn process_latest(c: Capture) {
    // we need to scale this enormous PNG
    // use these rectangle coordinates:
    // (6212,3584)------------\
    // |                      |
    // \-----------(10376,5916)
    // which is: w 4164
    //           h 2332
    // fixme: adjust as needed to hit 16:9 exactly
    // scale this clipped area to 2k
}

fn main() {
    match Mirror::try_from(SourceConfig::goes_abi()) {
        Ok(mut m) => match m.status() {
            Ok(MirrorStatus::Full(_) | MirrorStatus::Partial(_)) => eprintln!("ok to proceed"),
            Err(e) => {
                eprintln!("mirror {} status error: {:?}", m.name, e);
                os::exit(2);
            }
        },
        Err(e) => {
            eprintln!("error setting up ABI mirror: {:?}", e);
            os::exit(1);
        }
    }
}
