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

use curl::easy::{ Easy };
use futures_util::StreamExt;

pub struct HopperClient {
    config: Config,
    client: Easy,
}

impl HopperClient {
    pub fn new(config: Config) -> Self {
        curl::init();
        Self {
            config: config,
            client: Easy::new()
        }
    }

    pub async fn search_mods(
        &mut self,
        search_args: &SearchArgs,
    ) /*-> Result<SearchResponse, (String, u32)>*/ {
        println!("Searching with query “{}”...", search_args.package_name);

        let mut urls = Vec::new();

        for entry in self.config.sources.modrinth.iter() {
            urls.push(format!("{}/v2/search", entry));
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
        if let Some(package_type) = search_args.package_type {
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
                PackageType::ResourcePack => "",
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
    }
}
