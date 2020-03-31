use crate::storage::magic_number::PAGE_SIZE;
use std::u16;
use protobuf::ProtobufError;
use crate::storage::data::Tuple;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Page {
    pub id: u16,
    pub tuples: Vec<Tuple>,
}

impl Page {
    pub fn new() -> Self {
        Default::default()
    }
}

pub mod function {
    use crate::storage::page::Page;
    use crate::storage::data::Tuple;
    use crate::storage::tuple::functions::{serialize_tuple, deserialize_tuple};
    use crate::storage::magic_number::PAGE_SIZE;
    use protobuf::ProtobufError;
    use std::{u16, mem};
    use std::io::{Cursor, Read};
    use std::io::Write;
    use crate::storage::data;
    use std::path::Component::CurDir;
    use crate::storage::util::gen_hash;
    use std::time::SystemTime;
    use std::slice::Iter;

//  ===========Page Layout===========
//  |[attribute]  |  [size(byte)]
//  |-------------------------------------
//  | id          |      2
//  |-------------------------------------
//  | tuple_num   |      8
//  |-------------------------------------
//  |slots[]      | slot  | tuple_id    4
//  |             |       | tuple_address 1
//  |             |  6    | tuple_size    1
//  |             |-------|---------------
//  |             |       |   :
//  |_____________|_______|_______________
//  |     :
//  |     :
//  |--------------------------------
//  | tuple[]    |  variable_size
//  |--------------------------------

    //    Field length
    pub(self) mod header {
        pub const ID: usize = 2;
        pub const TUPLE_NUM: usize = 8;
    }

    pub(self) mod slot {
        pub const ID: usize = 4;
        pub const ADDRESS: usize = 2;
        pub const SIZE: usize = 2;
    }

    const HEADER: usize = header::ID + header::TUPLE_NUM;
    const SLOT: usize = slot::ID + slot::ADDRESS + slot::SIZE;

    type Slot = (u32, u16, u16);

    pub fn serialize_page(p: &Page) -> Result<([u8; PAGE_SIZE], Option<Vec<Tuple>>), ProtobufError> {
        let (mut slots, mut tuples) = (Vec::with_capacity(p.tuples.len()), Vec::with_capacity(p.tuples.len()));
        {
            let mut tuple_address = PAGE_SIZE;
            let mut total_size = HEADER;
            for (i, t) in p.tuples.iter().enumerate() {
                let (tuple, slot) = {
                    let tuple = serialize_tuple(t.clone())?;
                    let tuple_len = tuple.len();
                    if PAGE_SIZE <= tuple.len() {
                        panic!("a tuple size overed PAGE_SIZE({:?})", PAGE_SIZE);
//                      ToDo: For BLOB tuple.
                    }
                    tuple_address -= tuple.len();
                    (tuple, encode_slot((t.id, tuple_address as u16, tuple_len as u16))?)
                };
                total_size += tuple.len() + slot.len();
                if PAGE_SIZE < total_size {
//                      When buffer is overflowing. making new page binary.
                    let page = aux_serialize_page(encode_page_id(p.id), slots, tuples)?;
                    return Ok((page, Some(p.tuples.split_at(i).1.to_vec())));
                };
                tuples.insert(0, tuple);
                slots.push(slot);
            }
        }
        let page: [u8; PAGE_SIZE] = aux_serialize_page(encode_page_id(p.id), slots, tuples)?;
        return Ok((page, None));
    }

    pub fn deserialize_page(mut bytes: [u8; PAGE_SIZE]) -> Result<Page, ProtobufError> {
        let (id, tuple_num) = {
            let bytes: &mut [u8] = &mut bytes;
            let mut cur: Cursor<&mut [u8]> = Cursor::new(bytes);
            let (mut id, mut tuple_num) = ([0u8; header::ID], [0u8; header::TUPLE_NUM]);
            cur.read_exact(&mut id)?;
            cur.read_exact(&mut tuple_num)?;
            (u16::from_le_bytes(id), u64::from_le_bytes(tuple_num) as usize)
        };
        let tuples: Vec<Tuple> = {
            let slots: Vec<Slot> = {
                let mut buf: Vec<u8> = Vec::with_capacity(tuple_num * SLOT);
                let mut cur = Cursor::new(buf);
                cur.write(&bytes[HEADER..HEADER + (tuple_num * SLOT)])?;
                decode_slots(cur.get_ref().clone(), tuple_num)?
            };
            let mut tuples: Vec<Tuple> = Vec::with_capacity(tuple_num);
            for &(_, address, size) in slots.iter() {
                let mut tuple = Vec::with_capacity(size as usize);
                let mut cur = Cursor::new(tuple);
                cur.write(&bytes[address as usize..])?;
                tuples.push(deserialize_tuple(cur.get_ref())?);
            }
            tuples
        };
        Ok(Page { id, tuples })
    }

