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
    io::Read,
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

pub fn get_config(dirs: BaseDirectories) -> Result<PathBuf, (String, u32)> {
    match dirs.place_config_file("config.toml") {
        Ok(file) => Ok(file),
        Err(_) => {
            Err((
                format!("Unable to create configuration file."),
                EX_UNAVAILABLE,
            ))
        },
    }
}

impl Config {
    pub fn read_config(config_path: PathBuf) -> Result<Self, (String, u32)> {
        let mut buf: Vec<u8> = Vec::new();

        let mut config_file = match File::open(&config_path) {
            Ok(file) => file,
            Err(_) => {
                return Err((
                    format!("{}: Permission denied.", config_path.display()),
                    EX_UNAVAILABLE,
                ));
            },
        };

        if let Some(err) = config_file.read_to_end(&mut buf).err() {
            return Err((format!("{:?}", err), EX_DATAERR));
        };

        let toml = match String::from_utf8(buf) {
            Ok(contents) => contents,
            Err(err) => {
                return Err((
                    format!("{:?}", err),
                    EX_DATAERR,
                ));
            },
        };

        match toml::from_str(&toml) {
            Ok(val) => Ok(val),
            Err(_) => {
                Err((
                    format!(
                        "{}: Invalid configuration file.", config_path.display()
                    ),
                    EX_DATAERR,
                ))
            },
        }
    }
}
