extern crate clap;
use clap::{App, Arg, SubCommand};
use pmsg::*;
mod commands;

fn main() -> Result<()> {
    let version = "0.1.0";
    let author = "Tung L. Vo <tlv8864@tutanota.com>";

    let matches = App::new("PMSG")
        .version(version)
        .author(author)
        .about("Hide message(s) inside PNG file.")
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encode the given message inside the PNG file located at the given path.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("chunk_type")
                        .help("Type code of the chunk containing the message")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("message")
                        .help("The message to be encoded")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Sets the output file to use")
                        .required(false)
                        .index(4),
                ),
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decode the given message inside the PNG file located at the given path.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("chunk_type")
                        .help("Type code of the chunk containing the message")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Encode the given message inside the PNG file located at the given path.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("chunk_type")
                        .help("Type code of the chunk containing the message")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("print")
                .about("Encode the given message inside the PNG file located at the given path.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("encode") {
        commands::encode(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("decode") {
        commands::decode(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        commands::remove(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("print") {
        commands::print(matches)?;
    }

    Ok(())
}
