// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCustomDataIdentifierInput {
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetCustomDataIdentifierInput {
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetCustomDataIdentifierInput {
    /// Creates a new builder-style object to manufacture [`GetCustomDataIdentifierInput`](crate::operation::get_custom_data_identifier::GetCustomDataIdentifierInput).
    pub fn builder(
    ) -> crate::operation::get_custom_data_identifier::builders::GetCustomDataIdentifierInputBuilder
    {
        crate::operation::get_custom_data_identifier::builders::GetCustomDataIdentifierInputBuilder::default()
    }
}

/// A builder for [`GetCustomDataIdentifierInput`](crate::operation::get_custom_data_identifier::GetCustomDataIdentifierInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCustomDataIdentifierInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetCustomDataIdentifierInputBuilder {
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetCustomDataIdentifierInput`](crate::operation::get_custom_data_identifier::GetCustomDataIdentifierInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_custom_data_identifier::GetCustomDataIdentifierInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_custom_data_identifier::GetCustomDataIdentifierInput {
                id: self.id,
            },
        )
    }
}
