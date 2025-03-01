// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reset_cluster_parameter_group::_reset_cluster_parameter_group_output::ResetClusterParameterGroupOutputBuilder;

pub use crate::operation::reset_cluster_parameter_group::_reset_cluster_parameter_group_input::ResetClusterParameterGroupInputBuilder;

/// Fluent builder constructing a request to `ResetClusterParameterGroup`.
///
/// <p>Sets one or more parameters of the specified parameter group to their default values and sets the source values of the parameters to "engine-default". To reset the entire parameter group specify the <i>ResetAllParameters</i> parameter. For parameter changes to take effect you must reboot any associated clusters. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ResetClusterParameterGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::reset_cluster_parameter_group::builders::ResetClusterParameterGroupInputBuilder,
}
impl ResetClusterParameterGroupFluentBuilder {
    /// Creates a new `ResetClusterParameterGroup`.
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
            crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroupError,
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
        crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroupError,
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
        crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroupError,
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
            crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reset_cluster_parameter_group::ResetClusterParameterGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the cluster parameter group to be reset.</p>
    pub fn parameter_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.parameter_group_name(input.into());
        self
    }
    /// <p>The name of the cluster parameter group to be reset.</p>
    pub fn set_parameter_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_parameter_group_name(input);
        self
    }
    /// <p>If <code>true</code>, all parameters in the specified parameter group will be reset to their default values. </p>
    /// <p>Default: <code>true</code> </p>
    pub fn reset_all_parameters(mut self, input: bool) -> Self {
        self.inner = self.inner.reset_all_parameters(input);
        self
    }
    /// <p>If <code>true</code>, all parameters in the specified parameter group will be reset to their default values. </p>
    /// <p>Default: <code>true</code> </p>
    pub fn set_reset_all_parameters(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_reset_all_parameters(input);
        self
    }
    /// Appends an item to `Parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>An array of names of parameters to be reset. If <i>ResetAllParameters</i> option is not used, then at least one parameter name must be supplied. </p>
    /// <p>Constraints: A maximum of 20 parameters can be reset in a single request.</p>
    pub fn parameters(mut self, input: crate::types::Parameter) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>An array of names of parameters to be reset. If <i>ResetAllParameters</i> option is not used, then at least one parameter name must be supplied. </p>
    /// <p>Constraints: A maximum of 20 parameters can be reset in a single request.</p>
    pub fn set_parameters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Parameter>>,
    ) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
}
