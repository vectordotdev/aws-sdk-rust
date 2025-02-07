// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AddTagsToResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource to tag.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>One or more tags.</p>
    #[doc(hidden)]
    pub tag_list: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl AddTagsToResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource to tag.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>One or more tags.</p>
    pub fn tag_list(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tag_list.as_deref()
    }
}
impl AddTagsToResourceInput {
    /// Creates a new builder-style object to manufacture [`AddTagsToResourceInput`](crate::operation::add_tags_to_resource::AddTagsToResourceInput).
    pub fn builder(
    ) -> crate::operation::add_tags_to_resource::builders::AddTagsToResourceInputBuilder {
        crate::operation::add_tags_to_resource::builders::AddTagsToResourceInputBuilder::default()
    }
}

/// A builder for [`AddTagsToResourceInput`](crate::operation::add_tags_to_resource::AddTagsToResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AddTagsToResourceInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tag_list: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl AddTagsToResourceInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource to tag.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource to tag.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Appends an item to `tag_list`.
    ///
    /// To override the contents of this collection use [`set_tag_list`](Self::set_tag_list).
    ///
    /// <p>One or more tags.</p>
    pub fn tag_list(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tag_list.unwrap_or_default();
        v.push(input);
        self.tag_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more tags.</p>
    pub fn set_tag_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tag_list = input;
        self
    }
    /// Consumes the builder and constructs a [`AddTagsToResourceInput`](crate::operation::add_tags_to_resource::AddTagsToResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::add_tags_to_resource::AddTagsToResourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::add_tags_to_resource::AddTagsToResourceInput {
                resource_arn: self.resource_arn,
                tag_list: self.tag_list,
            },
        )
    }
}
