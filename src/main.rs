use futures_util::StreamExt;
use log::*;
use std::cmp::min;
use std::io::Write;
use structopt::StructOpt;

mod api;
mod config;

use api::*;
use config::*;

async fn search_mods(ctx: &AppContext, search_args: &SearchArgs) -> anyhow::Result<SearchResponse> {
    println!("Searching with query \"{}\"...", search_args.package_name);

    let client = reqwest::Client::new();
    let url = format!("https://{}/v2/search", ctx.config.upstream.server_address);

    let mut params = vec![("query", search_args.package_name.to_owned())];
    if let Some(versions) = &search_args.version {
        let versions_facets = versions
            .iter()
            .map(|e| format!("[\"versions:{}\"]", e))
            .collect::<Vec<String>>()
            .join(",");
        params.push(("facets", format!("[{}]", versions_facets)));
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
) -> anyhow::Result<Vec<usize>> {
    let input: String = dialoguer::Input::new()
        .with_prompt("Mods to install (eg: 1 2 3-5)")
        .interact_text()?;

    let mut selected: Vec<usize> = Vec::new();
    for token in input.split(" ") {
        let terms: Vec<&str> = token.split("-").collect();

        match terms.len() {
            1 => selected.push(terms[0].parse().expect("Token must be an integer")),
            2 => {
                let terms: Vec<usize> = terms
                    .iter()
                    .map(|term| term.parse().expect("Term must be an integer"))
                    .collect();
                let from = terms[0];
                let to = terms[1];

                for index in from..=to {
                    selected.push(index);
                }
            }
            _ => panic!("Invalid selection token {}", token),
        }
    }

    selected.dedup();

    let selected = selected
        .iter()
        .map(|index| {
            if *index < 1 || *index > response.hits.len() {
                // TODO return useful error instead of panicking
                panic!("Index {} is out of bounds", index);
            }

            // input is indexed from 1, but results are indexed from 0
            let index = index - 1;

            index
        })
        .collect();

    Ok(selected)
}

async fn fetch_mod_info(ctx: &AppContext, mod_result: &ModResult) -> anyhow::Result<ModInfo> {
    let mod_id = &mod_result.project_id;
    println!(
        "Fetching mod info for {} (ID: {})...",
        mod_result.title, mod_id
    );

    let client = reqwest::Client::new();
    let url = format!(
        "https://{}/v2/project/{}",
        ctx.config.upstream.server_address, mod_id
    );
    info!("GET {}", url);
    let response = client.get(url).send().await?;
    let response = response.json::<ModInfo>().await?;
    Ok(response)
}

async fn fetch_mod_version(ctx: &AppContext, version_id: &String) -> anyhow::Result<ModVersion> {
    println!("Fetching mod version {}...", version_id);

    let client = reqwest::Client::new();
    let url = format!(
        "https://{}/v2/version/{}",
        ctx.config.upstream.server_address, version_id
    );
    info!("GET {}", url);
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
    info!("GET {}", url);
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

    for selection in selected.iter() {
        let to_get = &response.hits[*selection];
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
