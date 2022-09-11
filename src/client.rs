use crate::api::{ModInfo, ModResult, ModVersion, ModVersionFile, SearchResponse};
use crate::config::{Args, Config, SearchArgs};
use futures_util::StreamExt;
use log::*;
use std::cmp::min;
use std::io::Write;

pub struct HopperClient {
    config: Config,
    client: reqwest::Client,
}

impl HopperClient {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search_mods(&self, search_args: &SearchArgs) -> anyhow::Result<SearchResponse> {
        println!("Searching with query \"{}\"...", search_args.package_name);

        let url = format!("https://{}/v2/search", self.config.upstream.server_address);

        let mut params = vec![("query", search_args.package_name.to_owned())];
        let mut facets: Vec<String> = Vec::new();
        if let Some(versions) = &search_args.version {
            let versions_facets = versions
                .iter()
                .map(|e| format!("[\"versions:{}\"]", e))
                .collect::<Vec<String>>()
                .join(",");
            facets.push(format!("{}", versions_facets));
        }
        if let Some(package_type) = &search_args.package_type {
            facets.push(format!("[\"project_type:{}\"]", package_type.to_string()));
        }
        params.push(("facets", format!("[{}]", facets.join(","))));

        let url = reqwest::Url::parse_with_params(url.as_str(), &params)?;
        info!("GET {}", url);
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;
        Ok(response)
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
        let response = response.json::<ModInfo>().await?;
        Ok(response)
    }

    pub async fn fetch_mod_version(&self, version_id: &String) -> anyhow::Result<ModVersion> {
        println!("Fetching mod version {}...", version_id);

        let url = format!(
            "https://{}/v2/version/{}",
            self.config.upstream.server_address, version_id
        );
        info!("GET {}", url);
        let response = self.client.get(url).send().await?;
        let response = response.json::<ModVersion>().await?;
        Ok(response)
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
