use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Install { package_name: String },
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Remove { package_name: String },
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Update,
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Clean,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "hopper", setting = structopt::clap::AppSettings::ColoredHelp)]
struct Args {
    /// Path to configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    /// Path to mod lockfile
    #[structopt(short, long, parse(from_os_str))]
    lockfile: Option<PathBuf>,

    #[structopt(subcommand)]
    command: Command,
}

impl Args {
    fn default_paths(&mut self) {

    }
}

#[derive(Deserialize, Debug)]
struct Upstream {
    /// Modrinth main server URL
    server_url: String, // TODO use URL?
}

#[derive(Deserialize, Debug)]
struct Config {
    /// Configuration for the upstream Modrinth server
    upstream: Upstream,
}

fn main() {
    let args = Args::from_args();
    println!("{:#?}", args);
}
