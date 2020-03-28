mod data;
mod page;
mod tuple;
mod disk_manager;
mod catalog;
mod buffer_pool;
mod storage;

mod magic_number {
    pub(crate) const PAGE_SIZE: usize = 8192; // 8KB
}

pub mod util {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    use serde::{Serialize, Deserialize};
    use crate::storage::page::Page;

    pub(crate) fn gen_hash(t: &impl Hash) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    pub(crate) fn compress(b: &[u8]) -> Vec<u8> {
//        FixMe: Snappy.compress()
        b.to_vec()
    }

    pub(crate) fn uncompress(b: &[u8]) -> Vec<u8> {
//        FixMe: Snappy.uncompress()
        b.to_vec()
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum ColumnType {
        text,
        varchar(usize),
        char(usize),
        bigint,
        real,
        bool,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Scheme {
        pub table_id: u64,
        pub table_name: String,
        pub page_num: u16,
        // keep order of field.
        pub column: Vec<(ColumnType, String)>,
        // don't need keep order.
        pub index: Vec<(ColumnType, String)>,
    }

    #[derive(Default, Clone, Debug)]
    pub struct PageAuxiliar {
        pub table_id: u64,
        pub page_id: u16,
        pub page: Page,
    }
}