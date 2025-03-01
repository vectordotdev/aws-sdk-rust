// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteSkillAuthorizationOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DeleteSkillAuthorizationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteSkillAuthorizationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteSkillAuthorizationOutput`](crate::operation::delete_skill_authorization::DeleteSkillAuthorizationOutput).
    pub fn builder(
    ) -> crate::operation::delete_skill_authorization::builders::DeleteSkillAuthorizationOutputBuilder
    {
        crate::operation::delete_skill_authorization::builders::DeleteSkillAuthorizationOutputBuilder::default()
    }
}

/// A builder for [`DeleteSkillAuthorizationOutput`](crate::operation::delete_skill_authorization::DeleteSkillAuthorizationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteSkillAuthorizationOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteSkillAuthorizationOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteSkillAuthorizationOutput`](crate::operation::delete_skill_authorization::DeleteSkillAuthorizationOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_skill_authorization::DeleteSkillAuthorizationOutput {
        crate::operation::delete_skill_authorization::DeleteSkillAuthorizationOutput {
            _request_id: self._request_id,
        }
    }
}
