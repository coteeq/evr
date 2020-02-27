extern crate lazy_static;

use clap::{ AppSettings, App, Arg };
use env_logger;
use log::{ error };
use std::io::prelude::*;

mod conf;
mod backends;
mod wait;
mod serde_duration;

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

    let config = match conf::get_conf() {
        Ok(c) => c,
        Err(err) => {
            error!("could not load config: {}", err);
            std::process::exit(64); // EX_USAGE (BSD sysexits(3))
        }
    };

    if src_path.exists() {
        if let Some(backend) = config.get_backend(&src_path) {
            match backend.run(&src_path) {
                Ok(status) => {
                    // print to stderr to allow `>/dev/null` for user programs
                    if matches.is_present("time") {
                        eprintln!("wall time: {:?}", status.wall_time);
                    }
                    if matches.is_present("mem") {
                        eprintln!("rss: {}K", status.usage.get_rss_bytes() / 1000);
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
