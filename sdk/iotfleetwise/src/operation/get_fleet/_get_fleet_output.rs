// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetFleetOutput {
    /// <p> The ID of the fleet.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p> The Amazon Resource Name (ARN) of the fleet. </p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p> A brief description of the fleet. </p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p> The ARN of a signal catalog associated with the fleet. </p>
    #[doc(hidden)]
    pub signal_catalog_arn: ::std::option::Option<::std::string::String>,
    /// <p> The time the fleet was created in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The time the fleet was last updated, in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    #[doc(hidden)]
    pub last_modification_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetFleetOutput {
    /// <p> The ID of the fleet.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p> The Amazon Resource Name (ARN) of the fleet. </p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p> A brief description of the fleet. </p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p> The ARN of a signal catalog associated with the fleet. </p>
    pub fn signal_catalog_arn(&self) -> ::std::option::Option<&str> {
        self.signal_catalog_arn.as_deref()
    }
    /// <p> The time the fleet was created in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p> The time the fleet was last updated, in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn last_modification_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modification_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetFleetOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetFleetOutput {
    /// Creates a new builder-style object to manufacture [`GetFleetOutput`](crate::operation::get_fleet::GetFleetOutput).
    pub fn builder() -> crate::operation::get_fleet::builders::GetFleetOutputBuilder {
        crate::operation::get_fleet::builders::GetFleetOutputBuilder::default()
    }
}

/// A builder for [`GetFleetOutput`](crate::operation::get_fleet::GetFleetOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetFleetOutputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) signal_catalog_arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modification_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetFleetOutputBuilder {
    /// <p> The ID of the fleet.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of the fleet.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the fleet. </p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the fleet. </p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p> A brief description of the fleet. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> A brief description of the fleet. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p> The ARN of a signal catalog associated with the fleet. </p>
    pub fn signal_catalog_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.signal_catalog_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ARN of a signal catalog associated with the fleet. </p>
    pub fn set_signal_catalog_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.signal_catalog_arn = input;
        self
    }
    /// <p> The time the fleet was created in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time the fleet was created in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p> The time the fleet was last updated, in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn last_modification_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modification_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> The time the fleet was last updated, in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn set_last_modification_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modification_time = input;
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
    /// Consumes the builder and constructs a [`GetFleetOutput`](crate::operation::get_fleet::GetFleetOutput).
    pub fn build(self) -> crate::operation::get_fleet::GetFleetOutput {
        crate::operation::get_fleet::GetFleetOutput {
            id: self.id,
            arn: self.arn,
            description: self.description,
            signal_catalog_arn: self.signal_catalog_arn,
            creation_time: self.creation_time,
            last_modification_time: self.last_modification_time,
            _request_id: self._request_id,
        }
    }
}
