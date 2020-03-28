use serde::{Serialize, Deserialize};
use std::fs::{File, read_dir};
use std::io::{Read, Error, BufWriter, Write, BufReader};
use crate::storage::magic_number::PAGE_SIZE;
use crate::storage::page::function::{serialize_page, deserialize_page};
use crate::storage::page::Page;
use crate::storage::util;
use protobuf::ProtobufError;
use std::hash::Hash;
use std::env;
use crate::storage::util::{Scheme, PageAuxiliar};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub fn read_page(table_id: u64, page_id: u16) -> Result<PageAuxiliar, ProtobufError> {
    let mut path = PathBuf::new();
    path.push(format!("{:?}", table_id));

    let mut buf = [0u8; PAGE_SIZE];
    path.push(format!("{:?}.page", page_id));
    BufReader::new(File::open(path)?).read_exact(&mut buf);
    let page = deserialize_page(buf)?;

    Ok(PageAuxiliar {
        table_id,
        page_id,
        page,
    })
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
