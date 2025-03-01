// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTargetResourceTypeOutput {
    /// <p>Information about the resource type.</p>
    #[doc(hidden)]
    pub target_resource_type: ::std::option::Option<crate::types::TargetResourceType>,
    _request_id: Option<String>,
}
impl GetTargetResourceTypeOutput {
    /// <p>Information about the resource type.</p>
    pub fn target_resource_type(&self) -> ::std::option::Option<&crate::types::TargetResourceType> {
        self.target_resource_type.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetTargetResourceTypeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTargetResourceTypeOutput {
    /// Creates a new builder-style object to manufacture [`GetTargetResourceTypeOutput`](crate::operation::get_target_resource_type::GetTargetResourceTypeOutput).
    pub fn builder(
    ) -> crate::operation::get_target_resource_type::builders::GetTargetResourceTypeOutputBuilder
    {
        crate::operation::get_target_resource_type::builders::GetTargetResourceTypeOutputBuilder::default()
    }
}

/// A builder for [`GetTargetResourceTypeOutput`](crate::operation::get_target_resource_type::GetTargetResourceTypeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTargetResourceTypeOutputBuilder {
    pub(crate) target_resource_type: ::std::option::Option<crate::types::TargetResourceType>,
    _request_id: Option<String>,
}
impl GetTargetResourceTypeOutputBuilder {
    /// <p>Information about the resource type.</p>
    pub fn target_resource_type(mut self, input: crate::types::TargetResourceType) -> Self {
        self.target_resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the resource type.</p>
    pub fn set_target_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::TargetResourceType>,
    ) -> Self {
        self.target_resource_type = input;
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
    /// Consumes the builder and constructs a [`GetTargetResourceTypeOutput`](crate::operation::get_target_resource_type::GetTargetResourceTypeOutput).
    pub fn build(self) -> crate::operation::get_target_resource_type::GetTargetResourceTypeOutput {
        crate::operation::get_target_resource_type::GetTargetResourceTypeOutput {
            target_resource_type: self.target_resource_type,
            _request_id: self._request_id,
        }
    }
}
