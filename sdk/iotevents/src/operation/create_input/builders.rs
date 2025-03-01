// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_input::_create_input_output::CreateInputOutputBuilder;

pub use crate::operation::create_input::_create_input_input::CreateInputInputBuilder;

/// Fluent builder constructing a request to `CreateInput`.
///
/// <p>Creates an input.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateInputFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_input::builders::CreateInputInputBuilder,
}
impl CreateInputFluentBuilder {
    /// Creates a new `CreateInput`.
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
            crate::operation::create_input::CreateInput,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_input::CreateInputError>,
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
        crate::operation::create_input::CreateInputOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_input::CreateInputError>,
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
        crate::operation::create_input::CreateInputOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_input::CreateInputError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_input::CreateInput,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_input::CreateInputError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name you want to give to the input.</p>
    pub fn input_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_name(input.into());
        self
    }
    /// <p>The name you want to give to the input.</p>
    pub fn set_input_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_name(input);
        self
    }
    /// <p>A brief description of the input.</p>
    pub fn input_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.input_description(input.into());
        self
    }
    /// <p>A brief description of the input.</p>
    pub fn set_input_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_input_description(input);
        self
    }
    /// <p>The definition of the input.</p>
    pub fn input_definition(mut self, input: crate::types::InputDefinition) -> Self {
        self.inner = self.inner.input_definition(input);
        self
    }
    /// <p>The definition of the input.</p>
    pub fn set_input_definition(
        mut self,
        input: ::std::option::Option<crate::types::InputDefinition>,
    ) -> Self {
        self.inner = self.inner.set_input_definition(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata that can be used to manage the input.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata that can be used to manage the input.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
