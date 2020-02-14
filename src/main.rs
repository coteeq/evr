extern crate lazy_static;

use clap::{ AppSettings, App, Arg };
use env_logger;
use log::{ error };
use std::io::prelude::*;

mod conf;
mod backends;
mod wait;

fn main() {
    let matches = App::new("evr")
        .version("0.1")
        .author("syn")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(Arg::with_name("src")
            .required(true)
            .index(1)
            .help("source file")
        )
        .arg(Arg::with_name("time")
            .short("t")
            .long("time")
            .help("show wall time")
        )
        .arg(Arg::with_name("mem")
            .short("m")
            .long("mem")
            .help("show mem usage (rss)")
        )
        .get_matches();


    env_logger::builder()
        .format_timestamp(None)
        .init();

    let src_path: std::path::PathBuf = matches.value_of("src")
                                              .expect("src is required")
                                              .into();
    let config = conf::get_conf();

    if src_path.exists() {
        if let Some(backend) = config.get_backend(&src_path) {
            match backend.run(&src_path) {
                Ok(status) => {
                    if matches.is_present("time") {
                        println!("wall time: {:?}", status.wall_time);
                    }
                    if matches.is_present("mem") {
                        println!("rss: {}K", status.usage.ru_maxrss);
                    }
                },
                Err(err) => error!("{}", err)
            };
        } else {
            error!("could not match backend");
        }
    } else {
        let template = config.get_template(&src_path).as_bytes();
        if let Err(err) =
            std::fs::File::create(&src_path)
                .and_then(|mut file| file.write_all(template)) {
            error!("{}", err);
        }
    };
}
