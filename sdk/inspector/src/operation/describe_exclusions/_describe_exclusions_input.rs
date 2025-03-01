// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExclusionsInput {
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    #[doc(hidden)]
    pub exclusion_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    #[doc(hidden)]
    pub locale: ::std::option::Option<crate::types::Locale>,
}
impl DescribeExclusionsInput {
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    pub fn exclusion_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.exclusion_arns.as_deref()
    }
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    pub fn locale(&self) -> ::std::option::Option<&crate::types::Locale> {
        self.locale.as_ref()
    }
}
impl DescribeExclusionsInput {
    /// Creates a new builder-style object to manufacture [`DescribeExclusionsInput`](crate::operation::describe_exclusions::DescribeExclusionsInput).
    pub fn builder(
    ) -> crate::operation::describe_exclusions::builders::DescribeExclusionsInputBuilder {
        crate::operation::describe_exclusions::builders::DescribeExclusionsInputBuilder::default()
    }
}

/// A builder for [`DescribeExclusionsInput`](crate::operation::describe_exclusions::DescribeExclusionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeExclusionsInputBuilder {
    pub(crate) exclusion_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) locale: ::std::option::Option<crate::types::Locale>,
}
impl DescribeExclusionsInputBuilder {
    /// Appends an item to `exclusion_arns`.
    ///
    /// To override the contents of this collection use [`set_exclusion_arns`](Self::set_exclusion_arns).
    ///
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    pub fn exclusion_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.exclusion_arns.unwrap_or_default();
        v.push(input.into());
        self.exclusion_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    pub fn set_exclusion_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.exclusion_arns = input;
        self
    }
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    pub fn locale(mut self, input: crate::types::Locale) -> Self {
        self.locale = ::std::option::Option::Some(input);
        self
    }
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    pub fn set_locale(mut self, input: ::std::option::Option<crate::types::Locale>) -> Self {
        self.locale = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeExclusionsInput`](crate::operation::describe_exclusions::DescribeExclusionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_exclusions::DescribeExclusionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_exclusions::DescribeExclusionsInput {
                exclusion_arns: self.exclusion_arns,
                locale: self.locale,
            },
        )
    }
}
