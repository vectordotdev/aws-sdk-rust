// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_source_repository_clone_urls::_get_source_repository_clone_urls_output::GetSourceRepositoryCloneUrlsOutputBuilder;

pub use crate::operation::get_source_repository_clone_urls::_get_source_repository_clone_urls_input::GetSourceRepositoryCloneUrlsInputBuilder;

/// Fluent builder constructing a request to `GetSourceRepositoryCloneUrls`.
///
/// <p>Returns information about the URLs that can be used with a Git client to clone a source repository.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSourceRepositoryCloneUrlsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_source_repository_clone_urls::builders::GetSourceRepositoryCloneUrlsInputBuilder,
}
impl GetSourceRepositoryCloneUrlsFluentBuilder {
    /// Creates a new `GetSourceRepositoryCloneUrls`.
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
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrls,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
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
        crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
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
        crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
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
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrls,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the source repository.</p>
    pub fn source_repository_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_repository_name(input.into());
        self
    }
    /// <p>The name of the source repository.</p>
    pub fn set_source_repository_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_repository_name(input);
        self
    }
}
