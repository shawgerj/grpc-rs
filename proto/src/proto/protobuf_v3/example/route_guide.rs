// This file is generated by rust-protobuf 3.2.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `grpc/example/route_guide.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobufv3::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:routeguide.Point)
pub struct Point {
    // message fields
    // @@protoc_insertion_point(field:routeguide.Point.latitude)
    pub latitude: i32,
    // @@protoc_insertion_point(field:routeguide.Point.longitude)
    pub longitude: i32,
    // special fields
    // @@protoc_insertion_point(special_field:routeguide.Point.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Point {
    fn default() -> &'a Point {
        <Point as ::protobufv3::Message>::default_instance()
    }
}

impl Point {
    pub fn new() -> Point {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "latitude",
            |m: &Point| { &m.latitude },
            |m: &mut Point| { &mut m.latitude },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "longitude",
            |m: &Point| { &m.longitude },
            |m: &mut Point| { &mut m.longitude },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<Point>(
            "Point",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for Point {
    const NAME: &'static str = "Point";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.latitude = is.read_int32()?;
                },
                16 => {
                    self.longitude = is.read_int32()?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.latitude != 0 {
            my_size += ::protobufv3::rt::int32_size(1, self.latitude);
        }
        if self.longitude != 0 {
            my_size += ::protobufv3::rt::int32_size(2, self.longitude);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.latitude != 0 {
            os.write_int32(1, self.latitude)?;
        }
        if self.longitude != 0 {
            os.write_int32(2, self.longitude)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Point {
        Point::new()
    }

    fn clear(&mut self) {
        self.latitude = 0;
        self.longitude = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Point {
        static instance: Point = Point {
            latitude: 0,
            longitude: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for Point {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Point").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for Point {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:routeguide.Rectangle)
pub struct Rectangle {
    // message fields
    // @@protoc_insertion_point(field:routeguide.Rectangle.lo)
    pub lo: ::protobufv3::MessageField<Point>,
    // @@protoc_insertion_point(field:routeguide.Rectangle.hi)
    pub hi: ::protobufv3::MessageField<Point>,
    // special fields
    // @@protoc_insertion_point(special_field:routeguide.Rectangle.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Rectangle {
    fn default() -> &'a Rectangle {
        <Rectangle as ::protobufv3::Message>::default_instance()
    }
}

impl Rectangle {
    pub fn new() -> Rectangle {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_message_field_accessor::<_, Point>(
            "lo",
            |m: &Rectangle| { &m.lo },
            |m: &mut Rectangle| { &mut m.lo },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_message_field_accessor::<_, Point>(
            "hi",
            |m: &Rectangle| { &m.hi },
            |m: &mut Rectangle| { &mut m.hi },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<Rectangle>(
            "Rectangle",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for Rectangle {
    const NAME: &'static str = "Rectangle";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobufv3::rt::read_singular_message_into_field(is, &mut self.lo)?;
                },
                18 => {
                    ::protobufv3::rt::read_singular_message_into_field(is, &mut self.hi)?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.lo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.hi.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if let Some(v) = self.lo.as_ref() {
            ::protobufv3::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.hi.as_ref() {
            ::protobufv3::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Rectangle {
        Rectangle::new()
    }

    fn clear(&mut self) {
        self.lo.clear();
        self.hi.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Rectangle {
        static instance: Rectangle = Rectangle {
            lo: ::protobufv3::MessageField::none(),
            hi: ::protobufv3::MessageField::none(),
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for Rectangle {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Rectangle").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for Rectangle {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:routeguide.Feature)
pub struct Feature {
    // message fields
    // @@protoc_insertion_point(field:routeguide.Feature.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:routeguide.Feature.location)
    pub location: ::protobufv3::MessageField<Point>,
    // special fields
    // @@protoc_insertion_point(special_field:routeguide.Feature.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Feature {
    fn default() -> &'a Feature {
        <Feature as ::protobufv3::Message>::default_instance()
    }
}

impl Feature {
    pub fn new() -> Feature {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Feature| { &m.name },
            |m: &mut Feature| { &mut m.name },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_message_field_accessor::<_, Point>(
            "location",
            |m: &Feature| { &m.location },
            |m: &mut Feature| { &mut m.location },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<Feature>(
            "Feature",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for Feature {
    const NAME: &'static str = "Feature";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    ::protobufv3::rt::read_singular_message_into_field(is, &mut self.location)?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobufv3::rt::string_size(1, &self.name);
        }
        if let Some(v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(v) = self.location.as_ref() {
            ::protobufv3::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Feature {
        Feature::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.location.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Feature {
        static instance: Feature = Feature {
            name: ::std::string::String::new(),
            location: ::protobufv3::MessageField::none(),
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for Feature {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Feature").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Feature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for Feature {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:routeguide.RouteNote)
pub struct RouteNote {
    // message fields
    // @@protoc_insertion_point(field:routeguide.RouteNote.location)
    pub location: ::protobufv3::MessageField<Point>,
    // @@protoc_insertion_point(field:routeguide.RouteNote.message)
    pub message: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:routeguide.RouteNote.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RouteNote {
    fn default() -> &'a RouteNote {
        <RouteNote as ::protobufv3::Message>::default_instance()
    }
}

impl RouteNote {
    pub fn new() -> RouteNote {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_message_field_accessor::<_, Point>(
            "location",
            |m: &RouteNote| { &m.location },
            |m: &mut RouteNote| { &mut m.location },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &RouteNote| { &m.message },
            |m: &mut RouteNote| { &mut m.message },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<RouteNote>(
            "RouteNote",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for RouteNote {
    const NAME: &'static str = "RouteNote";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobufv3::rt::read_singular_message_into_field(is, &mut self.location)?;
                },
                18 => {
                    self.message = is.read_string()?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.message.is_empty() {
            my_size += ::protobufv3::rt::string_size(2, &self.message);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if let Some(v) = self.location.as_ref() {
            ::protobufv3::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RouteNote {
        RouteNote::new()
    }

    fn clear(&mut self) {
        self.location.clear();
        self.message.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RouteNote {
        static instance: RouteNote = RouteNote {
            location: ::protobufv3::MessageField::none(),
            message: ::std::string::String::new(),
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for RouteNote {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RouteNote").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RouteNote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for RouteNote {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:routeguide.RouteSummary)
pub struct RouteSummary {
    // message fields
    // @@protoc_insertion_point(field:routeguide.RouteSummary.point_count)
    pub point_count: i32,
    // @@protoc_insertion_point(field:routeguide.RouteSummary.feature_count)
    pub feature_count: i32,
    // @@protoc_insertion_point(field:routeguide.RouteSummary.distance)
    pub distance: i32,
    // @@protoc_insertion_point(field:routeguide.RouteSummary.elapsed_time)
    pub elapsed_time: i32,
    // special fields
    // @@protoc_insertion_point(special_field:routeguide.RouteSummary.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RouteSummary {
    fn default() -> &'a RouteSummary {
        <RouteSummary as ::protobufv3::Message>::default_instance()
    }
}

impl RouteSummary {
    pub fn new() -> RouteSummary {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "point_count",
            |m: &RouteSummary| { &m.point_count },
            |m: &mut RouteSummary| { &mut m.point_count },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "feature_count",
            |m: &RouteSummary| { &m.feature_count },
            |m: &mut RouteSummary| { &mut m.feature_count },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "distance",
            |m: &RouteSummary| { &m.distance },
            |m: &mut RouteSummary| { &mut m.distance },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "elapsed_time",
            |m: &RouteSummary| { &m.elapsed_time },
            |m: &mut RouteSummary| { &mut m.elapsed_time },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<RouteSummary>(
            "RouteSummary",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for RouteSummary {
    const NAME: &'static str = "RouteSummary";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.point_count = is.read_int32()?;
                },
                16 => {
                    self.feature_count = is.read_int32()?;
                },
                24 => {
                    self.distance = is.read_int32()?;
                },
                32 => {
                    self.elapsed_time = is.read_int32()?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.point_count != 0 {
            my_size += ::protobufv3::rt::int32_size(1, self.point_count);
        }
        if self.feature_count != 0 {
            my_size += ::protobufv3::rt::int32_size(2, self.feature_count);
        }
        if self.distance != 0 {
            my_size += ::protobufv3::rt::int32_size(3, self.distance);
        }
        if self.elapsed_time != 0 {
            my_size += ::protobufv3::rt::int32_size(4, self.elapsed_time);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.point_count != 0 {
            os.write_int32(1, self.point_count)?;
        }
        if self.feature_count != 0 {
            os.write_int32(2, self.feature_count)?;
        }
        if self.distance != 0 {
            os.write_int32(3, self.distance)?;
        }
        if self.elapsed_time != 0 {
            os.write_int32(4, self.elapsed_time)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RouteSummary {
        RouteSummary::new()
    }

    fn clear(&mut self) {
        self.point_count = 0;
        self.feature_count = 0;
        self.distance = 0;
        self.elapsed_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RouteSummary {
        static instance: RouteSummary = RouteSummary {
            point_count: 0,
            feature_count: 0,
            distance: 0,
            elapsed_time: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for RouteSummary {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RouteSummary").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RouteSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for RouteSummary {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egrpc/example/route_guide.proto\x12\nrouteguide\"A\n\x05Point\x12\
    \x1a\n\x08latitude\x18\x01\x20\x01(\x05R\x08latitude\x12\x1c\n\tlongitud\
    e\x18\x02\x20\x01(\x05R\tlongitude\"Q\n\tRectangle\x12!\n\x02lo\x18\x01\
    \x20\x01(\x0b2\x11.routeguide.PointR\x02lo\x12!\n\x02hi\x18\x02\x20\x01(\
    \x0b2\x11.routeguide.PointR\x02hi\"L\n\x07Feature\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12-\n\x08location\x18\x02\x20\x01(\x0b2\x11.ro\
    uteguide.PointR\x08location\"T\n\tRouteNote\x12-\n\x08location\x18\x01\
    \x20\x01(\x0b2\x11.routeguide.PointR\x08location\x12\x18\n\x07message\
    \x18\x02\x20\x01(\tR\x07message\"\x93\x01\n\x0cRouteSummary\x12\x1f\n\
    \x0bpoint_count\x18\x01\x20\x01(\x05R\npointCount\x12#\n\rfeature_count\
    \x18\x02\x20\x01(\x05R\x0cfeatureCount\x12\x1a\n\x08distance\x18\x03\x20\
    \x01(\x05R\x08distance\x12!\n\x0celapsed_time\x18\x04\x20\x01(\x05R\x0be\
    lapsedTime2\x85\x02\n\nRouteGuide\x126\n\nGetFeature\x12\x11.routeguide.\
    Point\x1a\x13.routeguide.Feature\"\0\x12>\n\x0cListFeatures\x12\x15.rout\
    eguide.Rectangle\x1a\x13.routeguide.Feature\"\00\x01\x12>\n\x0bRecordRou\
    te\x12\x11.routeguide.Point\x1a\x18.routeguide.RouteSummary\"\0(\x01\x12\
    ?\n\tRouteChat\x12\x15.routeguide.RouteNote\x1a\x15.routeguide.RouteNote\
    \"\0(\x010\x01B6\n\x1bio.grpc.examples.routeguideB\x0fRouteGuideProtoP\
    \x01\xa2\x02\x03RTGb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobufv3::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobufv3::rt::Lazy<::protobufv3::descriptor::FileDescriptorProto> = ::protobufv3::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobufv3::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobufv3::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobufv3::rt::Lazy<::protobufv3::reflect::GeneratedFileDescriptor> = ::protobufv3::rt::Lazy::new();
    static file_descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::FileDescriptor> = ::protobufv3::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(Point::generated_message_descriptor_data());
            messages.push(Rectangle::generated_message_descriptor_data());
            messages.push(Feature::generated_message_descriptor_data());
            messages.push(RouteNote::generated_message_descriptor_data());
            messages.push(RouteSummary::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobufv3::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobufv3::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

pub use super::route_guide_grpc::*;