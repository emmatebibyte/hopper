/*
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
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

use core::str::FromStr;

use arg::Args;

#[derive(Args, Debug)]
struct Arguments {
    #[arg(short = "v")]
    v: bool,
    
    #[arg(sub)]
    sub: Command,
}

#[derive(Args, Debug)]
struct InitArgs {
    #[arg(short = "d")]
    dir: Option<String>,

    #[arg(short = "f")]
    template: Option<String>,

    #[arg(short = "m")]
    mc_version: Vec<String>,
    
    #[arg(short = "t", required)]
    package_type: PackageType,
}

#[derive(Args, Debug)]
struct HopArgs {
    #[arg(short = "f")]
    hopfile: Option<String>,

    #[arg(short = "m")]
    mc_version: Vec<String>,

    #[arg(short = "t")]
    package_type: Option<PackageType>,
}

#[derive(Args, Debug)]
struct SearchArgs {
    package_name: String,

    /// Overrides the download directory
    #[arg(short = "d")]
    dir: Option<String>,

    /// Restricts the target Minecraft version
    #[arg(short = "m")]
    mc_version: Vec<String>,

    /// Type of package to use
    #[arg(short = "t")]
    package_type: Option<PackageType>,
}

#[derive(Args, Debug)]
enum Command {
    Add(SearchArgs),
    Get(SearchArgs),
    Init(InitArgs),
    List(HopArgs),
    Remove(HopArgs),
    Update(HopArgs),
}

#[derive(Debug)]
enum PackageType {
    Mod(Loader),
    Pack(Loader),
    Plugin(Server),
    ResourcePack,
}

#[derive(Debug)]
enum Loader { 
    Fabric,
    Forge,
    Quilt,
}

#[derive(Debug)]
enum Server {
    Bukkit,
    Paper,
    Purpur,
    Spigot,
    Sponge,
}

#[derive(Debug)]
enum PackageParseError {
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
