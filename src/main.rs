/*
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 2022 Spookdot <https://git.tebibyte.media/spookdot/>
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

#![no_main]

mod api;
mod args;
mod client;
mod config;
mod hopfile;

use api::*;
use args::*;
use client::*;
use config::*;
use hopfile::*;

use yacexits::{
    exit,
    EX_SOFTWARE,
    EX_UNAVAILABLE,
    EX_USAGE,
};

struct AppContext {
    args: Arguments,
    config: Config,
}

#[tokio::main]
#[no_mangle]
async fn rust_main(arguments: c_main::Args) {
    let argv: Vec<&str> = arguments.into_iter().collect();

    let usage_info = format!(
        "Usage: {}{}",
        argv[0],
        " [-v] add | get | init | list | remove | update\n\n".to_owned() +
        "add [-m version] [-f hopfiles...] packages...\n" +
        "get [-n] [-d directory] [-m versions...] [-t types...] packages\n" +
        "init [-f hopfiles...] version type\n" +
        "list [[-f hopfiles...] | [-m versions...] [-t types...]]\n" +
        "remove [[-f hopfiles...] | type version]] packages...\n" +
        "update [[-f hopfiles... | [-m versions...] [-t types...]]",
    );


    let args = Arguments::from_args(
        argv
        .clone()
        .into_iter()
    ).unwrap_or_else(|_| {
        eprintln!("{}", usage_info);
        exit(EX_USAGE);
    });

    let xdg_basedirs = xdg::BaseDirectories::with_prefix("hopper");

    // this might be cursed; I haven’t decided
    let config = match get_config(
        xdg_basedirs.unwrap_or_else(|_| {
            eprintln!(
                "{}: Unable to open configuration file: Permission denied.",
                argv[0],
            );
            exit(EX_UNAVAILABLE);
        })
    ) {
        Ok(path) => {
            match Config::read_config(path) {
                Ok(file) => file,
                Err((err, code)) => {
                    eprintln!("{}: {}", argv[0], err);
                    exit(code);
                },
            }
        },
        Err((err, code)) => {
            eprintln!("{}: {}", argv[0], err);
            exit(code);
        },
    };

    let ctx = AppContext { args, config };

    match ctx.args.sub {
        // Command::Get(search_args) => cmd_get(&ctx, search_args).await,
        // Command::Init(hopfile_args) => cmd_init(hopfile_args).await,
        _ => {
            eprintln!(
                "{}: {}: Unimplemented subcommand.",
                argv[0],
                ctx.args.sub
            );
            exit(EX_SOFTWARE);
        },
    };
}
