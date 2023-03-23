# Contributing

When contributing a pull request to the main branch, please sign your commits
with a PGP key and add your name and the year to the bottom of the list of
copyright holders for the file. For example, an existing copyright header might
read:

```
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
```

You would add your name below it like this:

```
 * Copyright (c) 2022–2023 Emma Tebibyte <emma@tebibyte.media>
 * Copyright (c) 20XX Your Name <your e-mail address or website>
```

Only list years in which you worked on the source file. For example:

```
 * Copyright (c) 2020–2021, 2023 Your Name <yourname@example.com>
```

This header shows that `Your Name` worked on this source file in 2020, 2021, and
2023. Please use the en dash (“–”) to separate the years in the copyright
notice.

If you are contributing a new file, please add the following license header text
to it, replacing the proper text on the copyright line:

```
/*
 * Copyright (c) 20XX [Your Name] <yourname@example.com>
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
```

When including code provided under an AGPLv3-compatible license, please modify
the license notice. The following example contains an Expat (MIT) license
notice:

```
/*
 * Copyright (c) 20XX [Your Name] <yourname@example.com>
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
 *
 * This file incorporates work covered by the following copyright and permission
 * notice:
 *
 *     MIT License
 *
 *     Copyright (c) <year> <copyright holders>
 *
 *     Permission is hereby granted, free of charge, to any person obtaining a
 *     copy of this software and associated documentation files (the
 *     "Software"), to deal in the Software without restriction, including
 *     without limitation the rights to use, copy, modify, merge, publish,
 *     distribute, sublicense, and/or sell copies of the Software, and to permit
 *     persons to whom the Software is furnished to do so, subject to the
 *     following conditions:
 *
 *     The above copyright notice and this permission notice (including the next
 *     paragraph) shall be included in all copies or substantial portions of the
 *     Software.
 *
 *     THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 *     OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 *     MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
 *     NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 *     DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 *     OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 *     USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
```

When writing code, make sure lines never exceed 80 characters in width when
using four-character-wide tabs.
