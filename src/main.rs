extern crate rgmk;
extern crate clap;

use clap::{App, SubCommand, AppSettings};
use std::io::{self, stdin, stdout, stderr, BufReader};
use std::io::prelude::*;
use rgmk::GameData;
use std::path::Path;

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let strings_subcommand = SubCommand::with_name("strings")
                                 .about("Manipulate strings")
                                 .setting(AppSettings::SubcommandRequiredElseHelp)
                                 .setting(AppSettings::ArgRequiredElseHelp)
                                 .subcommand(SubCommand::with_name("dump")
                                                 .about("Dump strings.")
                                                 .args_from_usage("[OUTPUT_FILE] 'The file to \
                                                                   write the strings to. By \
                                                                   default, standard output.'"))
                                 .subcommand(SubCommand::with_name("repack")
                                                 .about("Repack strings")
                                                 .args_from_usage("[INPUT_FILE] 'The file \
                                                                   containing the strings to \
                                                                   repack. By default, standard \
                                                                   input.'"));
    let textures_subcommand = SubCommand::with_name("textures")
                                  .about("Manipulate textures")
                                  .setting(AppSettings::SubcommandRequiredElseHelp)
                                  .subcommand(SubCommand::with_name("dump").about("Dump textures"))
                                  .subcommand(SubCommand::with_name("repack")
                                                  .about("Repack textures"));
    let matches = App::new("gmktool")
                      .setting(AppSettings::SubcommandRequiredElseHelp)
                      .version(env!("CARGO_PKG_VERSION"))
                      .about("Tool for manipulating Game Maker Stdudio data files")
                      .args_from_usage("-b --backup=[BACKUP_FILE] 'Create a backup. By default, \
                                        it's DATA_FILE.bk'
                                    \
                                        <DATA_FILE> 'The game maker data file to operate on'")
                      .subcommand(strings_subcommand)
                      .subcommand(textures_subcommand)
                      .get_matches();
    let data_file_path = matches.value_of("DATA_FILE").unwrap();
    let backup = matches.is_present("backup");
    let backup_path = format!("{}.bk", data_file_path);
    let backup_path = matches.value_of("BACKUP_FILE").unwrap_or(&backup_path);
    let mut game_data = match GameData::from_file(data_file_path) {
        Ok(g) => g,
        Err(e) => {
            writeln!(stderr(),
                     "Failed to load game data for {:?}: {}",
                     data_file_path,
                     e)
                .unwrap();
            return 1;
        }
    };
    if let Some(matches) = matches.subcommand_matches("strings") {
        if let Some(matches) = matches.subcommand_matches("dump") {
            match matches.value_of("OUTPUT_FILE") {
                Some(path) => {
                    let mut file = match std::fs::File::create(path) {
                        Ok(f) => f,
                        Err(e) => {
                            writeln!(stderr(), "Failed to create output file: {}", e).unwrap();
                            return 1;
                        }
                    };
                    if let Err(e) = dump_strings(&game_data, &mut file) {
                        writeln!(stderr(), "Failed to dump strings: {}", e).unwrap();
                    }
                }
                None => {
                    let stdout = stdout();
                    let mut lock = stdout.lock();
                    if let Err(e) = dump_strings(&game_data, &mut lock) {
                        writeln!(stderr(), "Failed to dump strings: {}", e).unwrap();
                    }
                }
            }
        } else if let Some(matches) = matches.subcommand_matches("repack") {
            if backup {
                if let Err(e) = std::fs::copy(data_file_path, backup_path) {
                    writeln!(stderr(), "Failed to create backup: {}", e).unwrap();
                }
            }
            match matches.value_of("INPUT_FILE") {
                Some(path) => {
                    let file = match std::fs::File::open(path) {
                        Ok(f) => f,
                        Err(e) => {
                            writeln!(stderr(), "Failed to open input file: {}", e).unwrap();
                            return 1;
                        }
                    };
                    if let Err(e) = repack_strings(&mut game_data,
                                                   &mut BufReader::new(file),
                                                   data_file_path) {
                        writeln!(stderr(), "Failed to repack strings: {}", e).unwrap();
                    }
                }
                None => {
                    if let Err(e) = repack_strings(&mut game_data,
                                                   &mut BufReader::new(stdin()),
                                                   data_file_path) {
                        writeln!(stderr(), "Failed to repack strings: {}", e).unwrap();
                    }
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("textures") {
        if let Some(_) = matches.subcommand_matches("dump") {
            if let Err(e) = dump_textures(&game_data) {
                writeln!(stderr(), "Failed to dump textures: {}", e).unwrap();
                return 1;
            }
        } else if let Some(_) = matches.subcommand_matches("repack") {
            if let Err(e) = repack_textures(&mut game_data, data_file_path) {
                writeln!(stderr(), "Failed to repack textures: {}", e).unwrap();
            }
        }
    }
    0
}

fn dump_strings<W: Write>(game_data: &GameData, writer: &mut W) -> io::Result<()> {
    for s in &game_data.strings.strings {
        try!(writeln!(writer, "{}", s));
    }
    Ok(())
}

fn repack_strings<R: BufRead, P: AsRef<Path>>(game_data: &mut GameData,
                                              reader: &mut R,
                                              path: P)
                                              -> io::Result<()> {
    for (s, l) in game_data.strings.strings.iter_mut().zip(reader.lines()) {
        *s = try!(l);
    }
    game_data.save_to_file(path)
}

fn dump_textures(game_data: &GameData) -> io::Result<()> {
    for (i, t) in game_data.textures.textures.iter().enumerate() {
        let path = format!("{}.png", i);
        let mut f = try!(std::fs::File::create(path));
        try!(f.write_all(&t.png_data));
    }
    Ok(())
}

fn repack_textures<P: AsRef<Path>>(game_data: &mut GameData, path: P) -> io::Result<()> {
    for (i, t) in game_data.textures.textures.iter_mut().enumerate() {
        let path = format!("{}.png", i);
        let mut f = try!(std::fs::File::open(path));
        let mut png_data = Vec::new();
        try!(f.read_to_end(&mut png_data));
        (*t).png_data = png_data;
    }
    game_data.save_to_file(path)
}
