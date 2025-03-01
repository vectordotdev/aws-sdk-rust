// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_users::_describe_users_output::DescribeUsersOutputBuilder;

pub use crate::operation::describe_users::_describe_users_input::DescribeUsersInputBuilder;

/// Fluent builder constructing a request to `DescribeUsers`.
///
/// <p>Retrieves a list that describes one or more specified users in the user pool.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeUsersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_users::builders::DescribeUsersInputBuilder,
}
impl DescribeUsersFluentBuilder {
    /// Creates a new `DescribeUsers`.
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
            crate::operation::describe_users::DescribeUsers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_users::DescribeUsersError>,
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
        crate::operation::describe_users::DescribeUsersOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_users::DescribeUsersError>,
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
        crate::operation::describe_users::DescribeUsersOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_users::DescribeUsersError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_users::DescribeUsers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_users::DescribeUsersError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The authentication type for the users in the user pool to describe. You must specify USERPOOL.</p>
    pub fn authentication_type(mut self, input: crate::types::AuthenticationType) -> Self {
        self.inner = self.inner.authentication_type(input);
        self
    }
    /// <p>The authentication type for the users in the user pool to describe. You must specify USERPOOL.</p>
    pub fn set_authentication_type(
        mut self,
        input: ::std::option::Option<crate::types::AuthenticationType>,
    ) -> Self {
        self.inner = self.inner.set_authentication_type(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
