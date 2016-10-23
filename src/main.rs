extern crate clap;
extern crate nanomsg;
extern crate rustc_serialize;

use std::path::PathBuf;
use std::io::{BufWriter, Write};

use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};
use nanomsg::{Socket, Protocol};
use rustc_serialize::json;

#[derive(Debug, RustcEncodable)]
struct CopyJob {
    src: String,
    dst: String,
}


impl CopyJob {
    fn new(src: &str, dst: &str) -> CopyJob {
        CopyJob { src: src.to_string(), dst: dst.to_string() }
    }
}


fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new("ECP - extended copy")
        .version("0.1")
        .about("Copies things obviously")
        .author("Lukas Nemec <lu.nemec@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add")
            .about("add file to copy queue")
            .arg(
                Arg::with_name("src")
                .help("source path to existing file or directory")
                .required(true)
                .validator(validate_path)
                .index(1)
            )
            .arg(
                Arg::with_name("dst")
                .help("destination path to copy the source to")
                .required(true)
                .index(2)
            )
        )
        .subcommand(
            SubCommand::with_name("status")
            .about("prints current copy status")
        )
}


fn validate_path(p: String) -> Result<(), String> {
    let path = PathBuf::from(&p);
    if !path.exists() {
        return Err(format!("{} isn't a valid file.", &p));
    }
    Ok(())
}


fn add_to_queue(job: CopyJob) {
    let json = json::encode(&job).unwrap();

    let mut socket = Socket::new(Protocol::Push).unwrap();
    socket.connect("ipc:///tmp/pipeline.ipc");
    socket.write_all(json.as_bytes());
}


fn main() {
    let mut app = build_app();
    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("add") {
        let src = matches.value_of("src").unwrap();
        let dst = matches.value_of("dst").unwrap();
        add_to_queue(CopyJob::new(src, dst));
    } else if let Some(ref matches) = matches.subcommand_matches("status") {
        println!("Copy status");
    } else {
        println!("print help!");
    }

}
