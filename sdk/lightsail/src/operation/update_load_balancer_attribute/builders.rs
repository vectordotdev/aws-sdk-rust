// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_load_balancer_attribute::_update_load_balancer_attribute_output::UpdateLoadBalancerAttributeOutputBuilder;

pub use crate::operation::update_load_balancer_attribute::_update_load_balancer_attribute_input::UpdateLoadBalancerAttributeInputBuilder;

/// Fluent builder constructing a request to `UpdateLoadBalancerAttribute`.
///
/// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p>
/// <p>The <code>update load balancer attribute</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLoadBalancerAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_load_balancer_attribute::builders::UpdateLoadBalancerAttributeInputBuilder,
}
impl UpdateLoadBalancerAttributeFluentBuilder {
    /// Creates a new `UpdateLoadBalancerAttribute`.
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
            crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttribute,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttributeError,
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
        crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttributeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttributeError,
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
        crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttributeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttributeError,
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
            crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttribute,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_load_balancer_attribute::UpdateLoadBalancerAttributeError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the load balancer that you want to modify (e.g., <code>my-load-balancer</code>.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.load_balancer_name(input.into());
        self
    }
    /// <p>The name of the load balancer that you want to modify (e.g., <code>my-load-balancer</code>.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_name(input);
        self
    }
    /// <p>The name of the attribute you want to update.</p>
    pub fn attribute_name(mut self, input: crate::types::LoadBalancerAttributeName) -> Self {
        self.inner = self.inner.attribute_name(input);
        self
    }
    /// <p>The name of the attribute you want to update.</p>
    pub fn set_attribute_name(
        mut self,
        input: ::std::option::Option<crate::types::LoadBalancerAttributeName>,
    ) -> Self {
        self.inner = self.inner.set_attribute_name(input);
        self
    }
    /// <p>The value that you want to specify for the attribute name.</p>
    /// <p>The following values are supported depending on what you specify for the <code>attributeName</code> request parameter:</p>
    /// <ul>
    /// <li> <p>If you specify <code>HealthCheckPath</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be the path to ping on the target (for example, <code>/weather/us/wa/seattle</code>).</p> </li>
    /// <li> <p>If you specify <code>SessionStickinessEnabled</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be <code>true</code> to activate session stickiness or <code>false</code> to deactivate session stickiness.</p> </li>
    /// <li> <p>If you specify <code>SessionStickiness_LB_CookieDurationSeconds</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be an interger that represents the cookie duration in seconds.</p> </li>
    /// <li> <p>If you specify <code>HttpsRedirectionEnabled</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be <code>true</code> to activate HTTP to HTTPS redirection or <code>false</code> to deactivate HTTP to HTTPS redirection.</p> </li>
    /// <li> <p>If you specify <code>TlsPolicyName</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be the name of the TLS policy.</p> <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetLoadBalancerTlsPolicies.html">GetLoadBalancerTlsPolicies</a> action to get a list of TLS policy names that you can specify.</p> </li>
    /// </ul>
    pub fn attribute_value(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.attribute_value(input.into());
        self
    }
    /// <p>The value that you want to specify for the attribute name.</p>
    /// <p>The following values are supported depending on what you specify for the <code>attributeName</code> request parameter:</p>
    /// <ul>
    /// <li> <p>If you specify <code>HealthCheckPath</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be the path to ping on the target (for example, <code>/weather/us/wa/seattle</code>).</p> </li>
    /// <li> <p>If you specify <code>SessionStickinessEnabled</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be <code>true</code> to activate session stickiness or <code>false</code> to deactivate session stickiness.</p> </li>
    /// <li> <p>If you specify <code>SessionStickiness_LB_CookieDurationSeconds</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be an interger that represents the cookie duration in seconds.</p> </li>
    /// <li> <p>If you specify <code>HttpsRedirectionEnabled</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be <code>true</code> to activate HTTP to HTTPS redirection or <code>false</code> to deactivate HTTP to HTTPS redirection.</p> </li>
    /// <li> <p>If you specify <code>TlsPolicyName</code> for the <code>attributeName</code> request parameter, then the <code>attributeValue</code> request parameter must be the name of the TLS policy.</p> <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetLoadBalancerTlsPolicies.html">GetLoadBalancerTlsPolicies</a> action to get a list of TLS policy names that you can specify.</p> </li>
    /// </ul>
    pub fn set_attribute_value(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_attribute_value(input);
        self
    }
}
