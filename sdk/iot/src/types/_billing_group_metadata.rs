// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Additional information about the billing group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BillingGroupMetadata {
    /// <p>The date the billing group was created.</p>
    #[doc(hidden)]
    pub creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BillingGroupMetadata {
    /// <p>The date the billing group was created.</p>
    pub fn creation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_date.as_ref()
    }
}
impl BillingGroupMetadata {
    /// Creates a new builder-style object to manufacture [`BillingGroupMetadata`](crate::types::BillingGroupMetadata).
    pub fn builder() -> crate::types::builders::BillingGroupMetadataBuilder {
        crate::types::builders::BillingGroupMetadataBuilder::default()
    }
}

/// A builder for [`BillingGroupMetadata`](crate::types::BillingGroupMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BillingGroupMetadataBuilder {
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BillingGroupMetadataBuilder {
    /// <p>The date the billing group was created.</p>
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the billing group was created.</p>
    pub fn set_creation_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_date = input;
        self
    }
    /// Consumes the builder and constructs a [`BillingGroupMetadata`](crate::types::BillingGroupMetadata).
    pub fn build(self) -> crate::types::BillingGroupMetadata {
        crate::types::BillingGroupMetadata {
            creation_date: self.creation_date,
        }
    }
}
