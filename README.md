# High-level Goals

- modrinth mod searching
- modrinth mod installation
- per-instance mod management
- mod updating

If modrinth supports it:
- conflict resolution
- dependency resolution

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
$ hopper add xx
3> xxxy
  Description for xxxy
2> xxyz
  Description for xxyz
1> xx
  Description for xx
:: Select a mod
:: ...
```

## `hopper get`

Just like `hopper add` but simply downloads a mod jar.
