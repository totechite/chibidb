use crate::sql::plan::{SELECT, Field, Table, CREATE, INSERT};
use std::collections::HashMap;
use crate::storage::util::{gen_hash, PageAuxiliar, Scheme};
use crate::storage::storage::Storage;
use crate::storage::page::Page;
use std::ops::Index;
use crate::storage::data::{TupleData, TupleData_Type};
use protobuf::well_known_types::Type;
use crate::sql::lexer::{select_parse, create_parse};

pub struct Executor {
    pub storage: Storage
}

impl Executor {
    pub fn parse(&mut self, token: Vec<String>) {
        let t = token.first().unwrap();
        match t.as_str() {
            "SELECT" => { self.select(select_parse(token)) }
            "INSERT" => {}
            "CREATE" => { self.create_table(create_parse(token)) }
            default => {}
        }
    }

    pub fn select(&mut self, s: SELECT) {
        let tables: HashMap<u64, (&Scheme, Vec<PageAuxiliar>)> = self.get_table(s.FROM.unwrap());
        let mut display_records: (Vec<String>, Vec<Vec<String>>) = (vec![], vec![]);
        println!("{:?}", s.fields);
        for field in s.fields {
            match field {
                All => {
                    for (scheme, table) in tables.values() {
                        for (index, field) in scheme.column.iter().enumerate(){
                            let records = get_column(&table, index);
                            display_records.0.push(field.clone().1);
                            display_records.1.push(records);

                        }
                    }
                }
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

    pub fn create_table(&mut self, c: CREATE) {
        if let Some((table_name, fields)) = c.TABLE {
            self.storage.create_table(table_name, fields);
        }
    }

    pub fn insert(&mut self, i: INSERT) {
        let table_name = i.INTO.0;
        let records = i.VALUES;
        let scheme = self.storage.catalog.schemes.get(&gen_hash(&table_name)).unwrap();
        let mut optional_records = vec![vec![]];
        let fields = if let Some(fields) = i.INTO.1 {
            for (idx, sch )in scheme.column.iter().enumerate() {
                let mut optional_record = vec![];
                if fields.contains(&sch.1){
                    for record in &records {
                        optional_record.push(Some(record[idx].clone()))
                    }
                }else { optional_record.push(None); }
                optional_records.push(optional_record);
            }
        } else { optional_records =  records.iter().map(|record| record.iter().map(|v|Some(v.clone())).collect::<Vec<Option<String>>>()).collect::<Vec<Vec<Option<String>>>>(); };
        self.storage.insert_records(scheme, optional_records);
    }

    fn get_table(&mut self, tables: Vec<String>) -> HashMap<u64, (&Scheme, Vec<PageAuxiliar>)> {
        let mut hashmap = HashMap::new();
        for table in tables {
            let pages = self.storage.read_table(&table);
            hashmap.insert(gen_hash(&table), pages);
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
                TupleData_Type::STRING => format!("{:?}", td.string),
                TupleData_Type::NULL => format!("null"),
            }
        }).collect::<Vec<String>>()
}