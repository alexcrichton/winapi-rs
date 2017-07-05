extern crate ctest;
extern crate toml;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("windows.h");

    let mut toml = String::new();
    File::open("../Cargo.toml").unwrap().read_to_string(&mut toml).unwrap();
    let toml: toml::Value = toml::from_str(&toml).unwrap();
    for (k, _v) in toml["features"].as_table().unwrap().iter() {
        cfg.cfg("feature", Some(k));
    }

    cfg.type_name(|t, is_struct| {
        if is_struct {
            format!("struct {}", t)
        } else if t.starts_with("__uint") {
            format!("uint{}_t", &t[6..])
        } else {
            t.to_string()
        }
    });
    cfg.generate("../src/lib.rs", "all.rs");
}
