// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_routing_profile::_create_routing_profile_output::CreateRoutingProfileOutputBuilder;

pub use crate::operation::create_routing_profile::_create_routing_profile_input::CreateRoutingProfileInputBuilder;

/// Fluent builder constructing a request to `CreateRoutingProfile`.
///
/// <p>Creates a new routing profile.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRoutingProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_routing_profile::builders::CreateRoutingProfileInputBuilder,
}
impl CreateRoutingProfileFluentBuilder {
    /// Creates a new `CreateRoutingProfile`.
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
            crate::operation::create_routing_profile::CreateRoutingProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_routing_profile::CreateRoutingProfileError,
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
        crate::operation::create_routing_profile::CreateRoutingProfileOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_routing_profile::CreateRoutingProfileError,
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
        crate::operation::create_routing_profile::CreateRoutingProfileOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_routing_profile::CreateRoutingProfileError,
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
            crate::operation::create_routing_profile::CreateRoutingProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_routing_profile::CreateRoutingProfileError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The name of the routing profile. Must not be more than 127 characters.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the routing profile. Must not be more than 127 characters.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>Description of the routing profile. Must not be more than 250 characters.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Description of the routing profile. Must not be more than 250 characters.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The default outbound queue for the routing profile.</p>
    pub fn default_outbound_queue_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.default_outbound_queue_id(input.into());
        self
    }
    /// <p>The default outbound queue for the routing profile.</p>
    pub fn set_default_outbound_queue_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_default_outbound_queue_id(input);
        self
    }
    /// Appends an item to `QueueConfigs`.
    ///
    /// To override the contents of this collection use [`set_queue_configs`](Self::set_queue_configs).
    ///
    /// <p>The inbound queues associated with the routing profile. If no queue is added, the agent can make only outbound calls.</p>
    /// <p>The limit of 10 array members applies to the maximum number of <code>RoutingProfileQueueConfig</code> objects that can be passed during a CreateRoutingProfile API request. It is different from the quota of 50 queues per routing profile per instance that is listed in <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">Amazon Connect service quotas</a>. </p>
    pub fn queue_configs(mut self, input: crate::types::RoutingProfileQueueConfig) -> Self {
        self.inner = self.inner.queue_configs(input);
        self
    }
    /// <p>The inbound queues associated with the routing profile. If no queue is added, the agent can make only outbound calls.</p>
    /// <p>The limit of 10 array members applies to the maximum number of <code>RoutingProfileQueueConfig</code> objects that can be passed during a CreateRoutingProfile API request. It is different from the quota of 50 queues per routing profile per instance that is listed in <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">Amazon Connect service quotas</a>. </p>
    pub fn set_queue_configs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RoutingProfileQueueConfig>>,
    ) -> Self {
        self.inner = self.inner.set_queue_configs(input);
        self
    }
    /// Appends an item to `MediaConcurrencies`.
    ///
    /// To override the contents of this collection use [`set_media_concurrencies`](Self::set_media_concurrencies).
    ///
    /// <p>The channels that agents can handle in the Contact Control Panel (CCP) for this routing profile.</p>
    pub fn media_concurrencies(mut self, input: crate::types::MediaConcurrency) -> Self {
        self.inner = self.inner.media_concurrencies(input);
        self
    }
    /// <p>The channels that agents can handle in the Contact Control Panel (CCP) for this routing profile.</p>
    pub fn set_media_concurrencies(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MediaConcurrency>>,
    ) -> Self {
        self.inner = self.inner.set_media_concurrencies(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
