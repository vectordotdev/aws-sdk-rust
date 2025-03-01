// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>PrincipalIdFormat description</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PrincipalIdFormat {
    /// <p>PrincipalIdFormatARN description</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>PrincipalIdFormatStatuses description</p>
    #[doc(hidden)]
    pub statuses: ::std::option::Option<::std::vec::Vec<crate::types::IdFormat>>,
}
impl PrincipalIdFormat {
    /// <p>PrincipalIdFormatARN description</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>PrincipalIdFormatStatuses description</p>
    pub fn statuses(&self) -> ::std::option::Option<&[crate::types::IdFormat]> {
        self.statuses.as_deref()
    }
}
impl PrincipalIdFormat {
    /// Creates a new builder-style object to manufacture [`PrincipalIdFormat`](crate::types::PrincipalIdFormat).
    pub fn builder() -> crate::types::builders::PrincipalIdFormatBuilder {
        crate::types::builders::PrincipalIdFormatBuilder::default()
    }
}

/// A builder for [`PrincipalIdFormat`](crate::types::PrincipalIdFormat).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PrincipalIdFormatBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) statuses: ::std::option::Option<::std::vec::Vec<crate::types::IdFormat>>,
}
impl PrincipalIdFormatBuilder {
    /// <p>PrincipalIdFormatARN description</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>PrincipalIdFormatARN description</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Appends an item to `statuses`.
    ///
    /// To override the contents of this collection use [`set_statuses`](Self::set_statuses).
    ///
    /// <p>PrincipalIdFormatStatuses description</p>
    pub fn statuses(mut self, input: crate::types::IdFormat) -> Self {
        let mut v = self.statuses.unwrap_or_default();
        v.push(input);
        self.statuses = ::std::option::Option::Some(v);
        self
    }
    /// <p>PrincipalIdFormatStatuses description</p>
    pub fn set_statuses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IdFormat>>,
    ) -> Self {
        self.statuses = input;
        self
    }
    /// Consumes the builder and constructs a [`PrincipalIdFormat`](crate::types::PrincipalIdFormat).
    pub fn build(self) -> crate::types::PrincipalIdFormat {
        crate::types::PrincipalIdFormat {
            arn: self.arn,
            statuses: self.statuses,
        }
    }
}
