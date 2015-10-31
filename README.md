# gmktool
Tool for manipulating Game Maker Studio data files

## Building
gmktool is written in [Rust](https://www.rust-lang.org/).
You need to install the latest Rust **nightly** release.

Once you have done that, you can build gmktool with `cargo build --release`

## Usage
```
gmktool 0.1.0
Tool for manipulating Game Maker Stdudio data files

USAGE:
	gmktool [FLAGS] [OPTIONS] <DATA_FILE> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --backup <BACKUP_FILE>    Create a backup. By default, it's DATA_FILE.bk

ARGS:
    DATA_FILE    The game maker data file to operate on

SUBCOMMANDS:
    help        Prints this message
    strings     Manipulate strings
    textures    Manipulate textures

```
