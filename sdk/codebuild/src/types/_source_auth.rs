// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the authorization settings for CodeBuild to access the source code to be built.</p>
/// <p>This information is for the CodeBuild console's use only. Your code should not get or set this information directly.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourceAuth {
    /// <note>
    /// <p> This data type is deprecated and is no longer accurate or used. </p>
    /// </note>
    /// <p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::SourceAuthType>,
    /// <p>The resource value that applies to the specified authorization type.</p>
    #[doc(hidden)]
    pub resource: ::std::option::Option<::std::string::String>,
}
impl SourceAuth {
    /// <note>
    /// <p> This data type is deprecated and is no longer accurate or used. </p>
    /// </note>
    /// <p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::SourceAuthType> {
        self.r#type.as_ref()
    }
    /// <p>The resource value that applies to the specified authorization type.</p>
    pub fn resource(&self) -> ::std::option::Option<&str> {
        self.resource.as_deref()
    }
}
impl SourceAuth {
    /// Creates a new builder-style object to manufacture [`SourceAuth`](crate::types::SourceAuth).
    pub fn builder() -> crate::types::builders::SourceAuthBuilder {
        crate::types::builders::SourceAuthBuilder::default()
    }
}

/// A builder for [`SourceAuth`](crate::types::SourceAuth).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourceAuthBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::SourceAuthType>,
    pub(crate) resource: ::std::option::Option<::std::string::String>,
}
impl SourceAuthBuilder {
    /// <note>
    /// <p> This data type is deprecated and is no longer accurate or used. </p>
    /// </note>
    /// <p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p>
    pub fn r#type(mut self, input: crate::types::SourceAuthType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <note>
    /// <p> This data type is deprecated and is no longer accurate or used. </p>
    /// </note>
    /// <p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::SourceAuthType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The resource value that applies to the specified authorization type.</p>
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The resource value that applies to the specified authorization type.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceAuth`](crate::types::SourceAuth).
    pub fn build(self) -> crate::types::SourceAuth {
        crate::types::SourceAuth {
            r#type: self.r#type,
            resource: self.resource,
        }
    }
}
