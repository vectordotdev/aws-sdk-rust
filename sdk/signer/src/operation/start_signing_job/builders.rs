// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_signing_job::_start_signing_job_output::StartSigningJobOutputBuilder;

pub use crate::operation::start_signing_job::_start_signing_job_input::StartSigningJobInputBuilder;

/// Fluent builder constructing a request to `StartSigningJob`.
///
/// <p>Initiates a signing job to be performed on the code provided. Signing jobs are viewable by the <code>ListSigningJobs</code> operation for two years after they are performed. Note the following requirements: </p>
/// <ul>
/// <li> <p> You must create an Amazon S3 source bucket. For more information, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/gsg/CreatingABucket.html">Create a Bucket</a> in the <i>Amazon S3 Getting Started Guide</i>. </p> </li>
/// <li> <p>Your S3 source bucket must be version enabled.</p> </li>
/// <li> <p>You must create an S3 destination bucket. Code signing uses your S3 destination bucket to write your signed code.</p> </li>
/// <li> <p>You specify the name of the source and destination buckets when calling the <code>StartSigningJob</code> operation.</p> </li>
/// <li> <p>You must also specify a request token that identifies your request to code signing.</p> </li>
/// </ul>
/// <p>You can call the <code>DescribeSigningJob</code> and the <code>ListSigningJobs</code> actions after you call <code>StartSigningJob</code>.</p>
/// <p>For a Java example that shows how to use this action, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/">http://docs.aws.amazon.com/acm/latest/userguide/</a> </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartSigningJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_signing_job::builders::StartSigningJobInputBuilder,
}
impl StartSigningJobFluentBuilder {
    /// Creates a new `StartSigningJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_signing_job::StartSigningJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_signing_job::StartSigningJobError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_signing_job::StartSigningJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_signing_job::StartSigningJobError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_signing_job::StartSigningJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_signing_job::StartSigningJobError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_signing_job::StartSigningJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_signing_job::StartSigningJobError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The S3 bucket that contains the object to sign or a BLOB that contains your raw code.</p>
    pub fn source(mut self, input: crate::types::Source) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>The S3 bucket that contains the object to sign or a BLOB that contains your raw code.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::Source>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The S3 bucket in which to save your signed object. The destination contains the name of your bucket and an optional prefix.</p>
    pub fn destination(mut self, input: crate::types::Destination) -> Self {
        self.inner = self.inner.destination(input);
        self
    }
    /// <p>The S3 bucket in which to save your signed object. The destination contains the name of your bucket and an optional prefix.</p>
    pub fn set_destination(
        mut self,
        input: ::std::option::Option<crate::types::Destination>,
    ) -> Self {
        self.inner = self.inner.set_destination(input);
        self
    }
    /// <p>The name of the signing profile.</p>
    pub fn profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_name(input.into());
        self
    }
    /// <p>The name of the signing profile.</p>
    pub fn set_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_name(input);
        self
    }
    /// <p>String that identifies the signing request. All calls after the first that use this token return the same response as the first call.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>String that identifies the signing request. All calls after the first that use this token return the same response as the first call.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>The AWS account ID of the signing profile owner.</p>
    pub fn profile_owner(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.profile_owner(input.into());
        self
    }
    /// <p>The AWS account ID of the signing profile owner.</p>
    pub fn set_profile_owner(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_profile_owner(input);
        self
    }
}
