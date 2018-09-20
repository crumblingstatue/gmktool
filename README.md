# NOTE: Currently this project is BROKEN and unmaintained. IT DOESN'T WORK.
# Don't use it, unless you want to work on it.

# gmktool
Tool for manipulating Game Maker Studio data files.

Powered by [rgmk](https://github.com/crumblingstatue/rgmk).

## Building
gmktool is written in [Rust](https://www.rust-lang.org/).
You need to install the latest Rust **nightly** release.

Once you have done that, you can build gmktool with `cargo build --release`

## Usage

### Help
```
gmktool 0.1.0
Tool for manipulating Game Maker Stdudio data files

USAGE:
	gmktool [FLAGS] <DATA_FILE> [SUBCOMMAND]

FLAGS:
    -b, --backup     Create a backup (.bk) before writing to the data file.
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    DATA_FILE    The game maker data file to operate on

SUBCOMMANDS:
    help        Prints this message
    strings     Manipulate strings
    textures    Manipulate textures

```
### Example usage
```sh
# Dump strings to strings.txt
gmktool data.win strings dump strings.txt
# Now edit them with a text editor or something.
# And finally, repack the strings
gmktool data.win strings repack strings.txt
```
