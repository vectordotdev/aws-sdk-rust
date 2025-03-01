// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateLinkInput {
    /// <p>The ARN of the link that you want to update.</p>
    #[doc(hidden)]
    pub identifier: ::std::option::Option<::std::string::String>,
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    #[doc(hidden)]
    pub resource_types: ::std::option::Option<::std::vec::Vec<crate::types::ResourceType>>,
}
impl UpdateLinkInput {
    /// <p>The ARN of the link that you want to update.</p>
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    pub fn resource_types(&self) -> ::std::option::Option<&[crate::types::ResourceType]> {
        self.resource_types.as_deref()
    }
}
impl UpdateLinkInput {
    /// Creates a new builder-style object to manufacture [`UpdateLinkInput`](crate::operation::update_link::UpdateLinkInput).
    pub fn builder() -> crate::operation::update_link::builders::UpdateLinkInputBuilder {
        crate::operation::update_link::builders::UpdateLinkInputBuilder::default()
    }
}

/// A builder for [`UpdateLinkInput`](crate::operation::update_link::UpdateLinkInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateLinkInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) resource_types: ::std::option::Option<::std::vec::Vec<crate::types::ResourceType>>,
}
impl UpdateLinkInputBuilder {
    /// <p>The ARN of the link that you want to update.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the link that you want to update.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// Appends an item to `resource_types`.
    ///
    /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
    ///
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    pub fn resource_types(mut self, input: crate::types::ResourceType) -> Self {
        let mut v = self.resource_types.unwrap_or_default();
        v.push(input);
        self.resource_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    pub fn set_resource_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceType>>,
    ) -> Self {
        self.resource_types = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateLinkInput`](crate::operation::update_link::UpdateLinkInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_link::UpdateLinkInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_link::UpdateLinkInput {
            identifier: self.identifier,
            resource_types: self.resource_types,
        })
    }
}
