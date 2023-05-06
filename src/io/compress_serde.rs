use crate::io::Params;
use bincode::{deserialize_from, serialize_into};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, Read, Write};
use std::path::PathBuf;
use zstd::{Decoder, Encoder};

type Result<T> = std::result::Result<T, Error>;

fn compress<T: Write>(data: &HashMap<String, Params>, writer: &mut T) -> Result<()> {
    let mut encoder = Encoder::new(writer, 0)?;
    serialize_into(&mut encoder, data);
    encoder.finish().map_err(|e| Error::new(Error::Other, e))?;
    writer.flush()?;
    Ok(())
}

fn decompress<T: Read>(reader: &mut T) -> Result<HashMap<String, Params>> {
    let mut decoder = Decoder::new(reader)?;
    let data: HashMap<String, Params> = deserialize_from(&mut decoder)?;
    Ok(data)
}

// Compress to file
fn compress_to_file(path: PathBuf, data: &HashMap<String, Params>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    compress(data, &mut writer);
    Ok(())
}
