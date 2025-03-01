// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAssessmentFrameworkShareInput {
    /// <p>The unique identifier for the share request to be deleted.</p>
    #[doc(hidden)]
    pub request_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether the share request is a sent request or a received request.</p>
    #[doc(hidden)]
    pub request_type: ::std::option::Option<crate::types::ShareRequestType>,
}
impl DeleteAssessmentFrameworkShareInput {
    /// <p>The unique identifier for the share request to be deleted.</p>
    pub fn request_id(&self) -> ::std::option::Option<&str> {
        self.request_id.as_deref()
    }
    /// <p>Specifies whether the share request is a sent request or a received request.</p>
    pub fn request_type(&self) -> ::std::option::Option<&crate::types::ShareRequestType> {
        self.request_type.as_ref()
    }
}
impl DeleteAssessmentFrameworkShareInput {
    /// Creates a new builder-style object to manufacture [`DeleteAssessmentFrameworkShareInput`](crate::operation::delete_assessment_framework_share::DeleteAssessmentFrameworkShareInput).
    pub fn builder() -> crate::operation::delete_assessment_framework_share::builders::DeleteAssessmentFrameworkShareInputBuilder{
        crate::operation::delete_assessment_framework_share::builders::DeleteAssessmentFrameworkShareInputBuilder::default()
    }
}

/// A builder for [`DeleteAssessmentFrameworkShareInput`](crate::operation::delete_assessment_framework_share::DeleteAssessmentFrameworkShareInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteAssessmentFrameworkShareInputBuilder {
    pub(crate) request_id: ::std::option::Option<::std::string::String>,
    pub(crate) request_type: ::std::option::Option<crate::types::ShareRequestType>,
}
impl DeleteAssessmentFrameworkShareInputBuilder {
    /// <p>The unique identifier for the share request to be deleted.</p>
    pub fn request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.request_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the share request to be deleted.</p>
    pub fn set_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.request_id = input;
        self
    }
    /// <p>Specifies whether the share request is a sent request or a received request.</p>
    pub fn request_type(mut self, input: crate::types::ShareRequestType) -> Self {
        self.request_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the share request is a sent request or a received request.</p>
    pub fn set_request_type(
        mut self,
        input: ::std::option::Option<crate::types::ShareRequestType>,
    ) -> Self {
        self.request_type = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAssessmentFrameworkShareInput`](crate::operation::delete_assessment_framework_share::DeleteAssessmentFrameworkShareInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_assessment_framework_share::DeleteAssessmentFrameworkShareInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_assessment_framework_share::DeleteAssessmentFrameworkShareInput {
                request_id: self.request_id
                ,
                request_type: self.request_type
                ,
            }
        )
    }
}
