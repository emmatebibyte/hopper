use console::style;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub hits: Vec<ModResult>,
    pub offset: isize,
    pub limit: isize,
    pub total_hits: isize,
}

#[derive(Deserialize, Debug)]
pub struct ModResult {
    pub mod_id: String,               // TODO parse to `local-xxxxx` with regex
    pub project_type: Option<String>, // NOTE this isn't in all search results?
    pub author: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
    pub downloads: isize,
    pub page_url: String,
    pub icon_url: String,
    pub author_url: String,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub client_side: String,
    pub server_side: String,
    pub host: String,
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
    pub id: String, // TODO serialize mod id?
    pub slug: String,
    pub team: String, // TODO serialize team id?
    pub title: String,
    pub description: String,
    pub body: String,
    pub published: String, // TODO serialize datetime
    pub updated: String,   // TODO serialize datetime
    pub status: String,
    pub license: License,
    pub client_side: String, // TODO serialize as enum
    pub server_side: String, // TODO serialize as enum
    pub downloads: isize,
    pub followers: isize,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
    pub icon_url: Option<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Vec<DonationLink>,
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
    pub id: String,        // version id
    pub mod_id: String,    // mod id
    pub author_id: String, // user id
    // NOTE modrinth docs list this as a String, but is actually a bool?
    // featured: String,  // user id
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    pub changelog_url: Option<String>,
    pub date_published: String, // TODO serialize datetime
    pub downloads: isize,
    pub version_type: String, // TODO {alpha | beta | release}
    pub files: Vec<ModVersionFile>,
    // pub dependencies: Vec<String>, // TODO dependency wrangling, thank you modrinth, very cool
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ModVersionFile {
    pub hashes: HashMap<String, String>,
    pub url: String,
    pub filename: String,
}
