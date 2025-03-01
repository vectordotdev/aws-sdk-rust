// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_license_version::_create_license_version_output::CreateLicenseVersionOutputBuilder;

pub use crate::operation::create_license_version::_create_license_version_input::CreateLicenseVersionInputBuilder;

/// Fluent builder constructing a request to `CreateLicenseVersion`.
///
/// <p>Creates a new version of the specified license.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLicenseVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_license_version::builders::CreateLicenseVersionInputBuilder,
}
impl CreateLicenseVersionFluentBuilder {
    /// Creates a new `CreateLicenseVersion`.
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
            crate::operation::create_license_version::CreateLicenseVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_license_version::CreateLicenseVersionError,
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
        crate::operation::create_license_version::CreateLicenseVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_license_version::CreateLicenseVersionError,
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
        crate::operation::create_license_version::CreateLicenseVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_license_version::CreateLicenseVersionError,
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
            crate::operation::create_license_version::CreateLicenseVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_license_version::CreateLicenseVersionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    pub fn license_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.license_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the license.</p>
    pub fn set_license_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_license_arn(input);
        self
    }
    /// <p>License name.</p>
    pub fn license_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.license_name(input.into());
        self
    }
    /// <p>License name.</p>
    pub fn set_license_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_license_name(input);
        self
    }
    /// <p>Product name.</p>
    pub fn product_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.product_name(input.into());
        self
    }
    /// <p>Product name.</p>
    pub fn set_product_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_product_name(input);
        self
    }
    /// <p>License issuer.</p>
    pub fn issuer(mut self, input: crate::types::Issuer) -> Self {
        self.inner = self.inner.issuer(input);
        self
    }
    /// <p>License issuer.</p>
    pub fn set_issuer(mut self, input: ::std::option::Option<crate::types::Issuer>) -> Self {
        self.inner = self.inner.set_issuer(input);
        self
    }
    /// <p>Home Region of the license.</p>
    pub fn home_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.home_region(input.into());
        self
    }
    /// <p>Home Region of the license.</p>
    pub fn set_home_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_home_region(input);
        self
    }
    /// <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    pub fn validity(mut self, input: crate::types::DatetimeRange) -> Self {
        self.inner = self.inner.validity(input);
        self
    }
    /// <p>Date and time range during which the license is valid, in ISO8601-UTC format.</p>
    pub fn set_validity(
        mut self,
        input: ::std::option::Option<crate::types::DatetimeRange>,
    ) -> Self {
        self.inner = self.inner.set_validity(input);
        self
    }
    /// Appends an item to `LicenseMetadata`.
    ///
    /// To override the contents of this collection use [`set_license_metadata`](Self::set_license_metadata).
    ///
    /// <p>Information about the license.</p>
    pub fn license_metadata(mut self, input: crate::types::Metadata) -> Self {
        self.inner = self.inner.license_metadata(input);
        self
    }
    /// <p>Information about the license.</p>
    pub fn set_license_metadata(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Metadata>>,
    ) -> Self {
        self.inner = self.inner.set_license_metadata(input);
        self
    }
    /// Appends an item to `Entitlements`.
    ///
    /// To override the contents of this collection use [`set_entitlements`](Self::set_entitlements).
    ///
    /// <p>License entitlements.</p>
    pub fn entitlements(mut self, input: crate::types::Entitlement) -> Self {
        self.inner = self.inner.entitlements(input);
        self
    }
    /// <p>License entitlements.</p>
    pub fn set_entitlements(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Entitlement>>,
    ) -> Self {
        self.inner = self.inner.set_entitlements(input);
        self
    }
    /// <p>Configuration for consumption of the license. Choose a provisional configuration for workloads running with continuous connectivity. Choose a borrow configuration for workloads with offline usage.</p>
    pub fn consumption_configuration(
        mut self,
        input: crate::types::ConsumptionConfiguration,
    ) -> Self {
        self.inner = self.inner.consumption_configuration(input);
        self
    }
    /// <p>Configuration for consumption of the license. Choose a provisional configuration for workloads running with continuous connectivity. Choose a borrow configuration for workloads with offline usage.</p>
    pub fn set_consumption_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ConsumptionConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_consumption_configuration(input);
        self
    }
    /// <p>License status.</p>
    pub fn status(mut self, input: crate::types::LicenseStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>License status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::LicenseStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Current version of the license.</p>
    pub fn source_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_version(input.into());
        self
    }
    /// <p>Current version of the license.</p>
    pub fn set_source_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_version(input);
        self
    }
}
