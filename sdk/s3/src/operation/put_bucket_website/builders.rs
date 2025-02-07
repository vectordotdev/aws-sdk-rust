// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_website::_put_bucket_website_output::PutBucketWebsiteOutputBuilder;

pub use crate::operation::put_bucket_website::_put_bucket_website_input::PutBucketWebsiteInputBuilder;

/// Fluent builder constructing a request to `PutBucketWebsite`.
///
/// <p>Sets the configuration of the website that is specified in the <code>website</code> subresource. To configure a bucket as a website, you can add this subresource on the bucket with website configuration information such as the file name of the index document and any redirect rules. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on Amazon S3</a>.</p>
/// <p>This PUT action requires the <code>S3:PutBucketWebsite</code> permission. By default, only the bucket owner can configure the website attached to a bucket; however, bucket owners can allow other users to set the website configuration by writing a bucket policy that grants them the <code>S3:PutBucketWebsite</code> permission.</p>
/// <p>To redirect all website requests sent to the bucket's website endpoint, you add a website configuration with the following elements. Because all requests are sent to another website, you don't need to provide index document name for the bucket.</p>
/// <ul>
/// <li> <p> <code>WebsiteConfiguration</code> </p> </li>
/// <li> <p> <code>RedirectAllRequestsTo</code> </p> </li>
/// <li> <p> <code>HostName</code> </p> </li>
/// <li> <p> <code>Protocol</code> </p> </li>
/// </ul>
/// <p>If you want granular control over redirects, you can use the following elements to add routing rules that describe conditions for redirecting requests and information about the redirect destination. In this case, the website configuration must provide an index document for the bucket, because some requests might not be redirected. </p>
/// <ul>
/// <li> <p> <code>WebsiteConfiguration</code> </p> </li>
/// <li> <p> <code>IndexDocument</code> </p> </li>
/// <li> <p> <code>Suffix</code> </p> </li>
/// <li> <p> <code>ErrorDocument</code> </p> </li>
/// <li> <p> <code>Key</code> </p> </li>
/// <li> <p> <code>RoutingRules</code> </p> </li>
/// <li> <p> <code>RoutingRule</code> </p> </li>
/// <li> <p> <code>Condition</code> </p> </li>
/// <li> <p> <code>HttpErrorCodeReturnedEquals</code> </p> </li>
/// <li> <p> <code>KeyPrefixEquals</code> </p> </li>
/// <li> <p> <code>Redirect</code> </p> </li>
/// <li> <p> <code>Protocol</code> </p> </li>
/// <li> <p> <code>HostName</code> </p> </li>
/// <li> <p> <code>ReplaceKeyPrefixWith</code> </p> </li>
/// <li> <p> <code>ReplaceKeyWith</code> </p> </li>
/// <li> <p> <code>HttpRedirectCode</code> </p> </li>
/// </ul>
/// <p>Amazon S3 has a limitation of 50 routing rules per website configuration. If you require more than 50 routing rules, you can use object redirect. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html">Configuring an Object Redirect</a> in the <i>Amazon S3 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutBucketWebsiteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_bucket_website::builders::PutBucketWebsiteInputBuilder,
}
impl PutBucketWebsiteFluentBuilder {
    /// Creates a new `PutBucketWebsite`.
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
            crate::operation::put_bucket_website::PutBucketWebsite,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_website::PutBucketWebsiteError,
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
        crate::operation::put_bucket_website::PutBucketWebsiteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_website::PutBucketWebsiteError,
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
        crate::operation::put_bucket_website::PutBucketWebsiteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_website::PutBucketWebsiteError,
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
            crate::operation::put_bucket_website::PutBucketWebsite,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_website::PutBucketWebsiteError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The bucket name.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The bucket name.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn set_checksum_algorithm(
        mut self,
        input: ::std::option::Option<crate::types::ChecksumAlgorithm>,
    ) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
        self
    }
    /// <p>Container for the request.</p>
    pub fn website_configuration(mut self, input: crate::types::WebsiteConfiguration) -> Self {
        self.inner = self.inner.website_configuration(input);
        self
    }
    /// <p>Container for the request.</p>
    pub fn set_website_configuration(
        mut self,
        input: ::std::option::Option<crate::types::WebsiteConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_website_configuration(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
}
