/*
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 2022 Spookdot <https://git.tebibyte.media/spookdot/>
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

use crate::{
    api::{
        ModInfo,
        ModResult,
        ModVersion,
        ModVersionFile,
        SearchResponse,
        Error as APIError,
    },
    config::{
        Config,
    },
    args::{
        Arguments,
        Loader,
        PackageType,
        Server,
        SearchArgs,
    },
};

use std::cmp::min;
use std::io::Write;

use curl::easy::{ Easy2, Handler };
use futures_util::StreamExt;

pub struct HopperClient {
    config: Config,
    client: Easy2<Handler>,
}

impl HopperClient {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
            client: Easy2::new()
        }
    }

    pub async fn search_mods(
        &self,
        search_args: &SearchArgs,
    ) -> Result<SearchResponse, (String, u32)> {
        println!("Searching with query “{}”...", search_args.package_name);

        let urls = Vec::new();

        for entry in self.config.sources.drain() {
            let (source, domain) = entry;
            urls.push(format!("{}/v2/search", domain));
        }

        let mut params = vec![("query", search_args.package_name.to_owned())];
        let mut facets: Vec<String> = Vec::new();
        if let versions = &search_args.mc_version {
            let versions_facets = versions
                .iter()
                .map(|e| format!("[\"versions:{}\"]", e))
                .collect::<Vec<String>>()
                .join(",");
            facets.push(format!("{}", versions_facets));
        }
        if let Some(package_type) = &search_args.package_type {
            let project_type = match package_type {
                PackageType::Mod(_) => "[\"project_type:mod\"]",
                PackageType::Pack(_) => "[\"project_type:modpack\"]",
                PackageType::Plugin(_) => "[\"project_type:mod\"]",
                PackageType::ResourcePack => "[\"project_type:resourcepack\"]",
            };

            let project_category = match package_type {
                PackageType::Mod(kind) | PackageType::Pack(kind) => {
                    match kind {
                        Loader::Fabric => "[\"categories:fabric\"]",
                        Loader::Forge => "[\"categories:forge\"]",
                        Loader::Quilt => "[\"categories:quilt\"]",
                    }
                },
                PackageType::Plugin(kind) => {
                    match kind {
                        Server::Bukkit => "[\"categories:bukkit\"]",
                        Server::Paper => "[\"categories:paper\"]",
                        Server::Purpur => "[\"categories:purpur\"]",
                        Server::Spigot => "[\"categories:spigot\"]",
                        Server::Sponge => "[\"categories:sponge\"]",
                    }
                },
            };

            let package_type_facet = format!(
                "{},{}",
                project_type,
                project_category,
            );

            facets.push(package_type_facet);
        }

        if !facets.is_empty() {
            params.push(("facets", format!("[{}]", facets.join(","))));
        }

        // TODO: Rewrite using curl
        let url = reqwest::Url::parse_with_params(url.as_str(), &params)?;
        info!("GET {}", url);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.json::<SearchResponse>().await?)
        } else {
            Err(response.json::<APIError>().await?.into())
        }

    }
        pub async fn fetch_mod_info(&self, mod_result: &ModResult) -> anyhow::Result<ModInfo> {
        let mod_id = &mod_result.project_id;
        println!(
            "Fetching mod info for {} (ID: {})...",
            mod_result.title, mod_id
        );

        let url = format!(
            "https://{}/v2/project/{}",
            self.config.upstream.server_address, mod_id
        );
        info!("GET {}", url);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.json::<ModInfo>().await?)
        } else {
            Err(response.json::<APIError>().await?.into())
        }
    }

    pub async fn fetch_mod_version(&self, version_id: &String) -> anyhow::Result<ModVersion> {
        println!("Fetching mod version {}...", version_id);

        let url = format!(
            "https://{}/v2/version/{}",
            self.config.upstream.server_address, version_id
        );
        info!("GET {}", url);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            Ok(response.json::<ModVersion>().await?)
        } else {
            Err(response.json::<APIError>().await?.into())
        }
    }

    pub async fn download_version_file(
        &self,
        args: &Args,
        file: &ModVersionFile,
    ) -> anyhow::Result<()> {
        // TODO replace all uses of .unwrap() with proper error codes
        let filename = &file.filename;

        // TODO make confirmation skippable with flag argument
        if !args.auto_accept {
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
        let url = &file.url;
        info!("GET {}", url);
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(response.json::<APIError>().await?.into())
        }

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
}
