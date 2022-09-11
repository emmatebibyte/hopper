use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// TODO parameter to restrict target Minecraft version
#[derive(clap::Args, Clone, Debug)]
pub struct SearchArgs {
    pub package_name: String,

    /// Type of package to use
    #[clap(short, long, value_enum)]
    pub package_type: Option<PackageType>,

    /// Restricts the target Minecraft version
    #[clap(short, long)]
    pub version: Option<Vec<String>>,
}

// TODO use ColoredHelp by default?
#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Adds a mod to the current instance
    Add(SearchArgs),
    /// Removes a mod
    Remove {
        package_name: String,
    },
    Get(SearchArgs),
    Update,
    Clean,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PackageType {
    Fabric,
    Forge,
    Quilt,
    Resource,
    Pack,
}

impl ToString for PackageType {
    fn to_string(&self) -> String {
        match self {
            PackageType::Fabric => "fabric",
            PackageType::Forge => "forge",
            PackageType::Quilt => "quilt",
            PackageType::Resource => "resourcepack",
            PackageType::Pack => "modpack",
        }
        .to_string()
    }
}

// TODO move main body argument fields to substruct for ease of moving?
#[derive(Parser, Clone, Debug)]
#[clap(name = "hopper")]
pub struct Args {
    /// Path to configuration file
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,

    /// Path to mod lockfile
    #[clap(short, long, value_parser)]
    pub lockfile: Option<PathBuf>,

    /// Auto-accept confirmation dialogues
    #[clap(short = 'y', long = "yes")]
    pub auto_accept: bool,

    #[clap(subcommand)]
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