    fn decode_slot(mut slot: [u8; SLOT]) -> Result<Slot, std::io::Error> {
        let (mut tuple_id, mut tuple_address, mut tuple_size) = ([0u8; slot::ID], [0u8; slot::ADDRESS], [0u8; slot::SIZE]);
        let mut cur = Cursor::new(&mut slot);
        cur.read_exact(&mut tuple_id)?;
        cur.read_exact(&mut tuple_address)?;
        cur.read_exact(&mut tuple_size)?;
        Ok((u32::from_le_bytes(tuple_id), u16::from_le_bytes(tuple_address), u16::from_le_bytes(tuple_size)))
    }

    fn decode_slots(slots: Vec<u8>, num: usize) -> Result<Vec<Slot>, std::io::Error> {
        let mut buffer: Vec<Slot> = Vec::with_capacity(num);
        let mut cur: Cursor<Vec<u8>> = Cursor::new(slots);
        for _ in 0..num {
            let mut slot = [0u8; SLOT];
            cur.read_exact(&mut slot)?;
            buffer.push(decode_slot(slot)?);
        }
        Ok(buffer)
    }

    fn encode_page_id(id: u16) -> [u8; header::ID] {
        id.to_le_bytes() as [u8; header::ID]
    }

    fn encode_slot(slot: Slot) -> Result<[u8; SLOT], std::io::Error> {
        let mut buf: &mut [u8] = &mut [0u8; SLOT];
        let mut cur = Cursor::new(buf);
        let mut tuple_id = slot.0.to_le_bytes() as [u8; slot::ID];
        let mut tuple_address = slot.1.to_le_bytes() as [u8; slot::ADDRESS];
        let mut tuple_size = slot.2.to_le_bytes() as [u8; slot::SIZE];
        cur.write(&mut tuple_id)?;
        cur.write(&mut tuple_address)?;
        cur.write(&mut tuple_size)?;
        let mut result = [0u8; SLOT];
        result.copy_from_slice(&cur.get_ref()[..SLOT]);
        Ok(result)
    }

    fn aux_serialize_page(id: [u8; header::ID], slots: Vec<[u8; SLOT]>, tuples: Vec<Vec<u8>>) -> Result<[u8; PAGE_SIZE], std::io::Error> {
        let mut buf: &mut [u8] = &mut [0u8; PAGE_SIZE];

        let mut cur = Cursor::new(buf);
        cur.write(&id)?;
        let tuple_num = tuples.len().to_le_bytes() as [u8; 8];
        cur.write(&tuple_num)?;
        let slots = slots.concat();
        cur.write(slots.as_slice())?;
        let tuples = tuples.concat();
        cur.set_position((PAGE_SIZE - &tuples.as_slice().len()) as u64);
        cur.write(&tuples.as_slice())?;
        let mut page = [0u8; PAGE_SIZE];
        cur.set_position(0);
        cur.read(&mut page)?;
        Ok(page)
    }

    #[cfg(test)]
    mod tests {
        use crate::storage::data::{Tuple, TupleData, TupleData_Type};
        use crate::storage::tuple::functions::{serialize_tuple, deserialize_tuple};
        use crate::storage::util;
        use std::time::SystemTime;
        use protobuf::{RepeatedField, ProtobufError};
        use crate::storage::page::Page;
        use crate::storage::page::function::{serialize_page, deserialize_page, encode_slot, decode_slots, decode_slot, HEADER, SLOT};
        use std::io::{Cursor, Write};

        #[test]
        fn page() {
            let (mut tuple, mut data) = (Tuple::new(), TupleData::new());
            data.set_field_type(TupleData_Type::STRING);
            data.set_string("Hello!".to_string());
            tuple.set_id(util::gen_hash(&SystemTime::now()) as u32);
            let mut tuple_data: RepeatedField<TupleData> = RepeatedField::new();
            tuple_data.push(data);
            tuple.set_data(tuple_data);
            let mut page = Page { id: util::gen_hash(&SystemTime::now()) as u16, tuples: vec![tuple] };

            let deserialized_page = deserialize_page(serialize_page(&page.clone()).unwrap().0).unwrap();

            assert_eq!(page, deserialized_page);
        }

        #[test]
        fn slot() {
            let mut slot = (1u32, 7968u16, 128u16);
            let mut bytes = encode_slot(slot).unwrap();
            assert_eq!(slot, decode_slot(bytes).unwrap());
        }

        #[test]
        fn slots() {
            let mut slots = vec![(1u32, 7968u16, 128u16)];
            let mut bytes = encode_slot(slots[0]).unwrap();
            assert_eq!(slots, decode_slots(bytes.to_vec(), slots.len()).unwrap());
        }
    }
}




