// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a license type conversion task.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LicenseConversionContext {
    /// <p>The Usage operation value that corresponds to the license type you are converting your resource from. For more information about which platforms correspond to which usage operation values see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/billing-info-fields.html#billing-info">Sample data: usage operation by platform </a> </p>
    #[doc(hidden)]
    pub usage_operation: ::std::option::Option<::std::string::String>,
}
impl LicenseConversionContext {
    /// <p>The Usage operation value that corresponds to the license type you are converting your resource from. For more information about which platforms correspond to which usage operation values see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/billing-info-fields.html#billing-info">Sample data: usage operation by platform </a> </p>
    pub fn usage_operation(&self) -> ::std::option::Option<&str> {
        self.usage_operation.as_deref()
    }
}
impl LicenseConversionContext {
    /// Creates a new builder-style object to manufacture [`LicenseConversionContext`](crate::types::LicenseConversionContext).
    pub fn builder() -> crate::types::builders::LicenseConversionContextBuilder {
        crate::types::builders::LicenseConversionContextBuilder::default()
    }
}

/// A builder for [`LicenseConversionContext`](crate::types::LicenseConversionContext).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LicenseConversionContextBuilder {
    pub(crate) usage_operation: ::std::option::Option<::std::string::String>,
}
impl LicenseConversionContextBuilder {
    /// <p>The Usage operation value that corresponds to the license type you are converting your resource from. For more information about which platforms correspond to which usage operation values see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/billing-info-fields.html#billing-info">Sample data: usage operation by platform </a> </p>
    pub fn usage_operation(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.usage_operation = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Usage operation value that corresponds to the license type you are converting your resource from. For more information about which platforms correspond to which usage operation values see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/billing-info-fields.html#billing-info">Sample data: usage operation by platform </a> </p>
    pub fn set_usage_operation(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.usage_operation = input;
        self
    }
    /// Consumes the builder and constructs a [`LicenseConversionContext`](crate::types::LicenseConversionContext).
    pub fn build(self) -> crate::types::LicenseConversionContext {
        crate::types::LicenseConversionContext {
            usage_operation: self.usage_operation,
        }
    }
}
