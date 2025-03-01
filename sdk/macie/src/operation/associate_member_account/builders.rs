// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_member_account::_associate_member_account_output::AssociateMemberAccountOutputBuilder;

pub use crate::operation::associate_member_account::_associate_member_account_input::AssociateMemberAccountInputBuilder;

/// Fluent builder constructing a request to `AssociateMemberAccount`.
///
/// <p>(Discontinued) Associates a specified Amazon Web Services account with Amazon Macie Classic as a member account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateMemberAccountFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_member_account::builders::AssociateMemberAccountInputBuilder,
}
impl AssociateMemberAccountFluentBuilder {
    /// Creates a new `AssociateMemberAccount`.
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
            crate::operation::associate_member_account::AssociateMemberAccount,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_member_account::AssociateMemberAccountError,
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
        crate::operation::associate_member_account::AssociateMemberAccountOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_member_account::AssociateMemberAccountError,
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
        crate::operation::associate_member_account::AssociateMemberAccountOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_member_account::AssociateMemberAccountError,
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
            crate::operation::associate_member_account::AssociateMemberAccount,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_member_account::AssociateMemberAccountError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    pub fn member_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.member_account_id(input.into());
        self
    }
    /// <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
    pub fn set_member_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_member_account_id(input);
        self
    }
}
