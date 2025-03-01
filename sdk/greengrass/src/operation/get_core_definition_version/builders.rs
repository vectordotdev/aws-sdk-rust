// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_core_definition_version::_get_core_definition_version_output::GetCoreDefinitionVersionOutputBuilder;

pub use crate::operation::get_core_definition_version::_get_core_definition_version_input::GetCoreDefinitionVersionInputBuilder;

/// Fluent builder constructing a request to `GetCoreDefinitionVersion`.
///
/// Retrieves information about a core definition version.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCoreDefinitionVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_core_definition_version::builders::GetCoreDefinitionVersionInputBuilder,
}
impl GetCoreDefinitionVersionFluentBuilder {
    /// Creates a new `GetCoreDefinitionVersion`.
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
            crate::operation::get_core_definition_version::GetCoreDefinitionVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_core_definition_version::GetCoreDefinitionVersionError,
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
        crate::operation::get_core_definition_version::GetCoreDefinitionVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_core_definition_version::GetCoreDefinitionVersionError,
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
        crate::operation::get_core_definition_version::GetCoreDefinitionVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_core_definition_version::GetCoreDefinitionVersionError,
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
            crate::operation::get_core_definition_version::GetCoreDefinitionVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_core_definition_version::GetCoreDefinitionVersionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// The ID of the core definition.
    pub fn core_definition_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.core_definition_id(input.into());
        self
    }
    /// The ID of the core definition.
    pub fn set_core_definition_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_core_definition_id(input);
        self
    }
    /// The ID of the core definition version. This value maps to the ''Version'' property of the corresponding ''VersionInformation'' object, which is returned by ''ListCoreDefinitionVersions'' requests. If the version is the last one that was associated with a core definition, the value also maps to the ''LatestVersion'' property of the corresponding ''DefinitionInformation'' object.
    pub fn core_definition_version_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.core_definition_version_id(input.into());
        self
    }
    /// The ID of the core definition version. This value maps to the ''Version'' property of the corresponding ''VersionInformation'' object, which is returned by ''ListCoreDefinitionVersions'' requests. If the version is the last one that was associated with a core definition, the value also maps to the ''LatestVersion'' property of the corresponding ''DefinitionInformation'' object.
    pub fn set_core_definition_version_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_core_definition_version_id(input);
        self
    }
}
