extern crate clap;
use clap::{App, Arg, SubCommand};
use pmsg::*;
mod commands;

fn main() -> Result<()> {
    let version = "1.0.1";
    let author = "Tung L. Vo <tlv8864@tutanota.com>";

    let matches = App::new("PMSG")
        .version(version)
        .author(author)
        .about("Hide message(s) inside PNG file.")
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encode the message to the PNG file.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("PNG file")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("chunk_type")
                        .help("Chunk type code of message")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("message")
                        .help("Hidden message")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Output file")
                        .required(false)
                        .index(4),
                ),
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decode hidden messages in the PNG file.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("PNG file")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("chunk_type")
                        .help("Chunk type code of message")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove hidden messages in the PNG file.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("PNG file")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("chunk_type")
                        .help("Chunk type code of message")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("print")
                .about("Print raw data from the PNG file.")
                .version(version)
                .author(author)
                .arg(
                    Arg::with_name("file")
                        .help("PNG file")
                        .required(true)
                        .index(1),
                )
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
