use std::path::Path;
use crate::storage::magic_number::PAGE_SIZE;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
use crate::storage::page::function::{deserialize_page, serialize_page};
use crate::storage::page::Page;

pub fn read_page(path: &Path) -> Result<Page, std::io::Error> {
    let mut buf = [0u8; PAGE_SIZE];
    BufReader::new(File::open(path)?).read_exact(&mut buf);
    Ok(deserialize_page(buf)?)
}

pub fn create_page(p: &Page) -> Result<(), std::io::Error> {
    let mut buf = {
        let mut file = File::create(format!("{:?}.page", p.id))?;
        BufWriter::new(file)
    };
    let (bytes, surplus) = serialize_page(p)?;
    buf.write(&bytes)?;
    Ok(())
}
