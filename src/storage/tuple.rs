pub mod functions {
    use crate::storage::data::Tuple;
    use crate::storage::util;
    use protobuf::ProtobufError;
    use protobuf::Message;

    pub fn serialize_tuple(tuple: Tuple) -> Result<Vec<u8>, ProtobufError> {
//        compress(encode(tuple));
        let encoded = tuple.write_to_bytes()?;
        let compressed = util::compress(&encoded);
        Ok(compressed)
    }

    pub fn deserialize_tuple(bytes: &[u8]) -> Result<Tuple, ProtobufError> {
//        decode(uncompress(bytes));
        let mut tuple = Tuple::new();
        let uncompressed = util::uncompress(bytes);
        tuple.merge_from_bytes(&uncompressed)?;

        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::data::{Tuple, TupleData, TupleData_Type};
    use crate::storage::tuple::functions::{serialize_tuple, deserialize_tuple};
    use crate::storage::util;
    use std::time::SystemTime;
    use protobuf::{RepeatedField, ProtobufError};

    #[test]
    fn Eq_Origin_data_And_Deserialized_data() {
        let (mut tuple, mut data) = (Tuple::new(), TupleData::new());
        data.set_field_type(TupleData_Type::STRING);
        data.set_string("Hello!".to_string());
        tuple.set_id(util::gen_hash(&SystemTime::now()) as u32);
        let mut tuple_data = RepeatedField::new();
        tuple_data.push(data);
        tuple.set_data(tuple_data);
        let deserialized_tuple = deserialize_tuple(&serialize_tuple(tuple.clone()).unwrap().as_slice()).unwrap();
        assert_eq!(tuple, deserialized_tuple);
    }
}
