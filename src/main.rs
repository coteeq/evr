use clap::AppSettings;
use structopt::StructOpt;
use env_logger;
use log::{ trace, error };

mod conf;
mod backends;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "evr",
    version = "0.1",
    author = "by syn",
    global_settings = &[
        AppSettings::ColoredHelp,
        AppSettings::UnifiedHelpMessage
    ],
)]
struct EVROpts {
    /// source filename
    #[structopt(long, index = 1)]
    src: String,

    /// be quiet
    #[structopt(short, long)]
    quiet: bool,

    /// optimize with this level
    #[structopt(short, long)]
    opt: Option<u8>,

    /// show time (wall)
    #[structopt(short, long)]
    time: bool,
    
    /// show mem usage (rss)
    #[structopt(short, long)]
    mem: bool
}

fn main() {
    let opts = EVROpts::from_args();
    if !opts.quiet {
        env_logger::builder()
            .format_timestamp(None)
            .init();
    }

    trace!("{:#?}", opts);

    let src_path: std::path::PathBuf = opts.src.into();
    let config = conf::get_conf();

    trace!("{:#?}", config);

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
