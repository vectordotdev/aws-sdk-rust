// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteUsageLimitInput {
    /// <p>The identifier of the usage limit to delete.</p>
    #[doc(hidden)]
    pub usage_limit_id: ::std::option::Option<::std::string::String>,
}
impl DeleteUsageLimitInput {
    /// <p>The identifier of the usage limit to delete.</p>
    pub fn usage_limit_id(&self) -> ::std::option::Option<&str> {
        self.usage_limit_id.as_deref()
    }
}
impl DeleteUsageLimitInput {
    /// Creates a new builder-style object to manufacture [`DeleteUsageLimitInput`](crate::operation::delete_usage_limit::DeleteUsageLimitInput).
    pub fn builder() -> crate::operation::delete_usage_limit::builders::DeleteUsageLimitInputBuilder
    {
        crate::operation::delete_usage_limit::builders::DeleteUsageLimitInputBuilder::default()
    }
}

/// A builder for [`DeleteUsageLimitInput`](crate::operation::delete_usage_limit::DeleteUsageLimitInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteUsageLimitInputBuilder {
    pub(crate) usage_limit_id: ::std::option::Option<::std::string::String>,
}
impl DeleteUsageLimitInputBuilder {
    /// <p>The identifier of the usage limit to delete.</p>
    pub fn usage_limit_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.usage_limit_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the usage limit to delete.</p>
    pub fn set_usage_limit_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.usage_limit_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteUsageLimitInput`](crate::operation::delete_usage_limit::DeleteUsageLimitInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_usage_limit::DeleteUsageLimitInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_usage_limit::DeleteUsageLimitInput {
                usage_limit_id: self.usage_limit_id,
            },
        )
    }
}
