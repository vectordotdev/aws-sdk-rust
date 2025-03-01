// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTemplateInput {
    /// <p>The unique identifier of the Cases domain. </p>
    #[doc(hidden)]
    pub domain_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier of a template.</p>
    #[doc(hidden)]
    pub template_id: ::std::option::Option<::std::string::String>,
}
impl GetTemplateInput {
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn domain_id(&self) -> ::std::option::Option<&str> {
        self.domain_id.as_deref()
    }
    /// <p>A unique identifier of a template.</p>
    pub fn template_id(&self) -> ::std::option::Option<&str> {
        self.template_id.as_deref()
    }
}
impl GetTemplateInput {
    /// Creates a new builder-style object to manufacture [`GetTemplateInput`](crate::operation::get_template::GetTemplateInput).
    pub fn builder() -> crate::operation::get_template::builders::GetTemplateInputBuilder {
        crate::operation::get_template::builders::GetTemplateInputBuilder::default()
    }
}

/// A builder for [`GetTemplateInput`](crate::operation::get_template::GetTemplateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTemplateInputBuilder {
    pub(crate) domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) template_id: ::std::option::Option<::std::string::String>,
}
impl GetTemplateInputBuilder {
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the Cases domain. </p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_id = input;
        self
    }
    /// <p>A unique identifier of a template.</p>
    pub fn template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier of a template.</p>
    pub fn set_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.template_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetTemplateInput`](crate::operation::get_template::GetTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_template::GetTemplateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_template::GetTemplateInput {
            domain_id: self.domain_id,
            template_id: self.template_id,
        })
    }
}
