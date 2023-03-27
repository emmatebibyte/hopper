/*
 * Copyright (c) 2021–2022 Marceline Cramer <mars@tebibyte.media>
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 2022 Spookdot <https://git.tebibyte.media/spookdot/>
 * Copyright (c) 2022–2023 [ ] <https://git.tebibyte.media/BlankParenthesis/>
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
mod error;

use api::*;
use args::*;
use client::*;
use config::*;
use hopfile::*;
use error::*;

use yacexits::{
	exit,
	EX_OSERR,
	EX_SOFTWARE,
};

struct AppContext {
	args: Arguments,
	config: Config,
}

#[tokio::main]
#[no_mangle]
async fn rust_main(arguments: yacexits::Args) -> Result<u32, (String, u32)> {
	let argv: Vec<&str> = arguments.into_iter().collect();

	let args = match Arguments::from_args(argv.clone().into_iter()) {
		Ok(args) => args,
		Err(_) => {
			return Err((format!("Unable to ascertain arguments."), EX_OSERR));
		}
	};

	let config = Config::read_config()?;
	let ctx = AppContext { args, config };

	match ctx.args.sub {
		// Command::Get(search_args) => cmd_get(&ctx, search_args).await,
		// Command::Init(hopfile_args) => cmd_init(hopfile_args).await,
		_ => {
			let message = format!(
				"{}: Unimplemented subcommand.", ctx.args.sub
			);
			let code = EX_SOFTWARE;
			Err(HopError { message, code })
		},
	}?
}
