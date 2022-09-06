use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

// TODO parameter to restrict target Minecraft version
#[derive(StructOpt, Clone, Debug)]
pub struct SearchArgs {
    pub package_name: String,

    /// Restricts the target Minecraft version
    #[structopt(short, long)]
    pub version: Option<Vec<String>>,
}

// TODO use ColoredHelp by default?
#[derive(StructOpt, Clone, Debug)]
pub enum Command {
    /// Adds a mod to the current instance
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Add(SearchArgs),
    /// Removes a mod
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Remove { package_name: String },
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Get(SearchArgs),
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Update,
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    Clean,
}

// TODO move main body argument fields to substruct for ease of moving?
#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "hopper", setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Args {
    /// Path to configuration file
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Path to mod lockfile
    #[structopt(short, long, parse(from_os_str))]
    pub lockfile: Option<PathBuf>,

    /// Auto-accept confirmation dialogues
    #[structopt(short = "y", long = "yes")]
    pub auto_accept: bool,

    #[structopt(subcommand)]
    pub command: Command,
}

impl Args {
    pub fn load_config(&self) -> Result<Config, confy::ConfyError> {
        if let Some(config_path) = &self.config {
            confy::load_path(config_path)
        } else {
            confy::load("hopper")
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Upstream {
    /// Modrinth main server address
    pub server_address: String,
}

impl Default for Upstream {
    fn default() -> Self {
        Self {
            server_address: "api.modrinth.com".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Options {
    /// Whether to reverse search results
    pub reverse_search: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            reverse_search: true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Config {
    /// General settings
    pub options: Options,

    /// Configuration for the upstream Modrinth server
    pub upstream: Upstream,
}

pub struct AppContext {
    pub args: Args,
    pub config: Config,
}
