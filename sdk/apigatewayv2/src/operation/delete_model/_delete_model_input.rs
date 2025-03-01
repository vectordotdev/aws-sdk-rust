// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteModelInput {
    /// <p>The API identifier.</p>
    #[doc(hidden)]
    pub api_id: ::std::option::Option<::std::string::String>,
    /// <p>The model ID.</p>
    #[doc(hidden)]
    pub model_id: ::std::option::Option<::std::string::String>,
}
impl DeleteModelInput {
    /// <p>The API identifier.</p>
    pub fn api_id(&self) -> ::std::option::Option<&str> {
        self.api_id.as_deref()
    }
    /// <p>The model ID.</p>
    pub fn model_id(&self) -> ::std::option::Option<&str> {
        self.model_id.as_deref()
    }
}
impl DeleteModelInput {
    /// Creates a new builder-style object to manufacture [`DeleteModelInput`](crate::operation::delete_model::DeleteModelInput).
    pub fn builder() -> crate::operation::delete_model::builders::DeleteModelInputBuilder {
        crate::operation::delete_model::builders::DeleteModelInputBuilder::default()
    }
}

/// A builder for [`DeleteModelInput`](crate::operation::delete_model::DeleteModelInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteModelInputBuilder {
    pub(crate) api_id: ::std::option::Option<::std::string::String>,
    pub(crate) model_id: ::std::option::Option<::std::string::String>,
}
impl DeleteModelInputBuilder {
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.api_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.api_id = input;
        self
    }
    /// <p>The model ID.</p>
    pub fn model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The model ID.</p>
    pub fn set_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteModelInput`](crate::operation::delete_model::DeleteModelInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_model::DeleteModelInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_model::DeleteModelInput {
            api_id: self.api_id,
            model_id: self.model_id,
        })
    }
}
