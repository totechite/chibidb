use crate::sql::plan::{SELECT, Field, Table, CREATE};
use std::collections::HashMap;
use crate::storage::util::{gen_hash, PageAuxiliar, Scheme};
use crate::storage::storage::Storage;
use crate::storage::page::Page;
use std::ops::Index;
use crate::storage::data::{TupleData, TupleData_Type};

pub struct Executor {
    storage: Storage
}

impl Executor {
    pub fn select(&mut self, s: SELECT) {
        let tables: HashMap<u64, (&Scheme, Vec<PageAuxiliar>)> = self.get_table(s.FROM.unwrap());
        let mut display_records: (Vec<String>, Vec<Vec<String>>) = (vec![], vec![vec![]]);
        for field in s.fields {
            match field {
                All => {}
                Field::Plain { name, table_name, AS } => {
                    for (scheme, table) in tables.values() {
                        if let Some(index) = scheme.column.iter().map(|(a, b)| b).position(|column_name| column_name == &name) {
                            let records = get_column(&table, index);
                            let display_name = AS.unwrap_or(name);
                            display_records.0.push(display_name);
                            display_records.1.push(records);
                            break;
                        }
                    }
                }
                Field::Calc { expr, name, table_name, AS } => {}
            }
        }
        println!("{:#?}", display_records);
    }

    pub fn create_table(c: CREATE) {}

    pub fn insert() {}

    fn get_table(&mut self, tables: Vec<Table>) -> HashMap<u64, (&Scheme, Vec<PageAuxiliar>)> {
        let mut hashmap = HashMap::new();
        for table in tables {
            let pages = self.storage.read_table(&table.name);
            hashmap.insert(gen_hash(&table.name), pages);
        }
        hashmap
    }
}

fn get_column(table: &Vec<PageAuxiliar>, index: usize) -> Vec<String> {
    table.into_iter().map(|p: &PageAuxiliar|
        p.page.tuples.iter().map(
            |t| t.data.get(index).unwrap()
        ).collect::<Vec<&TupleData>>()
    ).collect::<Vec<Vec<&TupleData>>>().concat().iter()
        .map(|td| {
            match td.field_type {
                TupleData_Type::INT => format!("{:?}", td.number),
                TupleData_Type::STRING => format!("{:?}", td.string)
            }
        }).collect::<Vec<String>>()
}