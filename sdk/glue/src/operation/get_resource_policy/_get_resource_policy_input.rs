// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourcePolicyInput {
    /// <p>The ARN of the Glue resource for which to retrieve the resource policy. If not supplied, the Data Catalog resource policy is returned. Use <code>GetResourcePolicies</code> to view all existing resource policies. For more information see <a href="https://docs.aws.amazon.com/glue/latest/dg/glue-specifying-resource-arns.html">Specifying Glue Resource ARNs</a>. </p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl GetResourcePolicyInput {
    /// <p>The ARN of the Glue resource for which to retrieve the resource policy. If not supplied, the Data Catalog resource policy is returned. Use <code>GetResourcePolicies</code> to view all existing resource policies. For more information see <a href="https://docs.aws.amazon.com/glue/latest/dg/glue-specifying-resource-arns.html">Specifying Glue Resource ARNs</a>. </p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl GetResourcePolicyInput {
    /// Creates a new builder-style object to manufacture [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
    pub fn builder(
    ) -> crate::operation::get_resource_policy::builders::GetResourcePolicyInputBuilder {
        crate::operation::get_resource_policy::builders::GetResourcePolicyInputBuilder::default()
    }
}

/// A builder for [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetResourcePolicyInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl GetResourcePolicyInputBuilder {
    /// <p>The ARN of the Glue resource for which to retrieve the resource policy. If not supplied, the Data Catalog resource policy is returned. Use <code>GetResourcePolicies</code> to view all existing resource policies. For more information see <a href="https://docs.aws.amazon.com/glue/latest/dg/glue-specifying-resource-arns.html">Specifying Glue Resource ARNs</a>. </p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Glue resource for which to retrieve the resource policy. If not supplied, the Data Catalog resource policy is returned. Use <code>GetResourcePolicies</code> to view all existing resource policies. For more information see <a href="https://docs.aws.amazon.com/glue/latest/dg/glue-specifying-resource-arns.html">Specifying Glue Resource ARNs</a>. </p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_resource_policy::GetResourcePolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_resource_policy::GetResourcePolicyInput {
                resource_arn: self.resource_arn,
            },
        )
    }
}
