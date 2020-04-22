use crate::storage::catalog::Catalog;
use crate::storage::util::{gen_hash, PageAuxiliar, Scheme};
use crate::storage::buffer_pool::BufferPool;
use crate::storage::data::{Tuple, TupleData, TupleData_Type};
use crate::storage::disk_manager;
use std::hash::Hash;
use crate::storage::page::Page;
use std::path::PathBuf;
use std::fs::{read_dir, DirEntry, File, create_dir, create_dir_all};
use std::cell::RefCell;
use crate::sql::plan::{Type, FieldDefinition};
use crate::storage::disk_manager::create_page;
use protobuf::RepeatedField;

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

    pub fn insert_records(&self, scheme: &Scheme, records: Vec<Vec<Option<String>>>) {
        let mut page_data = vec![];
        let (mut tuple, mut tuple_data) = (Tuple::new(), RepeatedField::new());

        for record in records {
            for i in 0..scheme.column.len() {
                if let Some(value) = record[i].clone() {
                    match scheme.column[i].0 {
                        Type::integer => {
                            let mut data = TupleData::new();
                            data.set_field_type(TupleData_Type::INT);
                            data.set_number(value.parse().unwrap());
                            tuple_data.push(data);
                        }
                        Type::text => {
                            let mut data = TupleData::new();
                            data.set_field_type(TupleData_Type::STRING);
                            data.set_string(value);
                            tuple_data.push(data);
                        }
                    }
                } else {
                    let mut data = TupleData::new();
                    data.set_field_type(TupleData_Type::NULL);
                    tuple_data.push(data);
                }
            }
        }

        tuple.set_data(tuple_data);
        page_data.push(tuple);
        let page = Page {
            id: scheme.page_num + 1,
            tuples: page_data,
        };
        create_page(&page, scheme.table_name.clone());
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
            for p in [&std::env::var("CHIBIDB_DATA_PATH").unwrap(), &(table_name.clone() + "/")].iter() {
                path.push(p);
            }
            path
        };
        match create_dir_all(path) {
            Ok(_) => {
                let table_id = gen_hash(&table_name);
                let scheme = Scheme {
                    table_id,
                    table_name,
                    page_num: 0,
                    column: fields.into_iter().map(|f| (f.T, f.name)).collect::<Vec<(Type, String)>>(),
                    index: Vec::new(),
                };
                self.catalog.add(scheme);
                return true;
            }
            Err(e) => false
        }
    }
}

mod test {
    use crate::storage::storage::Storage;
    use crate::sql::plan::{FieldDefinition, Type};

    #[test]
    fn create_table() {
        let table_name = "test_table".to_string();
        let fields = vec![
            FieldDefinition { name: "id".to_string(), T: Type::integer },
            FieldDefinition { name: "name".to_string(), T: Type::text }
        ];
        let mut s = Storage::new();
        s.create_table(table_name, fields);
    }
}

