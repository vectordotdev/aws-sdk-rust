// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_delegate_from_resource::_disassociate_delegate_from_resource_output::DisassociateDelegateFromResourceOutputBuilder;

pub use crate::operation::disassociate_delegate_from_resource::_disassociate_delegate_from_resource_input::DisassociateDelegateFromResourceInputBuilder;

/// Fluent builder constructing a request to `DisassociateDelegateFromResource`.
///
/// <p>Removes a member from the resource's set of delegates.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateDelegateFromResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::disassociate_delegate_from_resource::builders::DisassociateDelegateFromResourceInputBuilder,
}
impl DisassociateDelegateFromResourceFluentBuilder {
    /// Creates a new `DisassociateDelegateFromResource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResource, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResourceError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResourceOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResourceError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResourceOutput, ::aws_smithy_http::result::SdkError<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResourceError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResource, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::disassociate_delegate_from_resource::DisassociateDelegateFromResourceError>
    >{
        self.customize_middleware().await
    }
    /// <p>The identifier for the organization under which the resource exists.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The identifier for the organization under which the resource exists.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The identifier of the resource from which delegates' set members are removed. </p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The identifier of the resource from which delegates' set members are removed. </p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The identifier for the member (user, group) to be removed from the resource's delegates.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_id(input.into());
        self
    }
    /// <p>The identifier for the member (user, group) to be removed from the resource's delegates.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_id(input);
        self
    }
}
