// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_package_version::_describe_package_version_output::DescribePackageVersionOutputBuilder;

pub use crate::operation::describe_package_version::_describe_package_version_input::DescribePackageVersionInputBuilder;

/// Fluent builder constructing a request to `DescribePackageVersion`.
///
/// <p>Returns information about a package version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribePackageVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_package_version::builders::DescribePackageVersionInputBuilder,
}
impl DescribePackageVersionFluentBuilder {
    /// Creates a new `DescribePackageVersion`.
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
            crate::operation::describe_package_version::DescribePackageVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_package_version::DescribePackageVersionError,
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
        crate::operation::describe_package_version::DescribePackageVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_package_version::DescribePackageVersionError,
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
        crate::operation::describe_package_version::DescribePackageVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_package_version::DescribePackageVersionError,
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
            crate::operation::describe_package_version::DescribePackageVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_package_version::DescribePackageVersionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The version's owner account.</p>
    pub fn owner_account(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.owner_account(input.into());
        self
    }
    /// <p>The version's owner account.</p>
    pub fn set_owner_account(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_owner_account(input);
        self
    }
    /// <p>The version's ID.</p>
    pub fn package_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_id(input.into());
        self
    }
    /// <p>The version's ID.</p>
    pub fn set_package_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_id(input);
        self
    }
    /// <p>The version's version.</p>
    pub fn package_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.package_version(input.into());
        self
    }
    /// <p>The version's version.</p>
    pub fn set_package_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_package_version(input);
        self
    }
    /// <p>The version's patch version.</p>
    pub fn patch_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.patch_version(input.into());
        self
    }
    /// <p>The version's patch version.</p>
    pub fn set_patch_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_patch_version(input);
        self
    }
}
