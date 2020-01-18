extern crate lazy_static;

use clap::{ AppSettings, App, Arg };
use env_logger;
use log::{ trace, error };

mod conf;
mod backends;

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

    let src_path: std::path::PathBuf = matches.value_of("src").expect("src is required").into();
    let config = conf::get_conf();

    let result =
        if src_path.exists() {
            config.run(&src_path)
        } else {
            config.make(&src_path)
        };

    match result {
        Ok(_) => trace!("ok"),
        Err(err) => error!("{}", err)
    }
}
