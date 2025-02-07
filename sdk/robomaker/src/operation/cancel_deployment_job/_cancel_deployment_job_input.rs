// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[deprecated(
    note = "Support for the AWS RoboMaker application deployment feature has ended. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/fleets.html."
)]
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelDeploymentJobInput {
    /// <p>The deployment job ARN to cancel.</p>
    #[doc(hidden)]
    pub job: ::std::option::Option<::std::string::String>,
}
impl CancelDeploymentJobInput {
    /// <p>The deployment job ARN to cancel.</p>
    pub fn job(&self) -> ::std::option::Option<&str> {
        self.job.as_deref()
    }
}
impl CancelDeploymentJobInput {
    /// Creates a new builder-style object to manufacture [`CancelDeploymentJobInput`](crate::operation::cancel_deployment_job::CancelDeploymentJobInput).
    pub fn builder(
    ) -> crate::operation::cancel_deployment_job::builders::CancelDeploymentJobInputBuilder {
        crate::operation::cancel_deployment_job::builders::CancelDeploymentJobInputBuilder::default(
        )
    }
}

/// A builder for [`CancelDeploymentJobInput`](crate::operation::cancel_deployment_job::CancelDeploymentJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelDeploymentJobInputBuilder {
    pub(crate) job: ::std::option::Option<::std::string::String>,
}
impl CancelDeploymentJobInputBuilder {
    /// <p>The deployment job ARN to cancel.</p>
    pub fn job(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The deployment job ARN to cancel.</p>
    pub fn set_job(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelDeploymentJobInput`](crate::operation::cancel_deployment_job::CancelDeploymentJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_deployment_job::CancelDeploymentJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::cancel_deployment_job::CancelDeploymentJobInput { job: self.job },
        )
    }
}
