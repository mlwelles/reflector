// use reflector::reflect::Config;
use toml;

fn basic_toml() -> &'static str {
    r#"
[[sources]]
name      = "Solar Data Observatory"
remote	  = "https://sdo.gsfc.nasa.gov/assets/img/dailymov"
pathmaker = "SDO"
local     = "/home/adam/tmp/sat/sdo"
freq      = "Daily"
flatten   = true

[[sources]]
name      = "GOES ABI_TrueColor"
remote    = "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor"
pathmaker = "GOES"
local     = "/home/adam/tmp/sat/abi_truecolor"
freq      = "Momentarily"
periodmin = 10
    "#
}

#[test]
fn test_basic_toml() {
    let basic: Config = toml::from_str(basic_toml());
    basic.unwrap();
}
