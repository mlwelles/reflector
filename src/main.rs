use reflector::{Config, Mirror};

fn main() {
    let cfg: Config = Default::default();
    println!("{:#?}", cfg);
    for src in cfg.sources {
        println!("{:#?}", src);
        match Mirror::new(src) {
            Ok(r) => {
                println!("ok, {:#?}", r)
            }
            Err(e) => eprintln!("error: {:#?}", e),
        }
    }
}
