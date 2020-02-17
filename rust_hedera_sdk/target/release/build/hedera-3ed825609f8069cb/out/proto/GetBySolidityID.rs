// This file is generated by rust-protobuf 2.3.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct GetBySolidityIDQuery {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::QueryHeader::QueryHeader>,
    pub solidityID: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl GetBySolidityIDQuery {
    pub fn new() -> GetBySolidityIDQuery {
        ::std::default::Default::default()
    }

    // .proto.QueryHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::QueryHeader::QueryHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::QueryHeader::QueryHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::QueryHeader::QueryHeader {
        self.header.take().unwrap_or_else(|| super::QueryHeader::QueryHeader::new())
    }

    pub fn get_header(&self) -> &super::QueryHeader::QueryHeader {
        self.header.as_ref().unwrap_or_else(|| super::QueryHeader::QueryHeader::default_instance())
    }

    // string solidityID = 2;

    pub fn clear_solidityID(&mut self) {
        self.solidityID.clear();
    }

    // Param is passed by value, moved
    pub fn set_solidityID(&mut self, v: ::std::string::String) {
        self.solidityID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_solidityID(&mut self) -> &mut ::std::string::String {
        &mut self.solidityID
    }

    // Take field
    pub fn take_solidityID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.solidityID, ::std::string::String::new())
    }

    pub fn get_solidityID(&self) -> &str {
        &self.solidityID
    }
}

impl ::protobuf::Message for GetBySolidityIDQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.solidityID)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.solidityID.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.solidityID);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.solidityID.is_empty() {
            os.write_string(2, &self.solidityID)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetBySolidityIDQuery {
        GetBySolidityIDQuery::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::QueryHeader::QueryHeader>>(
                    "header",
                    |m: &GetBySolidityIDQuery| { &m.header },
                    |m: &mut GetBySolidityIDQuery| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "solidityID",
                    |m: &GetBySolidityIDQuery| { &m.solidityID },
                    |m: &mut GetBySolidityIDQuery| { &mut m.solidityID },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBySolidityIDQuery>(
                    "GetBySolidityIDQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GetBySolidityIDQuery {
        static mut instance: ::protobuf::lazy::Lazy<GetBySolidityIDQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBySolidityIDQuery,
        };
        unsafe {
            instance.get(GetBySolidityIDQuery::new)
        }
    }
}

impl ::protobuf::Clear for GetBySolidityIDQuery {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_solidityID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBySolidityIDQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBySolidityIDQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBySolidityIDResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::ResponseHeader::ResponseHeader>,
    pub accountID: ::protobuf::SingularPtrField<super::BasicTypes::AccountID>,
    pub fileID: ::protobuf::SingularPtrField<super::BasicTypes::FileID>,
    pub contractID: ::protobuf::SingularPtrField<super::BasicTypes::ContractID>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl GetBySolidityIDResponse {
    pub fn new() -> GetBySolidityIDResponse {
        ::std::default::Default::default()
    }

    // .proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::ResponseHeader::ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::ResponseHeader::ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::ResponseHeader::ResponseHeader {
        self.header.take().unwrap_or_else(|| super::ResponseHeader::ResponseHeader::new())
    }

