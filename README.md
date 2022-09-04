```
              ___---___
        ___---    |    ---___
     ---      ___---___      ---
    |---___---         ---___---|
    |      ---___   ___---      |
     ---___       |       ___---
           |---___|___---|
            --__  -  __--
                |-_-|
                 -_-
    __  __
   / / / /___  ____  ____  ___  _____
  / /_/ / __ \/ __ \/ __ \/ _ \/ ___/
 / __  / /_/ / /_/ / /_/ /  __/ /
/_/ /_/\____/ .___/ .___/\___/_/
           /_/   /_/
```

# Hopper

A Minecraft package manager for the terminal.

Hopper can automatically search, download, and update Minecraft mods from
https://modrinth.com/ so that keeping your mods up-to-date and compatible with
each other is easy. With Hopper, you won't have to manually visit CurseForge and
download each mod one-by-one every time you set up a new instance, or deal with
the hassle of swapping out different mod versions for hours while trying to get
Minecraft to accept them all at once.

Hopper is still very early in development, but important features are coming
along smoothly, and we'll have lots of progress to show off in the coming weeks.
It's written in Rust and released under the AGPLv3.

We're looking for people to help contribute code and write documentation. Please
reach out to us in [our Discord "server"](https://discord.gg/jJutHQjsh9) if
you're interested in helping out. If you have a taste in CLI apps like Hopper,
your input is especially appreciated.

Inspired by applications like [paru](https://github.com/morganamilo/paru), a
feature-packed AUR helper and [topgrade](https://github.com/r-darwish/topgrade).
a tool to upgrade everything

# Donate

<noscript><a
href="https://liberapay.com/tebibytemedia/donate"><img alt="Donate using
Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a></noscript>

# High-level Goals

## Continuous
- Small binary size
- Minimal compile times

## Features

### High Priority:
- Modrinth package searching
- Modrinth package installation
- Parallel package downloading
- Per-instance package management
- Package updating
- Listing installed packages

### Medium Priority
- CurseForge package searching
- CurseForge package installation
- A `man(1)` entry

### Low Priority:
- `fish(1)` autocomplete
- `bash(1)` autocomplete
- `zsh(1)` autocomplete
- [Nushell](https://www.nushell.sh/) autocomplete
- Configurable search result display like [Starship](https://starship.rs)

## External-Dependent:
- Conflict resolution
- Dependency resolution
- Integration into [PolyMC](https://polymc.org/) and/or
[theseus](https://github.com/modrinth/theseus)
- Integration into `togprade(1)`
- Graphical frontend with notifications

[Modrinth REST API
docs](https://docs.modrinth.com/api-spec/)

# File Architecture

```
~/.hopper/
├── hopper.conf
├── cache/
│   ├── 1.19.1/
│   │ └── fabric
│   └── 1.18.2/
│     ├── forge
│     └── plugin
└── templates/
      └── arbitrary.hop -> ~/.minecraft/mods/arbitrary.hop
```

# Hopfile Structure

Hopfiles will contain a Minecraft version number, a list of packages, the name
of the type of package it uses, and any references to other templates it's based
on. If a template is based on other templates, it inherits the packages from
those templates. A template does not inherit the package or Minecraft version
from another template.

```
template = abitrary

mc-version = 1.19.2

type = fabric

[packages]
sodium
```

# Hopper Configuration File Structure

Hopper will maintain a list of all hopfiles known to hopper. Its config will
also contain a list of mod hosting sites like Modrinth and CurseForge and a list
of (remote or local) version-control repositories from which to compile mods.
The latter will use a (potentially custom) build file format to be defined at a
later date.

```
[hopfiles]
file = ~/.minecraft/mods/1.19.1.hop

# Mod Hosts

[Modrinth]
api = https://api.modrinth.com/

[CurseForge]
api = https://api.curseforge.com/

# Git Repositories

[Iris Shaders]
source = git+https://github.com/IrisShaders/Iris.git
```

# Docs

## Usage

`hopper [SUBCOMMAND] [OPTIONS]`

## Common OPTIONS:

`-d`, `--dir[=FILE]`
    Specifies the path for the hopfile being utilized

`-f`, `--filename=[FILE]`
    Saves to a specific file name.

`-m`, `--mc-version[=VERSION]`
    Specifies for what VERSION of Minecraft PACKAGES are being managed

`-t`, `--type[=TYPE]`
    Specifies what TYPE of PACKAGEs is being referenced

`-v`, `--verbose`
    Includes debug information in the output of `hopper` commands.

## SUBCOMMANDs

`get [OPTIONS] PACKAGE`
    Searches for a PACKAGE, displays the results, and downloads any selected
    PACKAGES to the local cache.

OPTIONS
    `-n`, `--no-confirm`
        Does not display search results and downloads exact matches to the
        cache. Requires `--mc-version` and `--type` be specified.

`init [OPTIONS] [--mc-version=VERSION] [--type=TYPE] TEMPLATE`
    Creates a hopfile in the current directory and adds it to the global known
    hopfiles list in the configuration file. If a TEMPLATE is passed as an
    argument, the hopfile is added as a new template. A name is generated using
    the VERSION and TYPE specified unless `--filename` is used.

OPTIONS
    `--template[=TEMPLATE1,TEMPLATE2...]`
        Specifies TEMPLATE hopfiles' names upon which to base the new hopfile.

`install [OPTIONS] PACKAGE`
    Adds a PACKAGE to the current hopfile and runs `hopper update`. If the
    PACKAGE cannot be found in the package cache, it runs `hopper get` first.

OPTIONS
    `--template[=TEMPLATE1,TEMPLATE2...]`
        Specifies a template hopfile to which to install mods

`list [OPTIONS]`
    Lists all installed packages.

`update [OPTIONS] PACKAGE`
    Updates installed PACKAGEs and adds mods if they're missing to directories
    with known hopfiles. If a PACKAGE is passed, `--type` must be specified so
    that hopper `update`s the correct package.