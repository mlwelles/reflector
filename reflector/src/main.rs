mod reflect;
use reflect::Config;

fn main() {
    let cfg: Config = Default::default();
    println!("{:#?}", cfg);
}
