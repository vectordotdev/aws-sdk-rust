// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_activity_stream::_start_activity_stream_output::StartActivityStreamOutputBuilder;

pub use crate::operation::start_activity_stream::_start_activity_stream_input::StartActivityStreamInputBuilder;

/// Fluent builder constructing a request to `StartActivityStream`.
///
/// <p>Starts a database activity stream to monitor activity on the database. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/DBActivityStreams.html"> Monitoring Amazon Aurora with Database Activity Streams</a> in the <i>Amazon Aurora User Guide</i> or <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/DBActivityStreams.html"> Monitoring Amazon RDS with Database Activity Streams</a> in the <i>Amazon RDS User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartActivityStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_activity_stream::builders::StartActivityStreamInputBuilder,
}
impl StartActivityStreamFluentBuilder {
    /// Creates a new `StartActivityStream`.
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
            crate::operation::start_activity_stream::StartActivityStream,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_activity_stream::StartActivityStreamError,
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
        crate::operation::start_activity_stream::StartActivityStreamOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_activity_stream::StartActivityStreamError,
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
        crate::operation::start_activity_stream::StartActivityStreamOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_activity_stream::StartActivityStreamError,
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
            crate::operation::start_activity_stream::StartActivityStream,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_activity_stream::StartActivityStreamError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the DB cluster, for example, <code>arn:aws:rds:us-east-1:12345667890:cluster:das-cluster</code>.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the DB cluster, for example, <code>arn:aws:rds:us-east-1:12345667890:cluster:das-cluster</code>.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>Specifies the mode of the database activity stream. Database events such as a change or access generate an activity stream event. The database session can handle these events either synchronously or asynchronously.</p>
    pub fn mode(mut self, input: crate::types::ActivityStreamMode) -> Self {
        self.inner = self.inner.mode(input);
        self
    }
    /// <p>Specifies the mode of the database activity stream. Database events such as a change or access generate an activity stream event. The database session can handle these events either synchronously or asynchronously.</p>
    pub fn set_mode(
        mut self,
        input: ::std::option::Option<crate::types::ActivityStreamMode>,
    ) -> Self {
        self.inner = self.inner.set_mode(input);
        self
    }
    /// <p>The Amazon Web Services KMS key identifier for encrypting messages in the database activity stream. The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The Amazon Web Services KMS key identifier for encrypting messages in the database activity stream. The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>Specifies whether or not the database activity stream is to start as soon as possible, regardless of the maintenance window for the database.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>Specifies whether or not the database activity stream is to start as soon as possible, regardless of the maintenance window for the database.</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
    /// <p>Specifies whether the database activity stream includes engine-native audit fields. This option applies to an Oracle or Microsoft SQL Server DB instance. By default, no engine-native audit fields are included.</p>
    pub fn engine_native_audit_fields_included(mut self, input: bool) -> Self {
        self.inner = self.inner.engine_native_audit_fields_included(input);
        self
    }
    /// <p>Specifies whether the database activity stream includes engine-native audit fields. This option applies to an Oracle or Microsoft SQL Server DB instance. By default, no engine-native audit fields are included.</p>
    pub fn set_engine_native_audit_fields_included(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_engine_native_audit_fields_included(input);
        self
    }
}
