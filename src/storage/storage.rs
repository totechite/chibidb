use crate::storage::catalog::Catalog;
use crate::storage::util::gen_hash;
use crate::storage::buffer_pool::BufferPool;
use crate::storage::data::Tuple;
use crate::storage::disk_manager;
use std::hash::Hash;
use crate::storage::page::Page;

#[derive(Default, Debug)]
struct Storage {
    buf_pool: BufferPool,
    catalog: Catalog,
}

impl Storage {
    fn new() -> Self {
        Storage {
            catalog: Catalog::load().unwrap(),
            ..Default::default()
        }
    }

    fn read_table(&mut self, table_name: impl ToString + Hash) -> Option<Vec<Page>> {
        let mut pages = vec![];
        let scheme = {
            let table_id = gen_hash(&table_name);
            self.catalog.schemes.get(&table_id)
        };
        return match scheme {
            Some(scheme) => {
                for i in 0..scheme.page_num {
                    if let Some(page) = self.buf_pool.read_page(scheme.table_id, i) {
                        pages.push(page.page.clone());
                    } else {
                        if let Ok(page) = disk_manager::read_page(scheme.table_id, i) {
                            pages.push(page.page.clone());
                        } else { break; };
                    };
                }
                Some(pages)
            }
            None => None
        };
    }
}





