// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_fast_snapshot_restores::_disable_fast_snapshot_restores_output::DisableFastSnapshotRestoresOutputBuilder;

pub use crate::operation::disable_fast_snapshot_restores::_disable_fast_snapshot_restores_input::DisableFastSnapshotRestoresInputBuilder;

/// Fluent builder constructing a request to `DisableFastSnapshotRestores`.
///
/// <p>Disables fast snapshot restores for the specified snapshots in the specified Availability Zones.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableFastSnapshotRestoresFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::disable_fast_snapshot_restores::builders::DisableFastSnapshotRestoresInputBuilder,
}
impl DisableFastSnapshotRestoresFluentBuilder {
    /// Creates a new `DisableFastSnapshotRestores`.
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
            crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestores,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresError,
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
        crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresError,
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
        crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresError,
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
            crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestores,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `AvailabilityZones`.
    ///
    /// To override the contents of this collection use [`set_availability_zones`](Self::set_availability_zones).
    ///
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn availability_zones(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.availability_zones(input.into());
        self
    }
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn set_availability_zones(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_availability_zones(input);
        self
    }
    /// Appends an item to `SourceSnapshotIds`.
    ///
    /// To override the contents of this collection use [`set_source_snapshot_ids`](Self::set_source_snapshot_ids).
    ///
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>.</p>
    pub fn source_snapshot_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_snapshot_ids(input.into());
        self
    }
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>.</p>
    pub fn set_source_snapshot_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_source_snapshot_ids(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
