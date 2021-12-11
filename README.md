# Hopper

A Minecraft mod manager for the terminal.

Hopper can automatically search, download, and update Minecraft mods from https://modrinth.com/ so that keeping your mods up-to-date and compatible with each other is easy. With Hopper, you won't have to manually visit CurseForge and download each mod one-by-one every time you set up a new instance, or deal with the hassle of swapping out different mod versions for hours while trying to get Minecraft to accept them all at once.

Hopper is still very early in development, but important features are coming along smoothly, and we'll have lots of progress to show off in the coming weeks. It's written in Rust and released under the AGPLv3.

We're looking for people to help contribute code, design the terminal interface, write documentation, and design a logo. Please reach out to us in [our Discord server](https://discord.gg/JWRFAbve9M) if you're interested in helping out. If you have a taste in CLI-based apps like Hopper, your input is especially appreciated.

Inspired by CLI apps like:
- [paru](https://github.com/morganamilo/paru): Feature packed AUR helper
- [topgrade](https://github.com/r-darwish/topgrade): Upgrade everything

### Donate

<noscript><a href="https://liberapay.com/tebibytemedia/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a></noscript>

# High-level Goals

Continuous:
- small binary size
- minimal compile times

Features:
- modrinth mod searching
- modrinth mod installation
- curseforge api too?
- per-instance mod management
- mod updating
- fish autocomplete
- bash autocomplete
- zsh autocomplete
- nushell autocomplete
- manpage
- configurable mod search result display like [Starship](https://starship.rs)
- `display` command or something that displays (cached?) mod info
- parallel mod downloading

Long-term/host-dependent:
- conflict resolution
- dependency resolution
- shaderpack and resource pack management
- integrate into multimc or theseus
- graphical frontend (w/ notifications?)

[Modrinth REST API docs](https://github.com/modrinth/labrinth/wiki/API-Documentation)

# File Architecture

```bash
- .config/hopper/config.toml # Main config file
- .local/share/multimc/instances/*/.minecraft/Hopfile.toml # Multimc
- .minecraft/Hopfile.toml # Official launcher
- .var/app/com.mojang.Minecraft/.minecraft/Hopfile.toml # Flatpak version
- .cache/hopper/ # Mod cache
|              - hopper.lock # Lock file
|              - mod1.jar # Mods
|              - mod2.jar
+------------- - ...
```

# Usage (Planned)

Create `Hopfile.toml` in your instance directory:
```
hopper init
```

Add mods:
```
hopper add iris
hopper add sodium
hopper add phosphor
```

Check for mod updates:
```
hopper update
```

# Docs (Planned)

## `hopper init`

```
hopper init < --dir=./path/to/instance >
```

Inits in current directory if `dir` is left out, otherwise inits in given dir.

## `hopper update`

```
hopper update < --mc-version=1.17 >
```

Updates all installed mods of a specific version, or a version set in the config.

## `hopper add`

```
$ hopper add sodium --mc-version 1.17
4 Indium 1.0.0+mc1.17.1 [1.17.1] (21557 downloads)
    Sodium addon providing support for the Fabric Rendering API, based on Indigo
3 Reese's Sodium Options 1.2.1 [1.16.5] (548 downloads)
    Alternative Options Menu for Sodium
2 Sodium Extra mc1.17.1-0.3.6 [1.17.1] (16387 downloads)
    Features that shouldn't be in Sodium.
1 Sodium mc1.17.1-0.3.2 [1.17.1] (962361 downloads)
    Modern rendering engine and client-side optimization mod for Minecraft
:: Select a mod
:: ...
```

## `hopper get`

Just like `hopper add` but simply downloads a mod jar to the current directory.
