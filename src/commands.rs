extern crate clap;

use pmsg::*;

use std::convert::TryFrom;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;

pub fn encode(matches: &clap::ArgMatches) -> Result<()> {
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

pub fn decode(matches: &clap::ArgMatches) -> Result<()> {
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

pub fn remove(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = PathBuf::from(matches.value_of("file").unwrap());
    let mut file = OpenOptions::new().read(true).open(&file_path)?;

    let mut png_data = Vec::new();
    file.read_to_end(&mut png_data)?;
    let mut png = Png::try_from(png_data.as_ref())?;

    let chunk_type = matches.value_of("chunk_type").unwrap();
    let removed = png.remove_chunk(chunk_type)?;

    let mut output = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_path)?;
    output.write_all(&png.as_bytes())?;

    println!("Remove {}", removed);
    Ok(())
}

pub fn print(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = PathBuf::from(matches.value_of("file").unwrap());
    let mut file = OpenOptions::new().read(true).open(&file_path)?;

    let mut png_data = Vec::new();
    file.read_to_end(&mut png_data)?;
    let png = Png::try_from(png_data.as_ref())?;
    println!("{}", png);

    Ok(())
}
