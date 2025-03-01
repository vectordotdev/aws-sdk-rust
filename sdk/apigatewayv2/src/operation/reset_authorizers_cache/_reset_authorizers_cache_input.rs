// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResetAuthorizersCacheInput {
    /// <p>The API identifier.</p>
    #[doc(hidden)]
    pub api_id: ::std::option::Option<::std::string::String>,
    /// <p>The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.</p>
    #[doc(hidden)]
    pub stage_name: ::std::option::Option<::std::string::String>,
}
impl ResetAuthorizersCacheInput {
    /// <p>The API identifier.</p>
    pub fn api_id(&self) -> ::std::option::Option<&str> {
        self.api_id.as_deref()
    }
    /// <p>The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.</p>
    pub fn stage_name(&self) -> ::std::option::Option<&str> {
        self.stage_name.as_deref()
    }
}
impl ResetAuthorizersCacheInput {
    /// Creates a new builder-style object to manufacture [`ResetAuthorizersCacheInput`](crate::operation::reset_authorizers_cache::ResetAuthorizersCacheInput).
    pub fn builder(
    ) -> crate::operation::reset_authorizers_cache::builders::ResetAuthorizersCacheInputBuilder
    {
        crate::operation::reset_authorizers_cache::builders::ResetAuthorizersCacheInputBuilder::default()
    }
}

/// A builder for [`ResetAuthorizersCacheInput`](crate::operation::reset_authorizers_cache::ResetAuthorizersCacheInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResetAuthorizersCacheInputBuilder {
    pub(crate) api_id: ::std::option::Option<::std::string::String>,
    pub(crate) stage_name: ::std::option::Option<::std::string::String>,
}
impl ResetAuthorizersCacheInputBuilder {
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
    /// <p>The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.</p>
    pub fn stage_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stage_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.</p>
    pub fn set_stage_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stage_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ResetAuthorizersCacheInput`](crate::operation::reset_authorizers_cache::ResetAuthorizersCacheInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reset_authorizers_cache::ResetAuthorizersCacheInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::reset_authorizers_cache::ResetAuthorizersCacheInput {
                api_id: self.api_id,
                stage_name: self.stage_name,
            },
        )
    }
}
