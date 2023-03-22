/*
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
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
    collections::HashMap,
    fs::File,
    io::Read,
};

use serde::Deserialize;
use toml::de::ValueDeserializer;
use yacexits::{
    EX_DATAERR,
    EX_UNAVAILABLE,
};

#[derive(Deserialize)]
pub struct Config {
    hopfiles: Vec<String>,
    sources: HashMap<String, String>,
}

pub fn get_config() -> Result<(), (String, u32)> {
    let xdg_dirs = match xdg::BaseDirectories::with_prefix("hopper") {
        Ok(dirs) => dirs,
        Err(err) => {
            return Err((
                format!("{:?}", err),
                EX_UNAVAILABLE,
            ));
        },
    };
    Ok(())
}

impl Config {
    pub fn read_config(config_path: String) -> Result<Self, (String, u32)> {
        let mut buf: Vec<u8> = Vec::new();

        let mut config_file = match File::open(&config_path) {
            Ok(file) => file,
            Err(_) => {
                return Err((
                    format!("{}: Permission denied.", &config_path),
                    EX_UNAVAILABLE,
                ));
            },
        };

        match config_file.read_to_end(&mut buf) {
            Ok(_) => {},
            Err(err) => {
                return Err((
                    format!("{:?}", err),
                    EX_DATAERR,
                ));
            },
        };

        let toml = match String::from_utf8(buf) {
            Ok(contents) => contents,
            Err(_) => {
                return Err((
                    format!("Invalid configuration file."),
                    EX_DATAERR,
                ));
            },
        };

        match Config::deserialize(ValueDeserializer::new(&toml)) {
            Ok(val) => Ok(val),
            Err(err) => Err((format!("{:?}", err), EX_DATAERR)),
        }
    }
}
