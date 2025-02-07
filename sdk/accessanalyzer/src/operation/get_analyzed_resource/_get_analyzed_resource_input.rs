// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Retrieves an analyzed resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAnalyzedResourceInput {
    /// <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> to retrieve information from.</p>
    #[doc(hidden)]
    pub analyzer_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the resource to retrieve information about.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl GetAnalyzedResourceInput {
    /// <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> to retrieve information from.</p>
    pub fn analyzer_arn(&self) -> ::std::option::Option<&str> {
        self.analyzer_arn.as_deref()
    }
    /// <p>The ARN of the resource to retrieve information about.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl GetAnalyzedResourceInput {
    /// Creates a new builder-style object to manufacture [`GetAnalyzedResourceInput`](crate::operation::get_analyzed_resource::GetAnalyzedResourceInput).
    pub fn builder(
    ) -> crate::operation::get_analyzed_resource::builders::GetAnalyzedResourceInputBuilder {
        crate::operation::get_analyzed_resource::builders::GetAnalyzedResourceInputBuilder::default(
        )
    }
}

/// A builder for [`GetAnalyzedResourceInput`](crate::operation::get_analyzed_resource::GetAnalyzedResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAnalyzedResourceInputBuilder {
    pub(crate) analyzer_arn: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl GetAnalyzedResourceInputBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> to retrieve information from.</p>
    pub fn analyzer_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.analyzer_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> to retrieve information from.</p>
    pub fn set_analyzer_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.analyzer_arn = input;
        self
    }
    /// <p>The ARN of the resource to retrieve information about.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the resource to retrieve information about.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAnalyzedResourceInput`](crate::operation::get_analyzed_resource::GetAnalyzedResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_analyzed_resource::GetAnalyzedResourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_analyzed_resource::GetAnalyzedResourceInput {
                analyzer_arn: self.analyzer_arn,
                resource_arn: self.resource_arn,
            },
        )
    }
}
