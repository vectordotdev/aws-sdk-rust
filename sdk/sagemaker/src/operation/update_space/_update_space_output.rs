// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSpaceOutput {
    /// <p>The space's Amazon Resource Name (ARN).</p>
    #[doc(hidden)]
    pub space_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateSpaceOutput {
    /// <p>The space's Amazon Resource Name (ARN).</p>
    pub fn space_arn(&self) -> ::std::option::Option<&str> {
        self.space_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateSpaceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateSpaceOutput {
    /// Creates a new builder-style object to manufacture [`UpdateSpaceOutput`](crate::operation::update_space::UpdateSpaceOutput).
    pub fn builder() -> crate::operation::update_space::builders::UpdateSpaceOutputBuilder {
        crate::operation::update_space::builders::UpdateSpaceOutputBuilder::default()
    }
}

/// A builder for [`UpdateSpaceOutput`](crate::operation::update_space::UpdateSpaceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateSpaceOutputBuilder {
    pub(crate) space_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateSpaceOutputBuilder {
    /// <p>The space's Amazon Resource Name (ARN).</p>
    pub fn space_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.space_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The space's Amazon Resource Name (ARN).</p>
    pub fn set_space_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.space_arn = input;
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
    /// Consumes the builder and constructs a [`UpdateSpaceOutput`](crate::operation::update_space::UpdateSpaceOutput).
    pub fn build(self) -> crate::operation::update_space::UpdateSpaceOutput {
        crate::operation::update_space::UpdateSpaceOutput {
            space_arn: self.space_arn,
            _request_id: self._request_id,
        }
    }
}
