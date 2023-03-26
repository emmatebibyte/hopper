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

use std::{
    fs::File,
    io::{Read, self},
    path::PathBuf,
};

use serde::Deserialize;
use xdg::BaseDirectories;
use yacexits::{
    EX_DATAERR,
    EX_UNAVAILABLE,
};

use crate::error::CError;

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
    CreateFailed(io::Error),
    OpenError(io::Error),
    ReadError(io::Error),
    FormatError(std::string::FromUtf8Error),
    ParseError(toml::de::Error),
}

impl CError for ConfigError {
    fn message(&self) -> String {
        match self {
            Self::CreateFailed(err) => {
                format!("Unable to create configuration file: {}", err)
            },
            Self::OpenError(err) => {
                format!("Unable to open configuration file: {}", err)
            },
            Self::ReadError(err) => {
                format!("Error while reading configuration file: {}", err) 
            },
            Self::FormatError(err) => {
                format!("Configuration file is not valid utf-8: {}", err) 
            },
            Self::ParseError(err) => {
                format!("Unable to parse configuration file: {}", err) 
            },
        }
    }

    fn code(&self) -> u32 { 
        match self {
            Self::CreateFailed(_) => EX_UNAVAILABLE,
            Self::OpenError(_) => EX_UNAVAILABLE,
            Self::ReadError(_) => EX_DATAERR,
            Self::FormatError(_) => EX_DATAERR,
            Self::ParseError(_) => EX_DATAERR,
        }
        
    }
}

pub fn get_config(dirs: BaseDirectories) -> Result<PathBuf, ConfigError> {
    dirs.place_config_file("config.toml").map_err(ConfigError::CreateFailed)
}

impl Config {
    pub fn read_config(config_path: PathBuf) -> Result<Self, ConfigError> {
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
