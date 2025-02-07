// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_package::_update_package_output::UpdatePackageOutputBuilder;

pub use crate::operation::update_package::_update_package_input::UpdatePackageInputBuilder;

/// Fluent builder constructing a request to `UpdatePackage`.
///
/// <p>Updates a package for use with Amazon OpenSearch Service domains. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/custom-packages.html">Custom packages for Amazon OpenSearch Service</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePackageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_package::builders::UpdatePackageInputBuilder,
}
impl UpdatePackageFluentBuilder {
    /// Creates a new `UpdatePackage`.
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
            crate::operation::update_package::UpdatePackage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_package::UpdatePackageError>,
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
        crate::operation::update_package::UpdatePackageOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_package::UpdatePackageError>,
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
        crate::operation::update_package::UpdatePackageOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_package::UpdatePackageError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_package::UpdatePackage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_package::UpdatePackageError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier for the package.</p>
    pub fn package_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_id(input.into());
        self
    }
    /// <p>The unique identifier for the package.</p>
    pub fn set_package_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_id(input);
        self
    }
    /// <p>Amazon S3 bucket and key for the package.</p>
    pub fn package_source(mut self, input: crate::types::PackageSource) -> Self {
        self.inner = self.inner.package_source(input);
        self
    }
    /// <p>Amazon S3 bucket and key for the package.</p>
    pub fn set_package_source(
        mut self,
        input: ::std::option::Option<crate::types::PackageSource>,
    ) -> Self {
        self.inner = self.inner.set_package_source(input);
        self
    }
    /// <p>A new description of the package.</p>
    pub fn package_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.package_description(input.into());
        self
    }
    /// <p>A new description of the package.</p>
    pub fn set_package_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_package_description(input);
        self
    }
    /// <p>Commit message for the updated file, which is shown as part of <code>GetPackageVersionHistoryResponse</code>.</p>
    pub fn commit_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.commit_message(input.into());
        self
    }
    /// <p>Commit message for the updated file, which is shown as part of <code>GetPackageVersionHistoryResponse</code>.</p>
    pub fn set_commit_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_commit_message(input);
        self
    }
}
