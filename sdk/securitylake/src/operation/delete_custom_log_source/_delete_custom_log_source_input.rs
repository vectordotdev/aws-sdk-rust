// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteCustomLogSourceInput {
    /// <p>The custom source name for the custom log source.</p>
    #[doc(hidden)]
    pub custom_source_name: ::std::option::Option<::std::string::String>,
}
impl DeleteCustomLogSourceInput {
    /// <p>The custom source name for the custom log source.</p>
    pub fn custom_source_name(&self) -> ::std::option::Option<&str> {
        self.custom_source_name.as_deref()
    }
}
impl DeleteCustomLogSourceInput {
    /// Creates a new builder-style object to manufacture [`DeleteCustomLogSourceInput`](crate::operation::delete_custom_log_source::DeleteCustomLogSourceInput).
    pub fn builder(
    ) -> crate::operation::delete_custom_log_source::builders::DeleteCustomLogSourceInputBuilder
    {
        crate::operation::delete_custom_log_source::builders::DeleteCustomLogSourceInputBuilder::default()
    }
}

/// A builder for [`DeleteCustomLogSourceInput`](crate::operation::delete_custom_log_source::DeleteCustomLogSourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteCustomLogSourceInputBuilder {
    pub(crate) custom_source_name: ::std::option::Option<::std::string::String>,
}
impl DeleteCustomLogSourceInputBuilder {
    /// <p>The custom source name for the custom log source.</p>
    pub fn custom_source_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.custom_source_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom source name for the custom log source.</p>
    pub fn set_custom_source_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.custom_source_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteCustomLogSourceInput`](crate::operation::delete_custom_log_source::DeleteCustomLogSourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_custom_log_source::DeleteCustomLogSourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_custom_log_source::DeleteCustomLogSourceInput {
                custom_source_name: self.custom_source_name,
            },
        )
    }
}
