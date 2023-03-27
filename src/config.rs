/*
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 2022 Spookdot <https://git.tebibyte.media/spookdot/>
 * Copyright (c) 2023 [ ] <https://git.tebibyte.media/BlankParenthesis/>
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

use crate::HopError;

use std::{
    fs::File,
    io::{ Read, self },
    path::PathBuf,
};

use serde::Deserialize;
use xdg::BaseDirectories;
use yacexits::{
    EX_DATAERR,
    EX_UNAVAILABLE,
};

#[derive(Deserialize)]
pub struct Config {
    pub hopfiles: Vec<String>,
    pub sources: Sources,
}

#[derive(Deserialize)]
pub struct Sources {
    pub modrinth: Vec<String>,
}

pub enum ConfigError {
    CreateError(io::Error),
    OpenError(io::Error),
    ReadError(io::Error),
    FormatError(std::string::FromUtf8Error),
    ParseError(toml::de::Error),
}

impl From<ConfigError> for (String, u32) {
	fn from(error: ConfigError) -> Self {
        let (message, code) = match error {
            ConfigError::CreateError(_) => {
				("Unable to create configuration file.", EX_UNAVAILABLE)
            },
            ConfigError::OpenError(_) => {
				("Unable to open configuration file.", EX_UNAVAILABLE)
            },
            ConfigError::ReadError(_) => {
				("Error while reading configuration file.", EX_DATAERR)
            },
            ConfigError::FormatError(_) => {
				("Configuration file is not valid UTF-8.", EX_DATAERR)
            },
            ConfigError::ParseError(_) => {
				("Unable to parse configuration file.", EX_DATAERR)
            },
        };

		(message.to_string(), code)
	}

}

impl From<xdg::BaseDirectoriesError> for ConfigError {
	fn from(err: xdg::BaseDirectoriesError) -> Self {
		ConfigError::CreateError(io::Error::from(err))
	}
}

impl Config {
    pub fn read_config() -> Result<Self, ConfigError> {
		let config_path = BaseDirectories::with_prefix("hopper")?
			.place_config_file("config.toml")
			.map_err(ConfigError::CreateError)?;
        let mut buf: Vec<u8> = Vec::new();

        let mut config_file = File::open(&config_path)
            .map_err(ConfigError::OpenError)?;

        config_file.read_to_end(&mut buf)
            .map_err(ConfigError::ReadError)?;

        let toml = String::from_utf8(buf)
            .map_err(ConfigError::FormatError)?;

        toml::from_str(&toml).map_err(ConfigError::ParseError)
    }
}
