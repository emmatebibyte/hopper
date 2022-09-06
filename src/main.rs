mod api;
mod client;
mod config;

use api::*;
use clap::Parser;
use client::*;
use config::*;

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

async fn cmd_get(ctx: &AppContext, search_args: SearchArgs) -> anyhow::Result<()> {
    let client = HopperClient::new(ctx.config.clone());
    let response = client.search_mods(&search_args).await?;

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
        let mod_info = client.fetch_mod_info(to_get).await?;

        // TODO allow the user to select multiple versions
        if let Some(version_id) = mod_info.versions.first() {
            println!("fetching version {}", version_id);

            let version = client.fetch_mod_version(version_id).await?;
            for file in version.files.iter() {
                client.download_version_file(&ctx.args, file).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config = args.load_config()?;
    let ctx = AppContext { args, config };
    match ctx.args.to_owned().command {
        Command::Get(search_args) => cmd_get(&ctx, search_args).await,
        _ => unimplemented!("unimplemented subcommand"),
    }
}
