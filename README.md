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

Hopper can automatically search, download, and update Minecraft mods, modpacks,
resource packs, and plugins from [Modrinth](https://modrinth.com/) so that
keeping your mods up-to-date and compatible with each other is easy. With
Hopper, you won't have to manually visit [CurseForge](https://curseforge.com/)
and download each mod one-by-one every time you set up a new instance, or deal
with the hassle of swapping out different mod versions for hours while trying to
get Minecraft to accept them all at once.

Hopper is still very early in development, but important features are coming
along smoothly, and we'll have lots of progress to show off in the coming weeks.
It's written in [Rust](https://www.rust-lang.org/) and released under the
[AGPLv3](LICENSE).

We're looking for people to help contribute code and write documentation. Please
reach out to us in [our Discord "server"](https://discord.gg/jJutHQjsh9) if
you're interested in helping out. If you have a taste in CLI apps like Hopper,
your input is especially appreciated.

Inspired by applications like [paru](https://github.com/morganamilo/paru), a
feature-packed AUR helper and [topgrade](https://github.com/r-darwish/topgrade),
a tool to upgrade everything

[![Donate using
Liberapay](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/tebibytemedia/donate)

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
- Shell autocomplete
- Configurable search result display like [Starship](https://starship.rs)
- Version-control system repository package management & compilation

## External-Dependent:
- Conflict resolution
- Dependency resolution
- Integration into [PolyMC](https://polymc.org/) and/or
[theseus](https://github.com/modrinth/theseus)
- Integration into `topgrade(1)`
- Graphical frontend with notifications

[Modrinth REST API
docs](https://docs.modrinth.com/api-spec/)

# File Structure

```
├── "$XDG_CONFIG_HOME"/hopper.toml
├── "$XDG_CACHE_HOME"/hopper/
│   ├── 1.19.1/
│   │ └── fabric/
│   └── 1.18.2/
│     ├── forge/
│     └── plugin/
└── "XDG_DATA_HOME"/templates/
      └── example-template.hop -> ~/.minecraft/mods/example-template.hop
```

# Hopfile Structure

Hopfiles will contain a Minecraft version number, a list of packages, and any 
references to other hopfiles on which it's based, or "templates". If a hopfile
is based on other template hopfiles, it inherits the packages from them. A
hopfile does not inherit the package or Minecraft version from a template.

```
template = "example-template"
mc-version = "1.19.2"

[packages]
fabric-mod = [ "sodium", "lithium" ]
resource = "alacrity"
```

# Hopper Configuration File Structure

Hopper's configuration will be maintained with a list of all hopfiles known to
hopper. Its config will also contain a list of mod hosting sites like Modrinth
and CurseForge and a list of (remote or local) version-control repositories from
which to compile mods. The latter will use a (potentially custom) build file
format to be defined at a later date.

```
hopfiles = [
  "~/.minecraft/mods/template.hop",
  "~/.minecraft/1.91.1/mods/1.19.1.hop" 
]

[sources]
modrinth = "https://api.modrinth.com/"
curseforge = "https://api.curseforge.com/"
git = [
  "git+https://github.com/IrisShaders/Iris.git"
  "git+https://github.com/CaffeineMC/sodium-fabric.git"
]
```

# Docs

## Usage

`hopper [options...] [subcommand...]`

## OPTIONS

`-v`, `--verbose`

&emsp;Includes debug information in the output of `hopper` subcommands.

## SUBCOMMANDS

`get [options...] [targets...]`

&emsp;Searches for packages, displays the results, and downloads any selected
packages to the local cache. If multiple targets are specified, results are
displayed in order of specification.

OPTIONS

&emsp;`-d`, `--dir [directory...]`

&emsp;&emsp;Specifies the directory to download to (default is "$XDG_CACHE_HOME"/hopper/).

&emsp;`-m`, `--mc-version [version...]`

&emsp;&emsp;Specifies for what version of Minecraft packages are being retrieved.

&emsp;`-n`, `--no-confirm`

&emsp;&emsp;Does not display search results and downloads exact matches to the
cache. Requires `--mc-version` and `--type` be specified.

&emsp;`-t`, `--type [types...]`

&emsp;&emsp;Specifies what types of packages are being queried.

`init [options...]`

&emsp;Creates a hopfile in the current directory and adds it to the global known
hopfiles list.

OPTIONS

&emsp;`-d`, `--dir [directory...]`

&emsp;&emsp;Specifies the directory in which the hopfile is being created.

&emsp;`-f`, `--hopfile [hopfiles...]`

&emsp;&emsp;Specifies templates upon which to base the new hopfile. Hopfile
names should be comma-delineated.

&emsp;`-m`, `--mc-version [version]`

&emsp;&emsp;Specifies for what version of Minecraft packages are being managed.

&emsp;`-t`, `--type [type...]`

&emsp;&emsp;Specifies what type of packages will be listed in this hopfile.

`install [options...] [packages...]`

&emsp;Adds packages to the current hopfile, symlinking them to its directory. If
the package cannot be found in the package cache, `hopper get` is run first.

OPTIONS

&emsp; `-f`, `--hopfile [hopfiles...]`

&emsp;&emsp;Specifies hopfiles to which mods will be added. Hopfile names and
paths should be comma-delineated.

`list [options...]`

&emsp;Lists all installed packages.

OPTIONS

&emsp; `-f` `--hopfile [hopfiles...]`

&emsp;&emsp;Lists packages installed in a specified hopfile.

&emsp;`-m`, `--mc-version [version]`

&emsp;&emsp;Specifies for what version of Minecraft packages are being managed.

&emsp;`-t`, `--type [types...]`

&emsp;&emsp;List all packages of a specified type.

`update [options...]`

&emsp;Updates installed packages and adds mods if they're missing to directories
with known hopfiles.

OPTIONS

&emsp;`-f`, `--hopfile [hopfiles...]`

&emsp;&emsp;Updates only packages in the specified hopfile. Note that this
option creates a new file and symlink as it does not update the packages for
other hopfiles.

&emsp;`-m`, `--mc-version [version]`

&emsp;&emsp;Specifies for what version of Minecraft packages are being updated.

&emsp;`-t`, `--type [types...] [packages...]`

&emsp;&emsp;Updates only packages of a specified type. Optionally takes a list
of packages as an argument.
