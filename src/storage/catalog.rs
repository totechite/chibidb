use std::env;
use crate::storage::util::Scheme;
use std::fs::{File, ReadDir, read_dir};
use std::io::{BufWriter, IntoInnerError, Error, Write, Read};
use std::env::VarError;
use std::collections::{HashSet, HashMap};
use serde_json::Deserializer;
use std::path::{Path, PathBuf};
use crate::storage::magic_number::PAGE_SIZE;

#[derive(Default, Debug)]
pub struct Catalog {
    pub schemes: HashMap<u64, Scheme>
}

impl Catalog {
    fn add(&mut self, s: Scheme) {
        self.schemes.insert(s.table_id, s);
    }
}

impl Catalog {
    fn new() -> Self { Default::default() }

    pub fn save(s: Scheme) -> Result<(), Error> {
        let mut f = {
            let SCHEME_PATH = env::var("CHIBIDB_SCHEME_PATH").unwrap();
            let mut path = PathBuf::from(SCHEME_PATH);
            path.push(format!("{:?}.scheme ", s.table_id));
            let mut f = File::create(path)?;
            BufWriter::new(f)
        };
        let json: String = serde_json::to_string(&s)?;
        f.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<Self, Error> {
        let mut schemes = HashMap::new();
        let mut scheme_dir = {
            let SCHEME_PATH = env::var("CHIBIDB_SCHEME_PATH").unwrap();
            read_dir(format!("{:?}", SCHEME_PATH))?
        };
        for f in scheme_dir {
            let mut buf = [0u8; PAGE_SIZE] ;
            File::open(f?.file_name())?.read(&mut buf)?;
            let scheme: Scheme = serde_json::from_slice::<Scheme>(&buf)?;
            schemes.insert(scheme.table_id, scheme);
        }
        Ok(Catalog { schemes })
    }

}
