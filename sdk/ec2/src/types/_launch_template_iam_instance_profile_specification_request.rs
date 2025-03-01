// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An IAM instance profile.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchTemplateIamInstanceProfileSpecificationRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the instance profile.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl LaunchTemplateIamInstanceProfileSpecificationRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the instance profile.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl LaunchTemplateIamInstanceProfileSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateIamInstanceProfileSpecificationRequest`](crate::types::LaunchTemplateIamInstanceProfileSpecificationRequest).
    pub fn builder(
    ) -> crate::types::builders::LaunchTemplateIamInstanceProfileSpecificationRequestBuilder {
        crate::types::builders::LaunchTemplateIamInstanceProfileSpecificationRequestBuilder::default(
        )
    }
}

/// A builder for [`LaunchTemplateIamInstanceProfileSpecificationRequest`](crate::types::LaunchTemplateIamInstanceProfileSpecificationRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LaunchTemplateIamInstanceProfileSpecificationRequestBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl LaunchTemplateIamInstanceProfileSpecificationRequestBuilder {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the instance profile.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the instance profile.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateIamInstanceProfileSpecificationRequest`](crate::types::LaunchTemplateIamInstanceProfileSpecificationRequest).
    pub fn build(self) -> crate::types::LaunchTemplateIamInstanceProfileSpecificationRequest {
        crate::types::LaunchTemplateIamInstanceProfileSpecificationRequest {
            arn: self.arn,
            name: self.name,
        }
    }
}
