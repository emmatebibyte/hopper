/*
 * Copyright (c) 2022 [ ] <https://git.tebibyte.media/BlankParenthesis/>
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

use std::str::FromStr;

use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hopfile {
    pub template: Option<String>,
    // TODO: possibly parse this into a more specific format (enum maybe?)
    pub mc_version: String,
    pub packages: Packages,
}

impl Hopfile {
    pub fn new(template: Option<String>, version: Option<String>) -> Self {
        Self {
            template,
            mc_version: version.unwrap_or_else(|| String::from("1.19.1")),
            packages: Packages::default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Packages {
    pub mods: Vec<Resource>,
    pub resources: Vec<Resource>,
}


#[derive(Debug, Clone, Copy, Default)]
enum Provider {
    #[default]
    Modrinth,
}

impl FromStr for Provider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "modrinth" => Ok(Self::Modrinth),
            _ => Err(()),
        }
    }
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Modrinth => "modrinth",
        })
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    provider: Provider,
    name: String,
}

impl FromStr for Resource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((provider, name)) = s.split_once(':') {
            Ok(Resource {
                provider: Provider::from_str(provider)?,
                name: name.to_string(),
            })
        } else if !s.is_empty() {
            Ok(Resource {
                provider: Provider::default(),
                name: s.to_string(),
            })
        } else {
            Err(())
        }
    }
}

impl ToString for Resource {
    fn to_string(&self) -> String {
        [self.provider.to_string().as_str(), self.name.as_str()].join(":")
    }
}

impl Serialize for Resource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct V;

impl<'de> Visitor<'de> for V {
    type Value = Resource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error, {
        Resource::from_str(v).map_err(|_| serde::de::Error::custom(
            format!("Failed to parse mod/resource: '{}'", v)
        ))
    }
}

impl<'de> Deserialize<'de> for Resource {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_str(V)
    }
}
