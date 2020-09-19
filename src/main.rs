extern crate clap;
use clap::{App, Arg, SubCommand};
use pmsg::*;
use std::convert::TryFrom;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

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
        encode(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("decode") {
        decode(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        remove(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("print") {
        print(matches)?;
    }

    Ok(())
}

fn encode(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = matches.value_of("file").unwrap();
    let file_pathbuf = PathBuf::from(file_path);
    let mut file = OpenOptions::new().read(true).open(&file_pathbuf)?;

    let mut png_data = Vec::new();
    file.read_to_end(&mut png_data)?;
    let mut png = Png::try_from(png_data.as_ref())?;

    let eof_chunk = png.remove_chunk("IEND")?;
    let chunk_type = matches.value_of("chunk_type").unwrap();
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let message = matches.value_of("message").unwrap();
    let message = message.as_bytes().to_vec();

    png.append_chunk(Chunk::new(chunk_type, message)?);
    png.append_chunk(eof_chunk);

    let output = matches.value_of("output").unwrap_or(file_path);
    let output = PathBuf::from(output);
    let mut output = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output)?;
    output.write_all(&png.as_bytes())?;

    Ok(())
}

fn decode(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = PathBuf::from(matches.value_of("file").unwrap());
    let mut file = OpenOptions::new().read(true).open(&file_path)?;

    let mut png_data = Vec::new();
    file.read_to_end(&mut png_data)?;
    let png = Png::try_from(png_data.as_ref())?;

    let chunk_type = matches.value_of("chunk_type").unwrap();
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunks = png
        .chunks()
        .iter()
        .filter(|c| *c.chunk_type() == chunk_type)
        .collect::<Vec<&Chunk>>();

    for c in chunks {
        println!("{}", c);
    }

    Ok(())
}

fn remove(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = PathBuf::from(matches.value_of("file").unwrap());
    let mut file = OpenOptions::new().read(true).open(&file_path)?;

    let mut png_data = Vec::new();
    file.read_to_end(&mut png_data)?;
    let mut png = Png::try_from(png_data.as_ref())?;

    let chunk_type = matches.value_of("chunk_type").unwrap();
    let removed = png.remove_chunk(chunk_type)?;

    let output = PathBuf::from(file_path);
    let mut output = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output)?;
    output.write_all(&png.as_bytes())?;

    println!("Remove {}", removed);
    Ok(())
}

fn print(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = PathBuf::from(matches.value_of("file").unwrap());
    let mut file = OpenOptions::new().read(true).open(&file_path)?;

    let mut png_data = Vec::new();
    file.read_to_end(&mut png_data)?;
    let png = Png::try_from(png_data.as_ref())?;
    println!("{}", png);

    Ok(())
}
