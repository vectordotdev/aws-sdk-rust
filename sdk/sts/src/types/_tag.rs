// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>You can pass custom key-value pair attributes when you assume a role or federate a user. These are called session tags. You can then use the session tags to control access to resources. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html">Tagging Amazon Web Services STS Sessions</a> in the <i>IAM User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Tag {
    /// <p>The key for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag keys can’t exceed 128 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>The value for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag values can’t exceed 256 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl Tag {
    /// <p>The key for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag keys can’t exceed 128 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The value for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag values can’t exceed 256 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::types::Tag).
    pub fn builder() -> crate::types::builders::TagBuilder {
        crate::types::builders::TagBuilder::default()
    }
}

/// A builder for [`Tag`](crate::types::Tag).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl TagBuilder {
    /// <p>The key for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag keys can’t exceed 128 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag keys can’t exceed 128 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The value for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag values can’t exceed 256 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag values can’t exceed 256 characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Tag`](crate::types::Tag).
    pub fn build(self) -> crate::types::Tag {
        crate::types::Tag {
            key: self.key,
            value: self.value,
        }
    }
}
