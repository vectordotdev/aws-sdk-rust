// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagQueueInput {
    /// <p>The URL of the queue.</p>
    #[doc(hidden)]
    pub queue_url: ::std::option::Option<::std::string::String>,
    /// <p>The list of tags to be added to the specified queue.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl TagQueueInput {
    /// <p>The URL of the queue.</p>
    pub fn queue_url(&self) -> ::std::option::Option<&str> {
        self.queue_url.as_deref()
    }
    /// <p>The list of tags to be added to the specified queue.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl TagQueueInput {
    /// Creates a new builder-style object to manufacture [`TagQueueInput`](crate::operation::tag_queue::TagQueueInput).
    pub fn builder() -> crate::operation::tag_queue::builders::TagQueueInputBuilder {
        crate::operation::tag_queue::builders::TagQueueInputBuilder::default()
    }
}

/// A builder for [`TagQueueInput`](crate::operation::tag_queue::TagQueueInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagQueueInputBuilder {
    pub(crate) queue_url: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl TagQueueInputBuilder {
    /// <p>The URL of the queue.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the queue.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_url = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags to be added to the specified queue.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The list of tags to be added to the specified queue.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TagQueueInput`](crate::operation::tag_queue::TagQueueInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::tag_queue::TagQueueInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::tag_queue::TagQueueInput {
            queue_url: self.queue_url,
            tags: self.tags,
        })
    }
}
