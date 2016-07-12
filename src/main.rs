extern crate argparse;

use std::path::Path;

use argparse::{ArgumentParser, StoreTrue, Store};

mod check;

fn parse_args() {
    let mut src_path = String::new();
    let mut dst_path = String::new();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("ecp (extended copy), a always copying utility.");
        parser.refer(&mut src_path)
            .add_argument("src_path", Store, "Source path")
            .required();
        parser.refer(&mut dst_path)
            .add_argument("src_path", Store, "Source path")
            .required();
        parser.parse_args_or_exit();
    }
    println!("{} -> {}", &src_path, &dst_path);
}


fn main() {
    parse_args();

    let src = Path::new("/home/");
    let checks = check::check_before_copy(&src);
    println!("Checky? {}", checks);
}
