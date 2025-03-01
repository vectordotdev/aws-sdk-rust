// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSecurityProfileInput {
    /// <p>The identifier for the security profle.</p>
    #[doc(hidden)]
    pub security_profile_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
}
impl DescribeSecurityProfileInput {
    /// <p>The identifier for the security profle.</p>
    pub fn security_profile_id(&self) -> ::std::option::Option<&str> {
        self.security_profile_id.as_deref()
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl DescribeSecurityProfileInput {
    /// Creates a new builder-style object to manufacture [`DescribeSecurityProfileInput`](crate::operation::describe_security_profile::DescribeSecurityProfileInput).
    pub fn builder(
    ) -> crate::operation::describe_security_profile::builders::DescribeSecurityProfileInputBuilder
    {
        crate::operation::describe_security_profile::builders::DescribeSecurityProfileInputBuilder::default()
    }
}

/// A builder for [`DescribeSecurityProfileInput`](crate::operation::describe_security_profile::DescribeSecurityProfileInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeSecurityProfileInputBuilder {
    pub(crate) security_profile_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
}
impl DescribeSecurityProfileInputBuilder {
    /// <p>The identifier for the security profle.</p>
    pub fn security_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.security_profile_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the security profle.</p>
    pub fn set_security_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.security_profile_id = input;
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSecurityProfileInput`](crate::operation::describe_security_profile::DescribeSecurityProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_security_profile::DescribeSecurityProfileInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_security_profile::DescribeSecurityProfileInput {
                security_profile_id: self.security_profile_id,
                instance_id: self.instance_id,
            },
        )
    }
}
