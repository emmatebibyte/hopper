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

use console::style;
use serde::Deserialize;
use std::{ collections::HashMap, fmt };

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub hits: Vec<ModResult>,
    pub offset: isize,
    pub limit: isize,
    pub total_hits: isize,
}

#[derive(Deserialize, Debug)]
pub struct ModResult {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub display_categories: Vec<String>, // NOTE this is not in the OpenAPI docs
    pub client_side: String,
    pub server_side: String,
    pub project_type: String, // NOTE this isn't in all search results?
    pub downloads: isize,
    pub icon_url: String,
    pub project_id: String, // TODO parse to 'local-xxxx' with regex
    pub author: String,
    pub versions: Vec<String>,
    pub follows: isize,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub gallery: Vec<String>,
}

impl ModResult {
    pub fn format_info(&self) -> String {
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

    pub fn format_description(&self) -> String {
        self.description.to_owned()
    }

    pub fn display(&self, index: usize) {
        let index = style(index).magenta();
        let info = self.format_info();
        let description = self.format_description();
        println!("{:>2} {}\n    {}", index, info, description);
    }
}

#[derive(Deserialize, Debug)]
pub struct ModInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub additional_categories: Vec<String>, // NOTE not listed in OpenAPI docs
    pub client_side: String,                // TODO serialize as enum
    pub server_side: String,                // TODO serialize as enum
    pub body: String,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Option<Vec<DonationLink>>,
    pub project_type: String,
    pub downloads: isize,
    pub icon_url: Option<String>,
    pub id: String,               // TODO serialize mod id?
    pub team: String,             // TODO serialize team id?
    pub body_url: Option<String>, // NOTE deprecated
    pub moderator_message: Option<String>,
    pub published: String,        // TODO serialize as datetime
    pub updated: String,          // TODO serialize as datetime
    pub approved: Option<String>, // NOTE not listed in OpenAPI docs, TODO serialize as datetime
    pub followers: isize,
    pub status: String,
    pub license: License,
    pub versions: Vec<String>,
    pub gallery: Option<Vec<GalleryEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct GalleryEntry {
    pub url: String,
    pub featured: bool,
    pub title: String,
    pub description: String,
    pub created: String,
}

#[derive(Deserialize, Debug)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct DonationLink {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ModVersion {
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    // pub dependencies: Option<Vec<String>>, // TODO dependency wrangling, thank you modrinth, very cool
    pub game_versions: Vec<String>,
    pub version_type: String, // TODO {alpha | beta | release}
    pub loaders: Vec<String>,
    pub featured: bool,
    pub id: String,             // version id
    pub project_id: String,     // mod id
    pub author_id: String,      // user id
    pub date_published: String, // TODO serialize datetime
    pub downloads: isize,
    pub changelog_url: Option<String>, // NOTE deprecated
    pub files: Vec<ModVersionFile>,
}

#[derive(Deserialize, Debug)]
pub struct ModVersionFile {
    pub hashes: HashMap<String, String>,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: isize,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub error: String,
    pub description: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.description)
    }
}

impl std::error::Error for Error {}
