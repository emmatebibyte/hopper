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
    EX_UNAVAILABLE,
};

struct AppContext {
    args: Arguments,
    config: Config,
}

#[tokio::main]
#[no_mangle]
async fn rust_main(args: c_main::Args) {
    let arguments = Arguments::from_args(args.into_iter());

    let xdg_dirs = match xdg::BaseDirectories::with_prefix("hopper") {
        Ok(dirs) => dirs,
        Err(err) => {
            eprintln!("{:?}", err);
            exit(EX_UNAVAILABLE);
        },
    };

    let config_path = match get_config(xdg_dirs) {
        Ok(path) => path,
        Err((err, code)) => {
            eprintln!("{:?}", err);
            exit(code);
        },
    };

    let config = match Config::read_config(config_path) {
        Ok(file) => file,
        Err((err, code)) => {
            eprintln!("{:?}", err);
            exit(code);
        },
    };

    let ctx = AppContext { args, config };

    match ctx.arguments.command {
        // Command::Get(search_args) => cmd_get(&ctx, search_args).await,
        // Command::Init(hopfile_args) => cmd_init(hopfile_args).await,
        _ => unimplemented!("unimplemented subcommand"),
    };
}
