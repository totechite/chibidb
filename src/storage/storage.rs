use crate::storage::catalog::Catalog;
use crate::storage::util::{gen_hash, PageAuxiliar, Scheme};
use crate::storage::buffer_pool::BufferPool;
use crate::storage::data::Tuple;
use crate::storage::disk_manager;
use std::hash::Hash;
use crate::storage::page::Page;
use std::path::PathBuf;
use std::fs::{read_dir, DirEntry, File};
use std::cell::RefCell;
use crate::sql::plan::{Type, FieldDefinition};

#[derive(Default, Debug)]
pub struct Storage {
    buf_pool: RefCell<BufferPool>,
    pub catalog: Catalog,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            catalog: Catalog::load().unwrap(),
            ..Default::default()
        }
    }

    pub fn read_table(&self, table_name: impl ToString + Hash) -> (&Scheme, Vec<PageAuxiliar>) {
        let mut scheme = {
            let table_id = gen_hash(&table_name);
            self.catalog.schemes.get(&table_id).unwrap()
        };
        let mut page_paths = {
            let mut path = PathBuf::new();
            for s in [std::env::var("CHIBIDB_DATA_PATH").unwrap(), scheme.table_id.to_string()].iter() {
                path.push(s);
            }
            read_dir(path).unwrap()
        };
        let mut pages = vec![];
        for page_path in page_paths.filter_map(Result::ok).collect::<Vec<DirEntry>>() {
            let page_id = String::from(page_path.path().as_path().file_stem().unwrap().to_str().unwrap().clone()).parse().unwrap();
            match self.buf_pool.borrow_mut().read_page(scheme.table_id, page_id) {
                Some(page) => {
//                    from cache
                    pages.push(page.clone());
                }
                None => {
//                    from disk
                    let page = PageAuxiliar { table_id: scheme.table_id, page_id, page: disk_manager::read_page(page_path.path().as_path()).unwrap() };
                    pages.push(page);
                }
            };
        }
        (scheme, pages)
    }

    pub fn create_table(&mut self, table_name: String, fields: Vec<FieldDefinition>) -> bool {
        let path = {
            let mut path = PathBuf::new();
            [&std::env::var("CHIBIDB_DATA_PATH").unwrap(), &table_name].iter().for_each(|p| path.push(p));
            path
        };
        match File::create(path) {
            Ok(_) => {
                let table_id = gen_hash(&table_name);
                let scheme = Scheme {
                    table_id,
                    table_name,
                    page_num: 0,
                    column: fields.into_iter().map(|f| (f.T, f.name)).collect::<Vec<(Type, String)>>(),
                    index: Vec::new(),
                };
                self.catalog.schemes.insert(table_id, scheme);
                return true;
            }
            Err(_) => false
        }
    }
}





