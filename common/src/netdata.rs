// This file is generated. Do not edit
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
pub struct SSpawnEntity {
    // message fields
    pub id: u64,
    pub field_type: ::std::string::String,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSpawnEntity {}

impl SSpawnEntity {
    pub fn new() -> SSpawnEntity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSpawnEntity {
        static mut instance: ::protobuf::lazy::Lazy<SSpawnEntity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSpawnEntity,
        };
        unsafe {
            instance.get(SSpawnEntity::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // float x = 3;

    pub fn clear_x(&mut self) {
        self.x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = v;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    fn get_x_for_reflect(&self) -> &f32 {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut f32 {
        &mut self.x
    }

    // float y = 4;

    pub fn clear_y(&mut self) {
        self.y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = v;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    fn get_y_for_reflect(&self) -> &f32 {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut f32 {
        &mut self.y
    }

    // float angle = 5;

    pub fn clear_angle(&mut self) {
        self.angle = 0.;
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: f32) {
        self.angle = v;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    fn get_angle_for_reflect(&self) -> &f32 {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut f32 {
        &mut self.angle
    }
}

impl ::protobuf::Message for SSpawnEntity {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.x = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.angle = tmp;
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
        };
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
        };
        if self.x != 0. {
            my_size += 5;
        };
        if self.y != 0. {
            my_size += 5;
        };
        if self.angle != 0. {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        };
        if !self.field_type.is_empty() {
            os.write_string(2, &self.field_type)?;
        };
        if self.x != 0. {
            os.write_float(3, self.x)?;
        };
        if self.y != 0. {
            os.write_float(4, self.y)?;
        };
        if self.angle != 0. {
            os.write_float(5, self.angle)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSpawnEntity {
    fn new() -> SSpawnEntity {
        SSpawnEntity::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSpawnEntity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    SSpawnEntity::get_id_for_reflect,
                    SSpawnEntity::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    SSpawnEntity::get_field_type_for_reflect,
                    SSpawnEntity::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    SSpawnEntity::get_x_for_reflect,
                    SSpawnEntity::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    SSpawnEntity::get_y_for_reflect,
                    SSpawnEntity::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "angle",
                    SSpawnEntity::get_angle_for_reflect,
                    SSpawnEntity::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSpawnEntity>(
                    "SSpawnEntity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSpawnEntity {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_field_type();
        self.clear_x();
        self.clear_y();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SSpawnEntity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SSpawnEntity {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SMoveEntity {
    // message fields
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SMoveEntity {}

impl SMoveEntity {
    pub fn new() -> SMoveEntity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SMoveEntity {
        static mut instance: ::protobuf::lazy::Lazy<SMoveEntity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SMoveEntity,
        };
        unsafe {
            instance.get(SMoveEntity::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // float x = 2;

    pub fn clear_x(&mut self) {
        self.x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = v;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    fn get_x_for_reflect(&self) -> &f32 {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut f32 {
        &mut self.x
    }

    // float y = 3;

    pub fn clear_y(&mut self) {
        self.y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = v;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    fn get_y_for_reflect(&self) -> &f32 {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut f32 {
        &mut self.y
    }

    // float angle = 5;

    pub fn clear_angle(&mut self) {
        self.angle = 0.;
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: f32) {
        self.angle = v;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    fn get_angle_for_reflect(&self) -> &f32 {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut f32 {
        &mut self.angle
    }
}

impl ::protobuf::Message for SMoveEntity {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.x = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.angle = tmp;
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
        };
        if self.x != 0. {
            my_size += 5;
        };
        if self.y != 0. {
            my_size += 5;
        };
        if self.angle != 0. {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        };
        if self.x != 0. {
            os.write_float(2, self.x)?;
        };
        if self.y != 0. {
            os.write_float(3, self.y)?;
        };
        if self.angle != 0. {
            os.write_float(5, self.angle)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SMoveEntity {
    fn new() -> SMoveEntity {
        SMoveEntity::new()
    }

    fn descriptor_static(_: ::std::option::Option<SMoveEntity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    SMoveEntity::get_id_for_reflect,
                    SMoveEntity::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    SMoveEntity::get_x_for_reflect,
                    SMoveEntity::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    SMoveEntity::get_y_for_reflect,
                    SMoveEntity::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "angle",
                    SMoveEntity::get_angle_for_reflect,
                    SMoveEntity::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SMoveEntity>(
                    "SMoveEntity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SMoveEntity {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_x();
        self.clear_y();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SMoveEntity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SMoveEntity {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SSplash {
    // message fields
    pub field_type: ::std::string::String,
    pub parent: u64,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub extra: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSplash {}

impl SSplash {
    pub fn new() -> SSplash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSplash {
        static mut instance: ::protobuf::lazy::Lazy<SSplash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSplash,
        };
        unsafe {
            instance.get(SSplash::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // uint64 parent = 10;

    pub fn clear_parent(&mut self) {
        self.parent = 0;
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: u64) {
        self.parent = v;
    }

    pub fn get_parent(&self) -> u64 {
        self.parent
    }

    fn get_parent_for_reflect(&self) -> &u64 {
        &self.parent
    }

    fn mut_parent_for_reflect(&mut self) -> &mut u64 {
        &mut self.parent
    }

    // float x = 2;

    pub fn clear_x(&mut self) {
        self.x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = v;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    fn get_x_for_reflect(&self) -> &f32 {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut f32 {
        &mut self.x
    }

    // float y = 3;

    pub fn clear_y(&mut self) {
        self.y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = v;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    fn get_y_for_reflect(&self) -> &f32 {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut f32 {
        &mut self.y
    }

    // float dx = 4;

    pub fn clear_dx(&mut self) {
        self.dx = 0.;
    }

    // Param is passed by value, moved
    pub fn set_dx(&mut self, v: f32) {
        self.dx = v;
    }

    pub fn get_dx(&self) -> f32 {
        self.dx
    }

    fn get_dx_for_reflect(&self) -> &f32 {
        &self.dx
    }

    fn mut_dx_for_reflect(&mut self) -> &mut f32 {
        &mut self.dx
    }

    // float dy = 5;

    pub fn clear_dy(&mut self) {
        self.dy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_dy(&mut self, v: f32) {
        self.dy = v;
    }

    pub fn get_dy(&self) -> f32 {
        self.dy
    }

    fn get_dy_for_reflect(&self) -> &f32 {
        &self.dy
    }

    fn mut_dy_for_reflect(&mut self) -> &mut f32 {
        &mut self.dy
    }

    // bytes extra = 11;

    pub fn clear_extra(&mut self) {
        self.extra.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra(&mut self, v: ::std::vec::Vec<u8>) {
        self.extra = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extra(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.extra
    }

    // Take field
    pub fn take_extra(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.extra, ::std::vec::Vec::new())
    }

    pub fn get_extra(&self) -> &[u8] {
        &self.extra
    }

    fn get_extra_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.extra
    }

    fn mut_extra_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.extra
    }
}

impl ::protobuf::Message for SSplash {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.parent = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.x = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.dx = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.dy = tmp;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.extra)?;
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
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        };
        if self.parent != 0 {
            my_size += ::protobuf::rt::value_size(10, self.parent, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.x != 0. {
            my_size += 5;
        };
        if self.y != 0. {
            my_size += 5;
        };
        if self.dx != 0. {
            my_size += 5;
        };
        if self.dy != 0. {
            my_size += 5;
        };
        if !self.extra.is_empty() {
            my_size += ::protobuf::rt::bytes_size(11, &self.extra);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        };
        if self.parent != 0 {
            os.write_uint64(10, self.parent)?;
        };
        if self.x != 0. {
            os.write_float(2, self.x)?;
        };
        if self.y != 0. {
            os.write_float(3, self.y)?;
        };
        if self.dx != 0. {
            os.write_float(4, self.dx)?;
        };
        if self.dy != 0. {
            os.write_float(5, self.dy)?;
        };
        if !self.extra.is_empty() {
            os.write_bytes(11, &self.extra)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSplash {
    fn new() -> SSplash {
        SSplash::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSplash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    SSplash::get_field_type_for_reflect,
                    SSplash::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parent",
                    SSplash::get_parent_for_reflect,
                    SSplash::mut_parent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    SSplash::get_x_for_reflect,
                    SSplash::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    SSplash::get_y_for_reflect,
                    SSplash::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "dx",
                    SSplash::get_dx_for_reflect,
                    SSplash::mut_dx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "dy",
                    SSplash::get_dy_for_reflect,
                    SSplash::mut_dy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "extra",
                    SSplash::get_extra_for_reflect,
                    SSplash::mut_extra_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSplash>(
                    "SSplash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSplash {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_parent();
        self.clear_x();
        self.clear_y();
        self.clear_dx();
        self.clear_dy();
        self.clear_extra();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SSplash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SSplash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FromServer {
    // message oneof groups
    pak: ::std::option::Option<FromServer_oneof_pak>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FromServer {}

#[derive(Clone,PartialEq)]
pub enum FromServer_oneof_pak {
    spawn_entity(SSpawnEntity),
    move_entity(SMoveEntity),
    splash(SSplash),
}

impl FromServer {
    pub fn new() -> FromServer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FromServer {
        static mut instance: ::protobuf::lazy::Lazy<FromServer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FromServer,
        };
        unsafe {
            instance.get(FromServer::new)
        }
    }

    // .SSpawnEntity spawn_entity = 5;

    pub fn clear_spawn_entity(&mut self) {
        self.pak = ::std::option::Option::None;
    }

    pub fn has_spawn_entity(&self) -> bool {
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_spawn_entity(&mut self, v: SSpawnEntity) {
        self.pak = ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(v))
    }

    // Mutable pointer to the field.
    pub fn mut_spawn_entity(&mut self) -> &mut SSpawnEntity {
        if let ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(_)) = self.pak {
        } else {
            self.pak = ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(SSpawnEntity::new()));
        }
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_spawn_entity(&mut self) -> SSpawnEntity {
        if self.has_spawn_entity() {
            match self.pak.take() {
                ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(v)) => v,
                _ => panic!(),
            }
        } else {
            SSpawnEntity::new()
        }
    }

    pub fn get_spawn_entity(&self) -> &SSpawnEntity {
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(ref v)) => v,
            _ => SSpawnEntity::default_instance(),
        }
    }

    // .SMoveEntity move_entity = 6;

    pub fn clear_move_entity(&mut self) {
        self.pak = ::std::option::Option::None;
    }

    pub fn has_move_entity(&self) -> bool {
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::move_entity(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_move_entity(&mut self, v: SMoveEntity) {
        self.pak = ::std::option::Option::Some(FromServer_oneof_pak::move_entity(v))
    }

    // Mutable pointer to the field.
    pub fn mut_move_entity(&mut self) -> &mut SMoveEntity {
        if let ::std::option::Option::Some(FromServer_oneof_pak::move_entity(_)) = self.pak {
        } else {
            self.pak = ::std::option::Option::Some(FromServer_oneof_pak::move_entity(SMoveEntity::new()));
        }
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::move_entity(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_move_entity(&mut self) -> SMoveEntity {
        if self.has_move_entity() {
            match self.pak.take() {
                ::std::option::Option::Some(FromServer_oneof_pak::move_entity(v)) => v,
                _ => panic!(),
            }
        } else {
            SMoveEntity::new()
        }
    }

    pub fn get_move_entity(&self) -> &SMoveEntity {
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::move_entity(ref v)) => v,
            _ => SMoveEntity::default_instance(),
        }
    }

    // .SSplash splash = 11;

    pub fn clear_splash(&mut self) {
        self.pak = ::std::option::Option::None;
    }

    pub fn has_splash(&self) -> bool {
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::splash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_splash(&mut self, v: SSplash) {
        self.pak = ::std::option::Option::Some(FromServer_oneof_pak::splash(v))
    }

    // Mutable pointer to the field.
    pub fn mut_splash(&mut self) -> &mut SSplash {
        if let ::std::option::Option::Some(FromServer_oneof_pak::splash(_)) = self.pak {
        } else {
            self.pak = ::std::option::Option::Some(FromServer_oneof_pak::splash(SSplash::new()));
        }
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::splash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_splash(&mut self) -> SSplash {
        if self.has_splash() {
            match self.pak.take() {
                ::std::option::Option::Some(FromServer_oneof_pak::splash(v)) => v,
                _ => panic!(),
            }
        } else {
            SSplash::new()
        }
    }

    pub fn get_splash(&self) -> &SSplash {
        match self.pak {
            ::std::option::Option::Some(FromServer_oneof_pak::splash(ref v)) => v,
            _ => SSplash::default_instance(),
        }
    }
}

impl ::protobuf::Message for FromServer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.pak = ::std::option::Option::Some(FromServer_oneof_pak::spawn_entity(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.pak = ::std::option::Option::Some(FromServer_oneof_pak::move_entity(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.pak = ::std::option::Option::Some(FromServer_oneof_pak::splash(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.pak {
            match v {
                &FromServer_oneof_pak::spawn_entity(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &FromServer_oneof_pak::move_entity(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &FromServer_oneof_pak::splash(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.pak {
            match v {
                &FromServer_oneof_pak::spawn_entity(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &FromServer_oneof_pak::move_entity(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &FromServer_oneof_pak::splash(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FromServer {
    fn new() -> FromServer {
        FromServer::new()
    }

    fn descriptor_static(_: ::std::option::Option<FromServer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SSpawnEntity>(
                    "spawn_entity",
                    FromServer::has_spawn_entity,
                    FromServer::get_spawn_entity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SMoveEntity>(
                    "move_entity",
                    FromServer::has_move_entity,
                    FromServer::get_move_entity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SSplash>(
                    "splash",
                    FromServer::has_splash,
                    FromServer::get_splash,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FromServer>(
                    "FromServer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FromServer {
    fn clear(&mut self) {
        self.clear_spawn_entity();
        self.clear_move_entity();
        self.clear_splash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FromServer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FromServer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPlaceStructure {
    // message fields
    pub id: ::std::string::String,
    pub x: i32,
    pub y: i32,
    pub rot: Rotation,
    pub extra: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPlaceStructure {}

impl CPlaceStructure {
    pub fn new() -> CPlaceStructure {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPlaceStructure {
        static mut instance: ::protobuf::lazy::Lazy<CPlaceStructure> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPlaceStructure,
        };
        unsafe {
            instance.get(CPlaceStructure::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // int32 x = 2;

    pub fn clear_x(&mut self) {
        self.x = 0;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = v;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    fn get_x_for_reflect(&self) -> &i32 {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut i32 {
        &mut self.x
    }

    // int32 y = 3;

    pub fn clear_y(&mut self) {
        self.y = 0;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = v;
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    fn get_y_for_reflect(&self) -> &i32 {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut i32 {
        &mut self.y
    }

    // .Rotation rot = 4;

    pub fn clear_rot(&mut self) {
        self.rot = Rotation::NORTH;
    }

    // Param is passed by value, moved
    pub fn set_rot(&mut self, v: Rotation) {
        self.rot = v;
    }

    pub fn get_rot(&self) -> Rotation {
        self.rot
    }

    fn get_rot_for_reflect(&self) -> &Rotation {
        &self.rot
    }

    fn mut_rot_for_reflect(&mut self) -> &mut Rotation {
        &mut self.rot
    }

    // bytes extra = 10;

    pub fn clear_extra(&mut self) {
        self.extra.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra(&mut self, v: ::std::vec::Vec<u8>) {
        self.extra = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extra(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.extra
    }

    // Take field
    pub fn take_extra(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.extra, ::std::vec::Vec::new())
    }

    pub fn get_extra(&self) -> &[u8] {
        &self.extra
    }

    fn get_extra_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.extra
    }

    fn mut_extra_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.extra
    }
}

impl ::protobuf::Message for CPlaceStructure {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.x = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.y = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.rot = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.extra)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        };
        if self.x != 0 {
            my_size += ::protobuf::rt::value_size(2, self.x, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.y != 0 {
            my_size += ::protobuf::rt::value_size(3, self.y, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.rot != Rotation::NORTH {
            my_size += ::protobuf::rt::enum_size(4, self.rot);
        };
        if !self.extra.is_empty() {
            my_size += ::protobuf::rt::bytes_size(10, &self.extra);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        };
        if self.x != 0 {
            os.write_int32(2, self.x)?;
        };
        if self.y != 0 {
            os.write_int32(3, self.y)?;
        };
        if self.rot != Rotation::NORTH {
            os.write_enum(4, self.rot.value())?;
        };
        if !self.extra.is_empty() {
            os.write_bytes(10, &self.extra)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CPlaceStructure {
    fn new() -> CPlaceStructure {
        CPlaceStructure::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPlaceStructure>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    CPlaceStructure::get_id_for_reflect,
                    CPlaceStructure::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    CPlaceStructure::get_x_for_reflect,
                    CPlaceStructure::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    CPlaceStructure::get_y_for_reflect,
                    CPlaceStructure::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Rotation>>(
                    "rot",
                    CPlaceStructure::get_rot_for_reflect,
                    CPlaceStructure::mut_rot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "extra",
                    CPlaceStructure::get_extra_for_reflect,
                    CPlaceStructure::mut_extra_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPlaceStructure>(
                    "CPlaceStructure",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPlaceStructure {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_x();
        self.clear_y();
        self.clear_rot();
        self.clear_extra();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPlaceStructure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPlaceStructure {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FromClient {
    // message oneof groups
    pak: ::std::option::Option<FromClient_oneof_pak>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FromClient {}

#[derive(Clone,PartialEq)]
pub enum FromClient_oneof_pak {
    place(CPlaceStructure),
}

impl FromClient {
    pub fn new() -> FromClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FromClient {
        static mut instance: ::protobuf::lazy::Lazy<FromClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FromClient,
        };
        unsafe {
            instance.get(FromClient::new)
        }
    }

    // .CPlaceStructure place = 10;

    pub fn clear_place(&mut self) {
        self.pak = ::std::option::Option::None;
    }

    pub fn has_place(&self) -> bool {
        match self.pak {
            ::std::option::Option::Some(FromClient_oneof_pak::place(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_place(&mut self, v: CPlaceStructure) {
        self.pak = ::std::option::Option::Some(FromClient_oneof_pak::place(v))
    }

    // Mutable pointer to the field.
    pub fn mut_place(&mut self) -> &mut CPlaceStructure {
        if let ::std::option::Option::Some(FromClient_oneof_pak::place(_)) = self.pak {
        } else {
            self.pak = ::std::option::Option::Some(FromClient_oneof_pak::place(CPlaceStructure::new()));
        }
        match self.pak {
            ::std::option::Option::Some(FromClient_oneof_pak::place(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_place(&mut self) -> CPlaceStructure {
        if self.has_place() {
            match self.pak.take() {
                ::std::option::Option::Some(FromClient_oneof_pak::place(v)) => v,
                _ => panic!(),
            }
        } else {
            CPlaceStructure::new()
        }
    }

    pub fn get_place(&self) -> &CPlaceStructure {
        match self.pak {
            ::std::option::Option::Some(FromClient_oneof_pak::place(ref v)) => v,
            _ => CPlaceStructure::default_instance(),
        }
    }
}

impl ::protobuf::Message for FromClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.pak = ::std::option::Option::Some(FromClient_oneof_pak::place(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.pak {
            match v {
                &FromClient_oneof_pak::place(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.pak {
            match v {
                &FromClient_oneof_pak::place(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FromClient {
    fn new() -> FromClient {
        FromClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<FromClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CPlaceStructure>(
                    "place",
                    FromClient::has_place,
                    FromClient::get_place,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FromClient>(
                    "FromClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FromClient {
    fn clear(&mut self) {
        self.clear_place();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FromClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FromClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Rotation {
    NORTH = 0,
    EAST = 1,
    SOUTH = 2,
    WEST = 3,
}

impl ::protobuf::ProtobufEnum for Rotation {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Rotation> {
        match value {
            0 => ::std::option::Option::Some(Rotation::NORTH),
            1 => ::std::option::Option::Some(Rotation::EAST),
            2 => ::std::option::Option::Some(Rotation::SOUTH),
            3 => ::std::option::Option::Some(Rotation::WEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Rotation] = &[
            Rotation::NORTH,
            Rotation::EAST,
            Rotation::SOUTH,
            Rotation::WEST,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Rotation>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Rotation", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Rotation {
}

impl ::std::default::Default for Rotation {
    fn default() -> Self {
        Rotation::NORTH
    }
}

impl ::protobuf::reflect::ProtobufValue for Rotation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x6e, 0x65, 0x74, 0x64, 0x61, 0x74, 0x61,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x64, 0x0a, 0x0c, 0x53, 0x53, 0x70, 0x61, 0x77, 0x6e,
    0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x01, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0x4f, 0x0a, 0x0b,
    0x53, 0x4d, 0x6f, 0x76, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x0e, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x0c, 0x0a, 0x01, 0x78,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0x87, 0x01,
    0x0a, 0x07, 0x53, 0x53, 0x70, 0x6c, 0x61, 0x73, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a,
    0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x70,
    0x61, 0x72, 0x65, 0x6e, 0x74, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01,
    0x79, 0x12, 0x0e, 0x0a, 0x02, 0x64, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x02, 0x64,
    0x78, 0x12, 0x0e, 0x0a, 0x02, 0x64, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x02, 0x64,
    0x79, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x9c, 0x01, 0x0a, 0x0a, 0x46, 0x72, 0x6f, 0x6d,
    0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x12, 0x32, 0x0a, 0x0c, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x53,
    0x53, 0x70, 0x61, 0x77, 0x6e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x48, 0x00, 0x52, 0x0b, 0x73,
    0x70, 0x61, 0x77, 0x6e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x2f, 0x0a, 0x0b, 0x6d, 0x6f,
    0x76, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x53, 0x4d, 0x6f, 0x76, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x48, 0x00, 0x52,
    0x0a, 0x6d, 0x6f, 0x76, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x22, 0x0a, 0x06, 0x73,
    0x70, 0x6c, 0x61, 0x73, 0x68, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x53, 0x53,
    0x70, 0x6c, 0x61, 0x73, 0x68, 0x48, 0x00, 0x52, 0x06, 0x73, 0x70, 0x6c, 0x61, 0x73, 0x68, 0x42,
    0x05, 0x0a, 0x03, 0x70, 0x61, 0x6b, 0x22, 0x70, 0x0a, 0x0f, 0x43, 0x50, 0x6c, 0x61, 0x63, 0x65,
    0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x01, 0x79, 0x12, 0x1b, 0x0a, 0x03, 0x72, 0x6f, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x09, 0x2e, 0x52, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x03, 0x72,
    0x6f, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x3d, 0x0a, 0x0a, 0x46, 0x72, 0x6f, 0x6d,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x28, 0x0a, 0x05, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x43, 0x50, 0x6c, 0x61, 0x63, 0x65, 0x53, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x48, 0x00, 0x52, 0x05, 0x70, 0x6c, 0x61, 0x63, 0x65,
    0x42, 0x05, 0x0a, 0x03, 0x70, 0x61, 0x6b, 0x2a, 0x34, 0x0a, 0x08, 0x52, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x09, 0x0a, 0x05, 0x4e, 0x4f, 0x52, 0x54, 0x48, 0x10, 0x00, 0x12, 0x08,
    0x0a, 0x04, 0x45, 0x41, 0x53, 0x54, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x53, 0x4f, 0x55, 0x54,
    0x48, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x57, 0x45, 0x53, 0x54, 0x10, 0x03, 0x4a, 0xb0, 0x10,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x36, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x07, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x03, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x03, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x08, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x05, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06,
    0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x0f, 0x10, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0a, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0a, 0x08,
    0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x0f, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x0b, 0x08, 0x0a, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0b, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b,
    0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0d, 0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x0d, 0x08, 0x0c, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0d,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0e, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0e, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x11, 0x00, 0x16, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x12, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x12, 0x0f, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12,
    0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x14, 0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x14, 0x08, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x14,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x0e, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x14, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x15, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x15, 0x08, 0x14, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x15, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x15, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x18, 0x00, 0x20, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x19, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x19, 0x08, 0x18, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x19, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x19, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19,
    0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x1b, 0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x1b, 0x08, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x0e, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1c, 0x08, 0x1b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x1c, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x1c, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1d, 0x08,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1c, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1d, 0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1d, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1e, 0x08, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04,
    0x12, 0x04, 0x1e, 0x08, 0x1d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x1e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1e,
    0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1e, 0x13, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1f, 0x08, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x1f, 0x08, 0x1e, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x1f, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x1f, 0x16, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x22, 0x00,
    0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x22, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x23, 0x08, 0x27, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x23, 0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x24, 0x10, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x24, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x24, 0x1d, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x2c,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x25, 0x10, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x25, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02,
    0x12, 0x03, 0x26, 0x10, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x26, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x18,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x21, 0x23, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2a, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x2b, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2b,
    0x08, 0x2a, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x0f, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x14, 0x15, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x2c, 0x08, 0x2b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2c, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2c, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2d, 0x08, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2d, 0x08, 0x2c, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x03, 0x12, 0x03, 0x2e, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x2e, 0x08, 0x2d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x11,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2f, 0x08, 0x2e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x2f, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x2f, 0x16, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x32, 0x00, 0x36,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x32, 0x08, 0x12, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x05, 0x08, 0x00, 0x12, 0x04, 0x33, 0x08, 0x35, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x08, 0x00, 0x01, 0x12, 0x03, 0x33, 0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x34, 0x10, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x34, 0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34,
    0x20, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x28, 0x2a,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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
