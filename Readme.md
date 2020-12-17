# Warp-Drive

## Commands

use `warp` and the commands below to go directly to your favorite directories

### <name\>
Warp directly to `<name>`

### dump
Dumps the config file to stdout

### edit
Opens the config file with your `$EDITOR`

### init
Used to get the wrapper function you need to use this in your shell. The function is written to be as POSIX compatible as possible, meaning it _should_ work in virtually all shells.

### add <name\>
Add the current directory as a waypoint that you can call with `<name>`

### eject <name\>
Remove the waypoint with the associated `<name>` 


### to <name\>
Go to the waypoint referenced by `<name>`!

### ls,list [-l,--long]
List waypoints you have saved. If you pass `-l/--long`, you will also see the associated file paths

## Getting Started

You should add `eval "$(warp-drive-bin init)"` someplace where your shell will pick it up on start (i.e. .bashrc or .zshrc)

## Config file

The config file is provided by [directories](https://docs.rs/directories). It should default to the system defaults. On linux, it should respect `$XDG_CONFIG_HOME` and use `$HOME/.config` if the former is not set.

`$XDG_CONFIG_HOME/warp_drive/warp_drive.toml` or `$HOME/.confg/warp_drive/warp_drive.toml`

## Shell Completion

Right now I have a completion for Zsh (because that is the shell that I currently use). It isn't particularly pretty, but it will complete the names for your waypoints from the config in addition to the subcommands and appropriate options for them.

## Installation

For now:

1. Clone Repo
2. Build with Cargo `cargo build --release` (developed with rustc 1.45.0)
3. Make sure `warp-drive-bin` is in your path
4. add `eval "$(warp-drive-bin init)"` someplace your shell will pick it up
5. Warp Speed Ahead!

## The nitty gritty

`warp-drive` is inspired by programs such as [ZSH-z](https://github.com/agkozak/zsh-z), [goto](https://github.com/iridakos/goto), [warp-dir](https://github.com/kigster/warp-dir), [wd](https://github.com/mfaeravaag/wd), [DirectoryWarp](https://github.com/LtHummus/DirectoryWarp), [zoxide](https://github.com/ajeetdsouza/zoxide), which is also written in Rust, and I'm sure more that I just haven't found yet.

Instead of being written in a shell language or relying on the presence of an interpreter, the heavy lifting is passed off to a Rust binary and exposes a thin shell function to execute the needed system commands in the current context (DirectoryWarp does this with Go and I do this also in [assimilate](https://github.com/fvhockney/assimilate)).

## Licensed under the MIT license
