use reflector::Config;

fn basic_toml() -> &'static str {
    r#"
[[sources]]
name      = "Solar Data Observatory"
abbrev    = "sdo"
remote	  = "https://sdo.gsfc.nasa.gov/assets/img/dailymov"
pathmaker = "SDO"
local     = "/tmp"
period    = 86400
flatten   = true

[[sources]]
name      = "GOES ABI_TrueColor"
abbrev    = "goesabi"
remote    = "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor"
pathmaker = "GOES"
local     = "/tmp"
period    = 600
    "#
}

#[test]
fn test_basic_toml() {
    let _basic: Config = toml::from_str(basic_toml()).unwrap();
}
