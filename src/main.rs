use futures_util::StreamExt;
use log::*;
use std::cmp::min;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod api;
mod config;
mod hopfile;

use api::*;
use config::*;
use hopfile::*;

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
async fn select_from_results(
    _ctx: &AppContext,
    response: &SearchResponse,
) -> anyhow::Result<Vec<ModResult>> {
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

    Ok(selected
        .iter()
        .map(|i| response.hits[*i].to_owned())
        .collect())
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

async fn download_version_file(
    ctx: &AppContext,
    target_dir: &Path,
    file: &ModVersionFile,
) -> anyhow::Result<()> {
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

    let filename = target_dir.join(&file.filename);
    let mut file = std::fs::File::create(&filename)?;
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

    pb.finish_with_message(&format!("Downloaded {} to {:#?}", url, filename));
    Ok(())
}

async fn search_and_select(
    ctx: &AppContext,
    search_args: &SearchArgs,
) -> anyhow::Result<Vec<ModResult>> {
    let response = search_mods(ctx, &search_args).await?;

    if response.hits.is_empty() {
        // TODO formatting
        println!("No results; nothing to do...");
        return Ok(vec![]);
    }

    display_search_results(ctx, &response);
    let selected = select_from_results(ctx, &response).await?;

    Ok(selected)
}

async fn download_mods(
    ctx: &AppContext,
    target_dir: Option<&Path>,
    mods: &Vec<ModResult>,
) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let target_dir = target_dir.unwrap_or(&current_dir);
    for to_get in mods.iter() {
        let mod_info = fetch_mod_info(ctx, to_get).await?;

        // TODO allow the user to select multiple versions
        // TODO select the most recent version matching search_args.versions
        if let Some(version_id) = mod_info.versions.first() {
            println!("fetching version {}", version_id);

            let version = fetch_mod_version(ctx, version_id).await?;
            for file in version.files.iter() {
                download_version_file(ctx, &target_dir, file).await?;
            }
        }
    }

    Ok(())
}

async fn cmd_get(ctx: &AppContext, search_args: SearchArgs) -> anyhow::Result<()> {
    let selected = search_and_select(ctx, &search_args).await?;
    if selected.is_empty() {
        // TODO formatting
        println!("No packages selected; nothing to do...");
        return Ok(());
    }

    // TODO arg for target directory?
    download_mods(ctx, None, &selected).await?;

    Ok(())
}

async fn cmd_update(ctx: &AppContext, instance_dir: Option<PathBuf>) -> anyhow::Result<()> {
    let instance_dir = instance_dir.unwrap_or(std::env::current_dir()?);
    let hopfile_path = instance_dir.join("Hopfile.toml");
    let hopfile = std::fs::read_to_string(hopfile_path)?;
    let hopfile: Hopfile = toml::from_str(&hopfile)?;

    println!("hopfile: {:#?}", hopfile);

    for (name, entry) in hopfile.mods.iter() {
        let search_args = SearchArgs {
            package_name: name.to_string(),
            version: Some(vec![hopfile.version.to_owned()]),
        };

        let response = search_mods(ctx, &search_args).await?;

        if response.hits.is_empty() {
            error!("No results for {}; skipping update...", name);
            continue;
        }

        display_search_results(ctx, &response);
        let selected = loop {
            let selected = select_from_results(ctx, &response).await?;
            if selected.is_empty() {
                use dialoguer::Confirm;
                let prompt = format!("Really skip updating {}?", name);
                let confirm = Confirm::new().with_prompt(prompt).interact()?;
                if !confirm {
                    println!("Skipping updating {}...", name);
                } else {
                    continue;
                }
            }

            break selected;
        };

        // TODO update Hopfile.toml with specific versions using toml_edit crate
        download_mods(ctx, Some(&instance_dir), &selected).await?;
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
        Command::Update { instance_dir } => cmd_update(&ctx, instance_dir).await,
        _ => unimplemented!("unimplemented subcommand"),
    }
}
