use reflector::{Config, Mirror};

fn main() {
    let cfg: Config = Default::default();
    println!("{:#?}", cfg);
    for src in cfg.sources {
        println!("{:#?}", src);
        let mut r = Mirror::new(src);
    }
}
