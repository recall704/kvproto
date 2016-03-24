// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Job {
    // message fields
    job_id: ::std::option::Option<u64>,
    status: ::std::option::Option<JobStatus>,
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    request: ::protobuf::SingularPtrField<super::raft_cmdpb::RaftCmdRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Job {}

impl Job {
    pub fn new() -> Job {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Job {
        static mut instance: ::protobuf::lazy::Lazy<Job> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Job,
        };
        unsafe {
            instance.get(|| {
                Job {
                    job_id: ::std::option::Option::None,
                    status: ::std::option::Option::None,
                    region: ::protobuf::SingularPtrField::none(),
                    request: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 job_id = 1;

    pub fn clear_job_id(&mut self) {
        self.job_id = ::std::option::Option::None;
    }

    pub fn has_job_id(&self) -> bool {
        self.job_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_job_id(&mut self, v: u64) {
        self.job_id = ::std::option::Option::Some(v);
    }

    pub fn get_job_id<'a>(&self) -> u64 {
        self.job_id.unwrap_or(0)
    }

    // optional .pd_jobpd.JobStatus status = 2;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: JobStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> JobStatus {
        self.status.unwrap_or(JobStatus::Pending)
    }

    // optional .metapb.Region region = 3;

    pub fn clear_region(&mut self) {
        self.region.clear();
    }

    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region<'a>(&'a mut self) -> &'a mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region<'a>(&'a self) -> &'a super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    // optional .raft_cmdpb.RaftCmdRequest request = 4;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: super::raft_cmdpb::RaftCmdRequest) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request<'a>(&'a mut self) -> &'a mut super::raft_cmdpb::RaftCmdRequest {
        if self.request.is_none() {
            self.request.set_default();
        };
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> super::raft_cmdpb::RaftCmdRequest {
        self.request.take().unwrap_or_else(|| super::raft_cmdpb::RaftCmdRequest::new())
    }

    pub fn get_request<'a>(&'a self) -> &'a super::raft_cmdpb::RaftCmdRequest {
        self.request.as_ref().unwrap_or_else(|| super::raft_cmdpb::RaftCmdRequest::default_instance())
    }
}

impl ::protobuf::Message for Job {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.job_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.job_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.region.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.job_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.status {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.region.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.request.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Job>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Job {
    fn new() -> Job {
        Job::new()
    }

    fn descriptor_static(_: ::std::option::Option<Job>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "job_id",
                    Job::has_job_id,
                    Job::get_job_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    Job::has_status,
                    Job::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region",
                    Job::has_region,
                    Job::get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "request",
                    Job::has_request,
                    Job::get_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Job>(
                    "Job",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Job {
    fn clear(&mut self) {
        self.clear_job_id();
        self.clear_status();
        self.clear_region();
        self.clear_request();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Job {
    fn eq(&self, other: &Job) -> bool {
        self.job_id == other.job_id &&
        self.status == other.status &&
        self.region == other.region &&
        self.request == other.request &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Job {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum JobStatus {
    Pending = 0,
    Running = 1,
    Finished = 2,
    Canceled = 3,
}

impl ::protobuf::ProtobufEnum for JobStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JobStatus> {
        match value {
            0 => ::std::option::Option::Some(JobStatus::Pending),
            1 => ::std::option::Option::Some(JobStatus::Running),
            2 => ::std::option::Option::Some(JobStatus::Finished),
            3 => ::std::option::Option::Some(JobStatus::Canceled),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [JobStatus] = &[
            JobStatus::Pending,
            JobStatus::Running,
            JobStatus::Finished,
            JobStatus::Canceled,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<JobStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("JobStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for JobStatus {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x70, 0x64, 0x5f, 0x6a, 0x6f, 0x62, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x08, 0x70, 0x64, 0x5f, 0x6a, 0x6f, 0x62, 0x70, 0x64, 0x1a, 0x10, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x6d, 0x65,
    0x74, 0x61, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x87, 0x01, 0x0a, 0x03, 0x4a,
    0x6f, 0x62, 0x12, 0x0e, 0x0a, 0x06, 0x6a, 0x6f, 0x62, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x70, 0x64, 0x5f, 0x6a, 0x6f, 0x62, 0x70, 0x64, 0x2e, 0x4a, 0x6f,
    0x62, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x1e, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f,
    0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6d, 0x64, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x2a, 0x41, 0x0a, 0x09, 0x4a, 0x6f, 0x62, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x10, 0x00, 0x12, 0x0b,
    0x0a, 0x07, 0x52, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x46,
    0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x61, 0x6e,
    0x63, 0x65, 0x6c, 0x65, 0x64, 0x10, 0x03, 0x4a, 0xc7, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x10, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04,
    0x07, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x06, 0x00, 0x10, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x06, 0x05, 0x0e, 0x0a, 0x58, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x10, 0x1a, 0x4b, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x70,
    0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x6a, 0x6f,
    0x62, 0x20, 0x71, 0x75, 0x65, 0x75, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x69, 0x73, 0x20, 0x70, 0x65, 0x6e, 0x64, 0x69,
    0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x08, 0x0e, 0x0f,
    0x0a, 0xd5, 0x01, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x10, 0x1a, 0xc7,
    0x01, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x20, 0x67, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20,
    0x6a, 0x6f, 0x62, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x68,
    0x61, 0x6e, 0x64, 0x6c, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x75, 0x73, 0x74,
    0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x27, 0x73, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x69, 0x73, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e,
    0x67, 0x2c, 0x20, 0x77, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6a, 0x6f, 0x62, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x66,
    0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0c, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x0c, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x04,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x0f, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x0f, 0x0f, 0x10, 0x0a, 0xa8, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14,
    0x00, 0x19, 0x01, 0x1a, 0x9b, 0x01, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20,
    0x63, 0x6f, 0x6e, 0x66, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2c, 0x20, 0x73, 0x70, 0x6c,
    0x69, 0x74, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x77, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x64, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69,
    0x6e, 0x74, 0x6f, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x71, 0x75, 0x65, 0x75, 0x65, 0x2c, 0x20, 0x0a,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x74, 0x68, 0x72, 0x65, 0x61, 0x64, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x67, 0x65, 0x74, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6a, 0x6f, 0x62, 0x2c, 0x20, 0x68,
    0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x69, 0x74, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x6e, 0x20, 0x70, 0x6f, 0x70, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x15, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x15, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15,
    0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x16, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x16, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x17, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x17,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x17, 0x0d, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x1b, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x18, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x18, 0x0d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x18, 0x27, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x18, 0x32,
    0x33,
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
