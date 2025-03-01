// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagSamlProviderInput {
    /// <p>The ARN of the SAML identity provider in IAM to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub saml_provider_arn: ::std::option::Option<::std::string::String>,
    /// <p>The list of tags that you want to attach to the SAML identity provider in IAM. Each tag consists of a key name and an associated value.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TagSamlProviderInput {
    /// <p>The ARN of the SAML identity provider in IAM to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn saml_provider_arn(&self) -> ::std::option::Option<&str> {
        self.saml_provider_arn.as_deref()
    }
    /// <p>The list of tags that you want to attach to the SAML identity provider in IAM. Each tag consists of a key name and an associated value.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TagSamlProviderInput {
    /// Creates a new builder-style object to manufacture [`TagSamlProviderInput`](crate::operation::tag_saml_provider::TagSamlProviderInput).
    pub fn builder() -> crate::operation::tag_saml_provider::builders::TagSamlProviderInputBuilder {
        crate::operation::tag_saml_provider::builders::TagSamlProviderInputBuilder::default()
    }
}

/// A builder for [`TagSamlProviderInput`](crate::operation::tag_saml_provider::TagSamlProviderInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagSamlProviderInputBuilder {
    pub(crate) saml_provider_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TagSamlProviderInputBuilder {
    /// <p>The ARN of the SAML identity provider in IAM to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn saml_provider_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.saml_provider_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the SAML identity provider in IAM to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_saml_provider_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.saml_provider_arn = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags that you want to attach to the SAML identity provider in IAM. Each tag consists of a key name and an associated value.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of tags that you want to attach to the SAML identity provider in IAM. Each tag consists of a key name and an associated value.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TagSamlProviderInput`](crate::operation::tag_saml_provider::TagSamlProviderInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::tag_saml_provider::TagSamlProviderInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::tag_saml_provider::TagSamlProviderInput {
            saml_provider_arn: self.saml_provider_arn,
            tags: self.tags,
        })
    }
}
