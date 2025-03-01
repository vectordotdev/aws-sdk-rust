// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Placeholder documentation for DescribeInputDeviceThumbnailResponse
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub struct DescribeInputDeviceThumbnailOutput {
    /// The binary data for the thumbnail that the Link device has most recently sent to MediaLive.
    pub body: ::aws_smithy_http::byte_stream::ByteStream,
    /// Specifies the media type of the thumbnail.
    #[doc(hidden)]
    pub content_type: ::std::option::Option<crate::types::ContentType>,
    /// The length of the content.
    #[doc(hidden)]
    pub content_length: ::std::option::Option<i64>,
    /// The unique, cacheable version of this thumbnail.
    #[doc(hidden)]
    pub e_tag: ::std::option::Option<::std::string::String>,
    /// The date and time the thumbnail was last updated at the device.
    #[doc(hidden)]
    pub last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl DescribeInputDeviceThumbnailOutput {
    /// The binary data for the thumbnail that the Link device has most recently sent to MediaLive.
    pub fn body(&self) -> &::aws_smithy_http::byte_stream::ByteStream {
        &self.body
    }
    /// Specifies the media type of the thumbnail.
    pub fn content_type(&self) -> ::std::option::Option<&crate::types::ContentType> {
        self.content_type.as_ref()
    }
    /// The length of the content.
    pub fn content_length(&self) -> ::std::option::Option<i64> {
        self.content_length
    }
    /// The unique, cacheable version of this thumbnail.
    pub fn e_tag(&self) -> ::std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// The date and time the thumbnail was last updated at the device.
    pub fn last_modified(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeInputDeviceThumbnailOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeInputDeviceThumbnailOutput {
    /// Creates a new builder-style object to manufacture [`DescribeInputDeviceThumbnailOutput`](crate::operation::describe_input_device_thumbnail::DescribeInputDeviceThumbnailOutput).
    pub fn builder() -> crate::operation::describe_input_device_thumbnail::builders::DescribeInputDeviceThumbnailOutputBuilder{
        crate::operation::describe_input_device_thumbnail::builders::DescribeInputDeviceThumbnailOutputBuilder::default()
    }
}

/// A builder for [`DescribeInputDeviceThumbnailOutput`](crate::operation::describe_input_device_thumbnail::DescribeInputDeviceThumbnailOutput).
#[non_exhaustive]
#[derive(::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeInputDeviceThumbnailOutputBuilder {
    pub(crate) body: ::std::option::Option<::aws_smithy_http::byte_stream::ByteStream>,
    pub(crate) content_type: ::std::option::Option<crate::types::ContentType>,
    pub(crate) content_length: ::std::option::Option<i64>,
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl DescribeInputDeviceThumbnailOutputBuilder {
    /// The binary data for the thumbnail that the Link device has most recently sent to MediaLive.
    pub fn body(mut self, input: ::aws_smithy_http::byte_stream::ByteStream) -> Self {
        self.body = ::std::option::Option::Some(input);
        self
    }
    /// The binary data for the thumbnail that the Link device has most recently sent to MediaLive.
    pub fn set_body(
        mut self,
        input: ::std::option::Option<::aws_smithy_http::byte_stream::ByteStream>,
    ) -> Self {
        self.body = input;
        self
    }
    /// Specifies the media type of the thumbnail.
    pub fn content_type(mut self, input: crate::types::ContentType) -> Self {
        self.content_type = ::std::option::Option::Some(input);
        self
    }
    /// Specifies the media type of the thumbnail.
    pub fn set_content_type(
        mut self,
        input: ::std::option::Option<crate::types::ContentType>,
    ) -> Self {
        self.content_type = input;
        self
    }
    /// The length of the content.
    pub fn content_length(mut self, input: i64) -> Self {
        self.content_length = ::std::option::Option::Some(input);
        self
    }
    /// The length of the content.
    pub fn set_content_length(mut self, input: ::std::option::Option<i64>) -> Self {
        self.content_length = input;
        self
    }
    /// The unique, cacheable version of this thumbnail.
    pub fn e_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.e_tag = ::std::option::Option::Some(input.into());
        self
    }
    /// The unique, cacheable version of this thumbnail.
    pub fn set_e_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.e_tag = input;
        self
    }
    /// The date and time the thumbnail was last updated at the device.
    pub fn last_modified(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified = ::std::option::Option::Some(input);
        self
    }
    /// The date and time the thumbnail was last updated at the device.
    pub fn set_last_modified(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInputDeviceThumbnailOutput`](crate::operation::describe_input_device_thumbnail::DescribeInputDeviceThumbnailOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_input_device_thumbnail::DescribeInputDeviceThumbnailOutput {
        crate::operation::describe_input_device_thumbnail::DescribeInputDeviceThumbnailOutput {
            body: self.body.unwrap_or_default(),
            content_type: self.content_type,
            content_length: self.content_length,
            e_tag: self.e_tag,
            last_modified: self.last_modified,
            _request_id: self._request_id,
        }
    }
}
