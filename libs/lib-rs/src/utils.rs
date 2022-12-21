extern crate web_sys;

use crate::log;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    log!("Panic hook set");
}

pub fn get_path(year: u32, day: u8, bigboy: bool) -> String {
    use std::path::PathBuf;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("data/");

    if bigboy {
        format!("{}/{}/.bigboy/{}.txt", d.to_str().unwrap(), year, day)
    } else {
        format!("{}/{}/{}.txt", d.to_str().unwrap(), year, day)
    }
}

#[allow(unused)]
pub fn read_to_string(year: u32, day: u8, bigboy: bool) -> String {
    let data_path = get_path(year, day, bigboy);
    let mut fp =
        File::open(&data_path).unwrap_or_else(|_| panic!("Cannot open file at {data_path}"));
    let mut buf = String::new();
    fp.read_to_string(&mut buf).unwrap();
    buf.trim_end().to_string()
}
