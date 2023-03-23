/*
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
 * Copyright (c) 2022 [ ] <https://git.tebibyte.media/BlankParenthesis/>
 * Copyright (c) 2023 Emma Tebibyte <emma@tebibyte.media>
 * SPDX-License-Identifier: AGPL-3.0-or-later
 *
 * This file is part of Hopper.
 *
 * Hopper is free software: you can redistribute it and/or modify it under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * Hopper is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with
 * Hopper. If not, see <https://www.gnu.org/licenses/>.
 */

pub async fn cmd_get(
    ctx: &AppContext,
    search_args: SearchArgs
) -> anyhow::Result<()> {
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

pub async fn cmd_init(args: HopfileArgs) -> anyhow::Result<()> {
    let mut path = args.dir.unwrap_or_default();
    path.push("info.hop");

    if path.try_exists().expect("Invalid dir") {
        let message = format!(
            "hopfile already exists: {}",
            path.to_str().unwrap()
        );
        Err(anyhow::Error::msg(message))
    } else {
        let mut file = File::create(&path).await?;
        let doc = Hopfile::new(args.template, args.version);
        let output = toml_edit::easy::to_string_pretty(&doc).unwrap();

        file.write_all(output.as_bytes()).await?;

        println!("Saved {}", path.to_str().unwrap());
        Ok(())
    }
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

