/*
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

use yacexits::*;

pub trait CError {
	fn code(&self) -> u32;

	fn message(&self) -> String;

	fn exit(&self) -> ! {
        eprintln!("{}: {}", program_invokation(), self.message());
        exit(self.code());
	}
}

fn program_invokation() -> String {
	// TODO: ideally this would be argv[0] from main.
	// This could be done with a const OnceCell, but I'm not sure I like that solution.
	// Using std, we can do this though:
	std::env::args().next()
		// with a fallback to the program name
		.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_owned())
}

impl<'l> CError for arg::ParseKind<'l> {
	fn message(&self) -> String {
		format!(
			"Usage: {}{}",
			program_invokation(), // argv[0],
			" [-v] add | get | init | list | remove | update\n\n".to_owned() +
			"add [-m version] [-f hopfiles...] packages...\n" +
			"get [-n] [-d directory] [-m versions...] [-t types...] packages\n" +
			"init [-f hopfiles...] version type\n" +
			"list [[-f hopfiles...] | [-m versions...] [-t types...]]\n" +
			"remove [[-f hopfiles...] | type version]] packages...\n" +
			"update [[-f hopfiles... | [-m versions...] [-t types...]]",
		)
	}

	fn code(&self) -> u32 { EX_USAGE }
}

impl CError for xdg::BaseDirectoriesError {
	fn message(&self) -> String {
		format!("Unable to open configuration file: {}", self)
	}

	fn code(&self) -> u32 { EX_UNAVAILABLE }
}