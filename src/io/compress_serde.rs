use crate::io::Params;
use bincode::{deserialize_from, serialize_into};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind::NotFound;
use std::io::{BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Write};
use std::path::PathBuf;
use zstd::{Decoder, Encoder};

type Result<T> = std::result::Result<T, Error>;

fn compress<T: Write>(data: &HashMap<String, Params>, mut writer: T) -> Result<()> {
    let mut encoder = Encoder::new(&mut writer, 0)?;
    serialize_into(&mut encoder, data)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Zstd encoder error {}", e)))?;
    encoder
        .finish()
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    writer.flush()?;
    Ok(())
}

fn decompress<T: Read>(reader: &mut T) -> Result<HashMap<String, Params>> {
    let mut decoder = Decoder::new(reader)?;
    let data: HashMap<String, Params> = deserialize_from(&mut decoder)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Zstd decoder error {}", e)))?;
    Ok(data)
}

// Compress to file
pub fn compress_to_file(path: PathBuf, data: &HashMap<String, Params>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    compress(data, &mut writer)?;
    Ok(())
}

// Decompress from file
pub fn decompress_from_file(path: PathBuf) -> Result<HashMap<String, Params>> {
    let mut file = OpenOptions::new().read(true).open(&path).or_else(|e| {
        if e.kind() == NotFound {
            File::create(path.as_path())
        } else {
            Err(e)
        }
    })?;

    // Check if the file is empty
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok::<HashMap<String, Params>, Error>(HashMap::new());
    }
    //

    let mut reader = BufReader::new(Cursor::new(buffer));
    decompress(&mut reader)
}
