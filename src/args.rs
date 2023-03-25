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

use core::{
    fmt,
    str::FromStr,
};

pub use arg::Args;

#[derive(Args, Debug)]
pub struct Arguments {
    pub argv0: String,

    #[arg(short = "v")]
    pub v: Option<bool>,
    
    #[arg(sub)]
    pub sub: Command,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(short = "d")]
    pub dir: Option<String>,

    #[arg(short = "f")]
    pub template: Option<String>,

    #[arg(short = "m")]
    pub mc_version: Vec<String>,
    
    #[arg(short = "t", required)]
    pub package_type: PackageType,
}

#[derive(Args, Debug)]
pub struct HopArgs {
    #[arg(short = "f")]
    pub hopfile: Option<String>,

    #[arg(short = "m")]
    pub mc_version: Vec<String>,

    #[arg(short = "t")]
    pub package_type: Option<PackageType>,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    pub package_name: String,

    /// Overrides the download directory
    #[arg(short = "d")]
    pub dir: Option<String>,

    /// Restricts the target Minecraft version
    #[arg(short = "m")]
    pub mc_version: Vec<String>,

    /// Type of package to use
    #[arg(short = "t")]
    pub package_type: Option<PackageType>,
}

#[derive(Args, Debug)]
pub enum Command {
    Add(SearchArgs),
    Get(SearchArgs),
    Init(InitArgs),
    List(HopArgs),
    Remove(HopArgs),
    Update(HopArgs),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Command::Add(_) => write!(f, "add"),
            Command::Get(_) => write!(f, "get"),
            Command::Init(_) => write!(f, "init"),
            Command::List(_) => write!(f, "list"),
            Command::Remove(_) => write!(f, "remove"),
            Command::Update(_) => write!(f, "update"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PackageType {
    Mod(Loader),
    Pack(Loader),
    Plugin(Server),
    ResourcePack,
}

#[derive(Clone, Copy, Debug)]
pub enum Loader { 
    Fabric,
    Forge,
    Quilt,
}

#[derive(Clone, Copy, Debug)]
pub enum Server {
    Bukkit,
    Paper,
    Purpur,
    Spigot,
    Sponge,
}

#[derive(Clone, Debug)]
pub enum PackageParseError {
    Invalid(String),
}

impl FromStr for PackageType {
    type Err = PackageParseError;
    fn from_str(s: &str) -> Result<PackageType, PackageParseError> {
        let pieces: Vec<&str> = s.split("-").collect();

        if pieces.len() > 2 || pieces.len() == 1 {
            return Err(PackageParseError::Invalid(
                format!("{}: Invalid package name.", s)
            ));
        }

        let (prefix, postfix) = (pieces[0], pieces[1]);

        let loader = match prefix {
            "bukkit" => return Ok(PackageType::Plugin(Server::Bukkit)),
            "fabric" => Loader::Fabric,
            "forge" => Loader::Forge,
            "paper" => return Ok(PackageType::Plugin(Server::Paper)),
            "purpur" => return Ok(PackageType::Plugin(Server::Purpur)),
            "quilt" => Loader::Quilt,
            "resource" => return Ok(PackageType::ResourcePack),
            "spigot" => return Ok(PackageType::Plugin(Server::Spigot)),
            "sponge" => return Ok(PackageType::Plugin(Server::Sponge)),
            _ => {
                return Err(PackageParseError::Invalid(
                    format!("{}: Invalid package type.", prefix)
                ))
            },
        };

        match postfix {
            "mod" => Ok(PackageType::Mod(loader)),
            "pack" => Ok(PackageType::Pack(loader)),
            _ => {
                Err(PackageParseError::Invalid(
                    format!("{}: Invalid package type.", postfix)
                ))
            },
        }
    }
}
