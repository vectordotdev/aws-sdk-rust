// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_primary_email_address::_update_primary_email_address_output::UpdatePrimaryEmailAddressOutputBuilder;

pub use crate::operation::update_primary_email_address::_update_primary_email_address_input::UpdatePrimaryEmailAddressInputBuilder;

/// Fluent builder constructing a request to `UpdatePrimaryEmailAddress`.
///
/// <p>Updates the primary email for a user, group, or resource. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email), and the email provided in the input is promoted as the primary.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePrimaryEmailAddressFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_primary_email_address::builders::UpdatePrimaryEmailAddressInputBuilder,
}
impl UpdatePrimaryEmailAddressFluentBuilder {
    /// Creates a new `UpdatePrimaryEmailAddress`.
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
            crate::operation::update_primary_email_address::UpdatePrimaryEmailAddress,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressError,
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
        crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressError,
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
        crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressError,
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
            crate::operation::update_primary_email_address::UpdatePrimaryEmailAddress,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_primary_email_address::UpdatePrimaryEmailAddressError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The organization that contains the user, group, or resource to update.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The organization that contains the user, group, or resource to update.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The user, group, or resource to update.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_id(input.into());
        self
    }
    /// <p>The user, group, or resource to update.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_id(input);
        self
    }
    /// <p>The value of the email to be updated as primary.</p>
    pub fn email(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.email(input.into());
        self
    }
    /// <p>The value of the email to be updated as primary.</p>
    pub fn set_email(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_email(input);
        self
    }
}
