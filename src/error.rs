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

pub struct HopError {
	pub code: u32,

	pub message: String,
}

impl From<arg::ParseKind<'_>> for HopError {
	fn from(_: arg::ParseKind) -> Self {
		let message = format!(
			"Usage: {}",
			"[-v] add | get | init | list | remove | update\n\n".to_owned() +
			"add [-m version] [-f hopfiles...] packages...\n" +
			"get [-n] [-d directory] [-m versions...] [-t types...] packages\n" +
			"init [-f hopfiles...] version type\n" +
			"list [[-f hopfiles...] | [-m versions...] [-t types...]]\n" +
			"remove [[-f hopfiles...] | type version]] packages...\n" +
			"update [[-f hopfiles... | [-m versions...] [-t types...]]",
		);
		Self { message, code: EX_USAGE }
	}
}

impl From<xdg::BaseDirectoriesError> for HopError  {
	fn from(err: xdg::BaseDirectoriesError) -> Self {
		let message = format!("{}: Unable to open configuration file", err);

		Self { message, code: EX_UNAVAILABLE }
	}
}

impl From<HopError> for (String, u32) {
	fn from(err: HopError) -> Self {
		(err.message, err.code)
	}
}
