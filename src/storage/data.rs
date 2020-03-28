// This file is generated by rust-protobuf 2.12.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/storage/data.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_12_0;

#[derive(PartialEq,Clone,Default)]
pub struct TupleData {
    // message fields
    pub field_type: TupleData_Type,
    pub number: i32,
    pub string: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TupleData {
    fn default() -> &'a TupleData {
        <TupleData as ::protobuf::Message>::default_instance()
    }
}

impl TupleData {
    pub fn new() -> TupleData {
        ::std::default::Default::default()
    }

    // .TupleData.Type type = 3;


    pub fn get_field_type(&self) -> TupleData_Type {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = TupleData_Type::INT;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: TupleData_Type) {
        self.field_type = v;
    }

    // int32 number = 4;


    pub fn get_number(&self) -> i32 {
        self.number
    }
    pub fn clear_number(&mut self) {
        self.number = 0;
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = v;
    }

    // string string = 5;


    pub fn get_string(&self) -> &str {
        &self.string
    }
    pub fn clear_string(&mut self) {
        self.string.clear();
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::std::string::String) {
        self.string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string(&mut self) -> &mut ::std::string::String {
        &mut self.string
    }

    // Take field
    pub fn take_string(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.string, ::std::string::String::new())
    }
}

impl ::protobuf::Message for TupleData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 3, &mut self.unknown_fields)?
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.number = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.string)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != TupleData_Type::INT {
            my_size += ::protobuf::rt::enum_size(3, self.field_type);
        }
        if self.number != 0 {
            my_size += ::protobuf::rt::value_size(4, self.number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.string.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.string);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != TupleData_Type::INT {
            os.write_enum(3, self.field_type.value())?;
        }
        if self.number != 0 {
            os.write_int32(4, self.number)?;
        }
        if !self.string.is_empty() {
            os.write_string(5, &self.string)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TupleData {
        TupleData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TupleData_Type>>(
                    "type",
                    |m: &TupleData| { &m.field_type },
                    |m: &mut TupleData| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "number",
                    |m: &TupleData| { &m.number },
                    |m: &mut TupleData| { &mut m.number },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string",
                    |m: &TupleData| { &m.string },
                    |m: &mut TupleData| { &mut m.string },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<TupleData>(
                    "TupleData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TupleData {
        static mut instance: ::protobuf::lazy::Lazy<TupleData> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(TupleData::new)
        }
    }
}

impl ::protobuf::Clear for TupleData {
    fn clear(&mut self) {
        self.field_type = TupleData_Type::INT;
        self.number = 0;
        self.string.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TupleData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TupleData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TupleData_Type {
    INT = 0,
    STRING = 1,
}

impl ::protobuf::ProtobufEnum for TupleData_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TupleData_Type> {
        match value {
            0 => ::std::option::Option::Some(TupleData_Type::INT),
            1 => ::std::option::Option::Some(TupleData_Type::STRING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TupleData_Type] = &[
            TupleData_Type::INT,
            TupleData_Type::STRING,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new_pb_name::<TupleData_Type>("TupleData.Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TupleData_Type {
}

impl ::std::default::Default for TupleData_Type {
    fn default() -> Self {
        TupleData_Type::INT
    }
}

impl ::protobuf::reflect::ProtobufValue for TupleData_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tuple {
    // message fields
    pub id: u32,
    pub minTxId: u32,
    pub maxTxId: u32,
    pub data: ::protobuf::RepeatedField<TupleData>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Tuple {
    fn default() -> &'a Tuple {
        <Tuple as ::protobuf::Message>::default_instance()
    }
}

impl Tuple {
    pub fn new() -> Tuple {
        ::std::default::Default::default()
    }

    // uint32 id = 1;


    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    // uint32 minTxId = 2;


    pub fn get_minTxId(&self) -> u32 {
        self.minTxId
    }
    pub fn clear_minTxId(&mut self) {
        self.minTxId = 0;
    }

    // Param is passed by value, moved
    pub fn set_minTxId(&mut self, v: u32) {
        self.minTxId = v;
    }

    // uint32 maxTxId = 3;


    pub fn get_maxTxId(&self) -> u32 {
        self.maxTxId
    }
    pub fn clear_maxTxId(&mut self) {
        self.maxTxId = 0;
    }

    // Param is passed by value, moved
    pub fn set_maxTxId(&mut self, v: u32) {
        self.maxTxId = v;
    }

    // repeated .TupleData data = 4;


    pub fn get_data(&self) -> &[TupleData] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::protobuf::RepeatedField<TupleData>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::protobuf::RepeatedField<TupleData> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::protobuf::RepeatedField<TupleData> {
        ::std::mem::replace(&mut self.data, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Tuple {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.minTxId = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maxTxId = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.minTxId != 0 {
            my_size += ::protobuf::rt::value_size(2, self.minTxId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.maxTxId != 0 {
            my_size += ::protobuf::rt::value_size(3, self.maxTxId, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.minTxId != 0 {
            os.write_uint32(2, self.minTxId)?;
        }
        if self.maxTxId != 0 {
            os.write_uint32(3, self.maxTxId)?;
        }
        for v in &self.data {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Tuple {
        Tuple::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    |m: &Tuple| { &m.id },
                    |m: &mut Tuple| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "minTxId",
                    |m: &Tuple| { &m.minTxId },
                    |m: &mut Tuple| { &mut m.minTxId },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maxTxId",
                    |m: &Tuple| { &m.maxTxId },
                    |m: &mut Tuple| { &mut m.maxTxId },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TupleData>>(
                    "data",
                    |m: &Tuple| { &m.data },
                    |m: &mut Tuple| { &mut m.data },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Tuple>(
                    "Tuple",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Tuple {
        static mut instance: ::protobuf::lazy::Lazy<Tuple> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Tuple::new)
        }
    }
}

impl ::protobuf::Clear for Tuple {
    fn clear(&mut self) {
        self.id = 0;
        self.minTxId = 0;
        self.maxTxId = 0;
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tuple {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tuple {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/storage/data.proto\"}\n\tTupleData\x12#\n\x04type\x18\x03\x20\
    \x01(\x0e2\x0f.TupleData.TypeR\x04type\x12\x16\n\x06number\x18\x04\x20\
    \x01(\x05R\x06number\x12\x16\n\x06string\x18\x05\x20\x01(\tR\x06string\"\
    \x1b\n\x04Type\x12\x07\n\x03INT\x10\0\x12\n\n\x06STRING\x10\x01\"k\n\x05\
    Tuple\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x12\x18\n\x07minTxId\x18\
    \x02\x20\x01(\rR\x07minTxId\x12\x18\n\x07maxTxId\x18\x03\x20\x01(\rR\x07\
    maxTxId\x12\x1e\n\x04data\x18\x04\x20\x03(\x0b2\n.TupleDataR\x04datab\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}