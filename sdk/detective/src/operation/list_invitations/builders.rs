// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_invitations::_list_invitations_output::ListInvitationsOutputBuilder;

pub use crate::operation::list_invitations::_list_invitations_input::ListInvitationsInputBuilder;

/// Fluent builder constructing a request to `ListInvitations`.
///
/// <p>Retrieves the list of open and accepted behavior graph invitations for the member account. This operation can only be called by an invited member account.</p>
/// <p>Open invitations are invitations that the member account has not responded to.</p>
/// <p>The results do not include behavior graphs for which the member account declined the invitation. The results also do not include behavior graphs that the member account resigned from or was removed from.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListInvitationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_invitations::builders::ListInvitationsInputBuilder,
}
impl ListInvitationsFluentBuilder {
    /// Creates a new `ListInvitations`.
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
            crate::operation::list_invitations::ListInvitations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_invitations::ListInvitationsError,
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
        crate::operation::list_invitations::ListInvitationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_invitations::ListInvitationsError,
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
        crate::operation::list_invitations::ListInvitationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_invitations::ListInvitationsError,
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
            crate::operation::list_invitations::ListInvitations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_invitations::ListInvitationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_invitations::paginator::ListInvitationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_invitations::paginator::ListInvitationsPaginator {
        crate::operation::list_invitations::paginator::ListInvitationsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>For requests to retrieve the next page of results, the pagination token that was returned with the previous page of results. The initial request does not include a pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>For requests to retrieve the next page of results, the pagination token that was returned with the previous page of results. The initial request does not include a pagination token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of behavior graph invitations to return in the response. The total must be less than the overall limit on the number of results to return, which is currently 200.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of behavior graph invitations to return in the response. The total must be less than the overall limit on the number of results to return, which is currently 200.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
