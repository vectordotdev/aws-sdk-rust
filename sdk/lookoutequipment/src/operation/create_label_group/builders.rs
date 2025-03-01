// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_label_group::_create_label_group_output::CreateLabelGroupOutputBuilder;

pub use crate::operation::create_label_group::_create_label_group_input::CreateLabelGroupInputBuilder;

/// Fluent builder constructing a request to `CreateLabelGroup`.
///
/// <p> Creates a group of labels. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLabelGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_label_group::builders::CreateLabelGroupInputBuilder,
}
impl CreateLabelGroupFluentBuilder {
    /// Creates a new `CreateLabelGroup`.
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
            crate::operation::create_label_group::CreateLabelGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_label_group::CreateLabelGroupError,
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
        crate::operation::create_label_group::CreateLabelGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_label_group::CreateLabelGroupError,
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
        crate::operation::create_label_group::CreateLabelGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_label_group::CreateLabelGroupError,
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
            crate::operation::create_label_group::CreateLabelGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_label_group::CreateLabelGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p> Names a group of labels.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data. </p>
    pub fn label_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.label_group_name(input.into());
        self
    }
    /// <p> Names a group of labels.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data. </p>
    pub fn set_label_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_label_group_name(input);
        self
    }
    /// Appends an item to `FaultCodes`.
    ///
    /// To override the contents of this collection use [`set_fault_codes`](Self::set_fault_codes).
    ///
    /// <p> The acceptable fault codes (indicating the type of anomaly associated with the label) that can be used with this label group.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn fault_codes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fault_codes(input.into());
        self
    }
    /// <p> The acceptable fault codes (indicating the type of anomaly associated with the label) that can be used with this label group.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn set_fault_codes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_fault_codes(input);
        self
    }
    /// <p> A unique identifier for the request to create a label group. If you do not set the client request token, Lookout for Equipment generates one. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p> A unique identifier for the request to create a label group. If you do not set the client request token, Lookout for Equipment generates one. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p> Tags that provide metadata about the label group you are creating. </p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p> Tags that provide metadata about the label group you are creating. </p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
