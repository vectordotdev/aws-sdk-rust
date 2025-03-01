// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A filter for reference stores.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReferenceStoreFilter {
    /// <p>The name to filter on.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The filter's start date.</p>
    #[doc(hidden)]
    pub created_after: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The filter's end date.</p>
    #[doc(hidden)]
    pub created_before: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ReferenceStoreFilter {
    /// <p>The name to filter on.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The filter's start date.</p>
    pub fn created_after(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_after.as_ref()
    }
    /// <p>The filter's end date.</p>
    pub fn created_before(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_before.as_ref()
    }
}
impl ReferenceStoreFilter {
    /// Creates a new builder-style object to manufacture [`ReferenceStoreFilter`](crate::types::ReferenceStoreFilter).
    pub fn builder() -> crate::types::builders::ReferenceStoreFilterBuilder {
        crate::types::builders::ReferenceStoreFilterBuilder::default()
    }
}

/// A builder for [`ReferenceStoreFilter`](crate::types::ReferenceStoreFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReferenceStoreFilterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) created_after: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) created_before: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ReferenceStoreFilterBuilder {
    /// <p>The name to filter on.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name to filter on.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The filter's start date.</p>
    pub fn created_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_after = ::std::option::Option::Some(input);
        self
    }
    /// <p>The filter's start date.</p>
    pub fn set_created_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_after = input;
        self
    }
    /// <p>The filter's end date.</p>
    pub fn created_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_before = ::std::option::Option::Some(input);
        self
    }
    /// <p>The filter's end date.</p>
    pub fn set_created_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_before = input;
        self
    }
    /// Consumes the builder and constructs a [`ReferenceStoreFilter`](crate::types::ReferenceStoreFilter).
    pub fn build(self) -> crate::types::ReferenceStoreFilter {
        crate::types::ReferenceStoreFilter {
            name: self.name,
            created_after: self.created_after,
            created_before: self.created_before,
        }
    }
}
