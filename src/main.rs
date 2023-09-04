use reflector::Config;

fn main() {
    let cfg: Config = Default::default();
    println!("{:#?}", cfg);
    for src in cfg.sources {
        println!("{:#?}", src);
        // let mut r = DataSource::from_config(src);
    }
}
