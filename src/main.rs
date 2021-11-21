use console::style;
use serde::{Deserialize, Serialize};
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
    fn load_config(&self) -> Result<Config, confy::ConfyError> {
        if let Some(config_path) = &self.config {
            confy::load_path(config_path)
        } else {
            confy::load("hopper")
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Upstream {
    /// Modrinth main server address
    server_address: String,
}

impl Default for Upstream {
    fn default() -> Self {
        Self {
            server_address: "api.modrinth.com".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Config {
    /// Configuration for the upstream Modrinth server
    upstream: Upstream,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    hits: Vec<ModResult>,
    offset: isize,
    limit: isize,
    total_hits: isize,
}

#[derive(Deserialize, Debug)]
struct ModResult {
    mod_id: String,
    project_type: Option<String>, // NOTE this isn't in all search results?
    author: String,
    title: String,
    description: String,
    categories: Vec<String>,
    versions: Vec<String>,
    downloads: isize,
    page_url: String,
    icon_url: String,
    author_url: String,
    date_created: String,
    date_modified: String,
    latest_version: String,
    license: String,
    client_side: String,
    server_side: String,
    host: String,
}

impl ModResult {
    fn format_info(&self) -> String {
        let title = style(self.title.clone()).bold();
        let downloads = style(self.downloads.clone()).bold().green();
        if let Some(latest_release) = self.versions.last() {
            // TODO fetch version numbers to display
            let latest_release = style(latest_release).bold().blue();
            format!("{} [{}] ({} downloads)", title, latest_release, downloads)
        } else {
            format!("{} [no releases]", title)
        }
    }

    fn format_description(&self) -> String {
        self.description.to_owned()
    }

    fn display(&self, index: usize) {
        let index = style(index).magenta();
        let info = self.format_info();
        let description = self.format_description();
        println!("{} {}\n    {}", index, info, description);
    }
}

async fn cmd_install(config: &Config, package_name: String) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let url = format!("https://{}/api/v1/mod", config.upstream.server_address);
    let params = [("query", package_name.as_str())];
    let url = reqwest::Url::parse_with_params(url.as_str(), &params)?;
    let response = client
        .get(url)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;
    for (i, result) in response.hits.iter().enumerate().rev() {
        result.display(i + 1);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    let config = args.load_config()?;
    match args.command {
        Command::Install { package_name } => cmd_install(&config, package_name).await,
        _ => unimplemented!("unimplemented subcommand"),
    }
}