    pub fn get_header(&self) -> &super::ResponseHeader::ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| super::ResponseHeader::ResponseHeader::default_instance())
    }

    // .proto.AccountID accountID = 2;

    pub fn clear_accountID(&mut self) {
        self.accountID.clear();
    }

    pub fn has_accountID(&self) -> bool {
        self.accountID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accountID(&mut self, v: super::BasicTypes::AccountID) {
        self.accountID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accountID(&mut self) -> &mut super::BasicTypes::AccountID {
        if self.accountID.is_none() {
            self.accountID.set_default();
        }
        self.accountID.as_mut().unwrap()
    }

    // Take field
    pub fn take_accountID(&mut self) -> super::BasicTypes::AccountID {
        self.accountID.take().unwrap_or_else(|| super::BasicTypes::AccountID::new())
    }

    pub fn get_accountID(&self) -> &super::BasicTypes::AccountID {
        self.accountID.as_ref().unwrap_or_else(|| super::BasicTypes::AccountID::default_instance())
    }

    // .proto.FileID fileID = 3;

    pub fn clear_fileID(&mut self) {
        self.fileID.clear();
    }

    pub fn has_fileID(&self) -> bool {
        self.fileID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileID(&mut self, v: super::BasicTypes::FileID) {
        self.fileID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileID(&mut self) -> &mut super::BasicTypes::FileID {
        if self.fileID.is_none() {
            self.fileID.set_default();
        }
        self.fileID.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileID(&mut self) -> super::BasicTypes::FileID {
        self.fileID.take().unwrap_or_else(|| super::BasicTypes::FileID::new())
    }

    pub fn get_fileID(&self) -> &super::BasicTypes::FileID {
        self.fileID.as_ref().unwrap_or_else(|| super::BasicTypes::FileID::default_instance())
    }

    // .proto.ContractID contractID = 4;

    pub fn clear_contractID(&mut self) {
        self.contractID.clear();
    }

    pub fn has_contractID(&self) -> bool {
        self.contractID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contractID(&mut self, v: super::BasicTypes::ContractID) {
        self.contractID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contractID(&mut self) -> &mut super::BasicTypes::ContractID {
        if self.contractID.is_none() {
            self.contractID.set_default();
        }
        self.contractID.as_mut().unwrap()
    }

    // Take field
    pub fn take_contractID(&mut self) -> super::BasicTypes::ContractID {
        self.contractID.take().unwrap_or_else(|| super::BasicTypes::ContractID::new())
    }

    pub fn get_contractID(&self) -> &super::BasicTypes::ContractID {
        self.contractID.as_ref().unwrap_or_else(|| super::BasicTypes::ContractID::default_instance())
    }
}

impl ::protobuf::Message for GetBySolidityIDResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.accountID {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileID {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.contractID {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.accountID)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileID)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contractID)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.accountID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fileID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.contractID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.accountID.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fileID.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.contractID.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetBySolidityIDResponse {
        GetBySolidityIDResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::ResponseHeader::ResponseHeader>>(
                    "header",
                    |m: &GetBySolidityIDResponse| { &m.header },
                    |m: &mut GetBySolidityIDResponse| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::AccountID>>(
                    "accountID",
                    |m: &GetBySolidityIDResponse| { &m.accountID },
                    |m: &mut GetBySolidityIDResponse| { &mut m.accountID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::FileID>>(
                    "fileID",
                    |m: &GetBySolidityIDResponse| { &m.fileID },
                    |m: &mut GetBySolidityIDResponse| { &mut m.fileID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::ContractID>>(
                    "contractID",
                    |m: &GetBySolidityIDResponse| { &m.contractID },
                    |m: &mut GetBySolidityIDResponse| { &mut m.contractID },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBySolidityIDResponse>(
                    "GetBySolidityIDResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GetBySolidityIDResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetBySolidityIDResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBySolidityIDResponse,
        };
        unsafe {
            instance.get(GetBySolidityIDResponse::new)
        }
    }
}

impl ::protobuf::Clear for GetBySolidityIDResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_accountID();
        self.clear_fileID();
        self.clear_contractID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBySolidityIDResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBySolidityIDResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15GetBySolidityID.proto\x12\x05proto\x1a\x10BasicTypes.proto\x1a\x11\
    QueryHeader.proto\x1a\x14ResponseHeader.proto\"b\n\x14GetBySolidityIDQue\
    ry\x12*\n\x06header\x18\x01\x20\x01(\x0b2\x12.proto.QueryHeaderR\x06head\
    er\x12\x1e\n\nsolidityID\x18\x02\x20\x01(\tR\nsolidityID\"\xd2\x01\n\x17\
    GetBySolidityIDResponse\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15.proto\
    .ResponseHeaderR\x06header\x12.\n\taccountID\x18\x02\x20\x01(\x0b2\x10.p\
    roto.AccountIDR\taccountID\x12%\n\x06fileID\x18\x03\x20\x01(\x0b2\r.prot\
    o.FileIDR\x06fileID\x121\n\ncontractID\x18\x04\x20\x01(\x0b2\x11.proto.C\
    ontractIDR\ncontractIDB&\n\"com.hederahashgraph.api.proto.javaP\x01b\x06\
    proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

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
