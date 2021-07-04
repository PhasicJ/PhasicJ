use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = std::env!("PHASICJ_VERSION"), author = "David Johnston <dwtj@dwtj.me>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    pub database: String,

    #[clap(short, long)]
    pub socket: String,
}
