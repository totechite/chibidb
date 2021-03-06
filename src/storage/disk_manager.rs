use std::path::{Path, PathBuf};
use crate::storage::magic_number::PAGE_SIZE;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
use crate::storage::page::function::{deserialize_page, serialize_page};
use crate::storage::page::Page;
use crate::storage::catalog::Catalog;
use crate::sql::plan::{FieldDefinition, Type};
use crate::storage::util::{gen_hash, Scheme};

pub fn read_page(path: &Path) -> Result<Page, std::io::Error> {
    let mut buf = [0u8; PAGE_SIZE];
    BufReader::new(File::open(path)?).read_exact(&mut buf);
    Ok(deserialize_page(buf)?)
}

pub fn create_page(p: &Page, table_name: String) -> Result<(), std::io::Error> {
    let mut buf = {
        let mut file = File::create(format!("{}{}/{:?}.page", std::env::var("CHIBIDB_DATA_PATH").unwrap(), table_name, p.id))?;
        BufWriter::new(file)
    };
    let (bytes, surplus) = serialize_page(p)?;
    if let Some(surplus) = surplus {
        create_page(&Page {
            id: p.id + 1,
            tuples: surplus,
        }, table_name)?;
    }
    buf.write(&bytes)?;
    Ok(())
}

