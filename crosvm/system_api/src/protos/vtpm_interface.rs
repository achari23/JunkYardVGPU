// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by protoc 3.21.12
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `vtpm_interface.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:vtpm.SendCommandRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SendCommandRequest {
    // message fields
    // @@protoc_insertion_point(field:vtpm.SendCommandRequest.command)
    pub command: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:vtpm.SendCommandRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SendCommandRequest {
    fn default() -> &'a SendCommandRequest {
        <SendCommandRequest as ::protobuf::Message>::default_instance()
    }
}

impl SendCommandRequest {
    pub fn new() -> SendCommandRequest {
        ::std::default::Default::default()
    }

    // optional bytes command = 1;

    pub fn command(&self) -> &[u8] {
        match self.command.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: ::std::vec::Vec<u8>) {
        self.command = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.command.is_none() {
            self.command = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.command.as_mut().unwrap()
    }

    // Take field
    pub fn take_command(&mut self) -> ::std::vec::Vec<u8> {
        self.command.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for SendCommandRequest {
    const NAME: &'static str = "SendCommandRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.command = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.command.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.command.as_ref() {
            os.write_bytes(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SendCommandRequest {
        SendCommandRequest::new()
    }

    fn clear(&mut self) {
        self.command = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SendCommandRequest {
        static instance: SendCommandRequest = SendCommandRequest {
            command: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:vtpm.SendCommandResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SendCommandResponse {
    // message fields
    // @@protoc_insertion_point(field:vtpm.SendCommandResponse.response)
    pub response: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:vtpm.SendCommandResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SendCommandResponse {
    fn default() -> &'a SendCommandResponse {
        <SendCommandResponse as ::protobuf::Message>::default_instance()
    }
}

impl SendCommandResponse {
    pub fn new() -> SendCommandResponse {
        ::std::default::Default::default()
    }

    // optional bytes response = 1;

    pub fn response(&self) -> &[u8] {
        match self.response.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ::std::vec::Vec<u8>) {
        self.response = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.response.is_none() {
            self.response = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> ::std::vec::Vec<u8> {
        self.response.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for SendCommandResponse {
    const NAME: &'static str = "SendCommandResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.response = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.response.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.response.as_ref() {
            os.write_bytes(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SendCommandResponse {
        SendCommandResponse::new()
    }

    fn clear(&mut self) {
        self.response = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SendCommandResponse {
        static instance: SendCommandResponse = SendCommandResponse {
            response: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}
