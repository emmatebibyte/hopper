use console::style;
use log::*;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

// TODO parameter to restrict target Minecraft version
#[derive(StructOpt, Clone, Debug)]
struct SearchArgs {
    package_name: String,

    /// Restricts the target Minecraft version
    #[structopt(short, long)]
    version: Option<Vec<String>>,
}

// TODO use ColoredHelp by default?
#[derive(StructOpt, Clone, Debug)]
enum Command {
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
struct Args {
    /// Path to configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    /// Path to mod lockfile
    #[structopt(short, long, parse(from_os_str))]
    lockfile: Option<PathBuf>,

    /// Auto-accept confirmation dialogues
    #[structopt(short = "y", long = "yes")]
    auto_accept: bool,

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

#[derive(Deserialize, Serialize, Debug)]
struct Options {
    /// Whether to reverse search results
    reverse_search: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            reverse_search: true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Config {
    /// General settings
    options: Options,

    /// Configuration for the upstream Modrinth server
    upstream: Upstream,
}

struct AppContext {
    pub args: Args,
    pub config: Config,
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
    mod_id: String,               // TODO parse to `local-xxxxx` with regex
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
        println!("{:>2} {}\n    {}", index, info, description);
    }
}

#[derive(Deserialize, Debug)]
struct ModInfo {
    id: String, // TODO serialize mod id?
    slug: String,
    team: String, // TODO serialize team id?
    title: String,
    description: String,
    body: String,
    published: String, // TODO serialize datetime
    updated: String,   // TODO serialize datetime
    status: String,
    license: License,
    client_side: String, // TODO serialize as enum
    server_side: String, // TODO serialize as enum
    downloads: isize,
    followers: isize,
    categories: Vec<String>,
    versions: Vec<String>,
    icon_url: Option<String>,
    issues_url: Option<String>,
    source_url: Option<String>,
    wiki_url: Option<String>,
    discord_url: Option<String>,
    donation_urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct License {
    id: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct ModVersion {
    id: String,        // version id
    mod_id: String,    // mod id
    author_id: String, // user id
    // NOTE modrinth docs list this as a String, but is actually a bool?
    // featured: String,  // user id
    name: String,
    version_number: String,
    changelog: Option<String>,
    changelog_url: Option<String>,
    date_published: String, // TODO serialize datetime
    downloads: isize,
    version_type: String, // TODO {alpha | beta | release}
    files: Vec<ModVersionFile>,
    dependencies: Vec<String>, // TODO dependency wrangling, thank you modrinth, very cool
    game_versions: Vec<String>,
    loaders: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ModVersionFile {
    hashes: HashMap<String, String>,
    url: String,
    filename: String,
}

async fn search_mods(ctx: &AppContext, search_args: &SearchArgs) -> anyhow::Result<SearchResponse> {
    let client = reqwest::Client::new();
    let url = format!("https://{}/api/v1/mod", ctx.config.upstream.server_address);

    let mut params = vec![("query", search_args.package_name.to_owned())];
    if let Some(versions) = &search_args.version {
        params.push(("versions", versions.join(",")));
    }

    let url = reqwest::Url::parse_with_params(url.as_str(), &params)?;
    info!("GET {}", url);
    let response = client
        .get(url)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;
    Ok(response)
}

fn display_search_results(ctx: &AppContext, response: &SearchResponse) {
    let iter = response.hits.iter().enumerate();
    if ctx.config.options.reverse_search {
        for (i, result) in iter.rev() {
            result.display(i + 1);
        }
    } else {
        for (i, result) in iter {
            result.display(i + 1);
        }
    }
}

// TODO implement enum for more graceful exiting
async fn select_from_results<'a>(
    _ctx: &AppContext,
    response: &'a SearchResponse,
) -> anyhow::Result<Vec<&'a ModResult>> {
    let input: String = dialoguer::Input::new()
        .with_prompt("Mods to install (eg: 1 2 3)")
        .interact_text()?;

    let mut selected: Vec<usize> = Vec::new();
    for token in input.split(" ") {
        // TODO range input (eg: 1-3)
        let index: usize = token.parse().expect("Token must be an integer");
        if index < 1 || index > response.hits.len() {
            // TODO return useful error instead of panicking
            panic!("Index {} is out of bounds", index);
        }

        // input is indexed from 1, but results are indexed from 0
        let index = index - 1;

        if !selected.contains(&index) {
            selected.push(index);
        } else {
            // TODO make this a proper warning log message
            println!("warning: repeated index {}", index);
        }
    }

    Ok(selected.iter().map(|i| &response.hits[*i]).collect())
}

async fn fetch_mod_info(ctx: &AppContext, mod_result: &ModResult) -> anyhow::Result<ModInfo> {
    let client = reqwest::Client::new();
    let mod_id = &mod_result.mod_id;
    let mod_id = mod_id[6..].to_owned(); // Remove "local-" prefix
    let url = format!(
        "https://{}/api/v1/mod/{}",
        ctx.config.upstream.server_address, mod_id
    );
    let response = client.get(url).send().await?;
    let response = response.json::<ModInfo>().await?;
    Ok(response)
}

async fn fetch_mod_version(ctx: &AppContext, version_id: &String) -> anyhow::Result<ModVersion> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://{}/api/v1/version/{}",
        ctx.config.upstream.server_address, version_id
    );
    let response = client.get(url).send().await?;
    let response = response.json::<ModVersion>().await?;
    Ok(response)
}

async fn download_version_file(ctx: &AppContext, file: &ModVersionFile) -> anyhow::Result<()> {
    // TODO replace all uses of .unwrap() with proper error codes
    let filename = &file.filename;

    // TODO make confirmation skippable with flag argument
    if !ctx.args.auto_accept {
        use dialoguer::Confirm;
        let prompt = format!("Download to {}?", filename);
        let confirm = Confirm::new()
            .with_prompt(prompt)
            .default(true)
            .interact()?;
        if !confirm {
            println!("Skipping downloading {}...", filename);
            return Ok(());
        }
    }

    let client = reqwest::Client::new();
    let url = &file.url;
    let response = client.get(url).send().await?;
    let total_size = response.content_length().unwrap();

    // TODO better colors and styling!
    // TODO square colored creeper face progress indicator (from top-left clockwise spiral in)
    use indicatif::{ProgressBar, ProgressStyle};
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar().template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").progress_chars("#>-"));
    pb.set_message(&format!("Downloading {}", url));

    let filename = &file.filename;
    let mut file = std::fs::File::create(filename)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    // TODO check hashes while streaming
    while let Some(item) = stream.next().await {
        let chunk = &item.unwrap();
        file.write(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, filename));
    Ok(())
}

async fn cmd_get(ctx: &AppContext, search_args: SearchArgs) -> anyhow::Result<()> {
    let response = search_mods(ctx, &search_args).await?;

    if response.hits.is_empty() {
        // TODO formatting
        println!("No results; nothing to do...");
        return Ok(());
    }

    display_search_results(ctx, &response);
    let selected = select_from_results(ctx, &response).await?;

    if selected.is_empty() {
        // TODO formatting
        println!("No packages selected; nothing to do...");
        return Ok(());
    }

    for to_get in selected.iter() {
        let mod_info = fetch_mod_info(ctx, to_get).await?;

        // TODO allow the user to select multiple versions
        if let Some(version_id) = mod_info.versions.first() {
            println!("fetching version {}", version_id);

            let version = fetch_mod_version(ctx, version_id).await?;
            for file in version.files.iter() {
                download_version_file(ctx, file).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::from_args();
    let config = args.load_config()?;
    let ctx = AppContext { args, config };
    match ctx.args.to_owned().command {
        Command::Get(search_args) => cmd_get(&ctx, search_args).await,
        _ => unimplemented!("unimplemented subcommand"),
    }
}
