// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDeviceDefinitionVersionsInput {
    /// The ID of the device definition.
    #[doc(hidden)]
    pub device_definition_id: ::std::option::Option<::std::string::String>,
    /// The maximum number of results to be returned per request.
    #[doc(hidden)]
    pub max_results: ::std::option::Option<::std::string::String>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListDeviceDefinitionVersionsInput {
    /// The ID of the device definition.
    pub fn device_definition_id(&self) -> ::std::option::Option<&str> {
        self.device_definition_id.as_deref()
    }
    /// The maximum number of results to be returned per request.
    pub fn max_results(&self) -> ::std::option::Option<&str> {
        self.max_results.as_deref()
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListDeviceDefinitionVersionsInput {
    /// Creates a new builder-style object to manufacture [`ListDeviceDefinitionVersionsInput`](crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsInput).
    pub fn builder() -> crate::operation::list_device_definition_versions::builders::ListDeviceDefinitionVersionsInputBuilder{
        crate::operation::list_device_definition_versions::builders::ListDeviceDefinitionVersionsInputBuilder::default()
    }
}

/// A builder for [`ListDeviceDefinitionVersionsInput`](crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListDeviceDefinitionVersionsInputBuilder {
    pub(crate) device_definition_id: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListDeviceDefinitionVersionsInputBuilder {
    /// The ID of the device definition.
    pub fn device_definition_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.device_definition_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the device definition.
    pub fn set_device_definition_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.device_definition_id = input;
        self
    }
    /// The maximum number of results to be returned per request.
    pub fn max_results(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.max_results = ::std::option::Option::Some(input.into());
        self
    }
    /// The maximum number of results to be returned per request.
    pub fn set_max_results(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.max_results = input;
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListDeviceDefinitionVersionsInput`](crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_device_definition_versions::ListDeviceDefinitionVersionsInput {
                device_definition_id: self.device_definition_id,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
