// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    client: aws_smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AWSHabaneroManagementService`.
///
/// This client allows ergonomic access to a `AWSHabaneroManagementService`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `CreateEnvironment` operation.
    ///
    /// See [`CreateEnvironment`](crate::client::fluent_builders::CreateEnvironment) for more information about the
    /// operation and its arguments.
    pub fn create_environment(&self) -> fluent_builders::CreateEnvironment<C, M, R> {
        fluent_builders::CreateEnvironment::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DeleteEnvironment` operation.
    ///
    /// See [`DeleteEnvironment`](crate::client::fluent_builders::DeleteEnvironment) for more information about the
    /// operation and its arguments.
    pub fn delete_environment(&self) -> fluent_builders::DeleteEnvironment<C, M, R> {
        fluent_builders::DeleteEnvironment::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetEnvironment` operation.
    ///
    /// See [`GetEnvironment`](crate::client::fluent_builders::GetEnvironment) for more information about the
    /// operation and its arguments.
    pub fn get_environment(&self) -> fluent_builders::GetEnvironment<C, M, R> {
        fluent_builders::GetEnvironment::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListEnvironments` operation.
    ///
    /// See [`ListEnvironments`](crate::client::fluent_builders::ListEnvironments) for more information about the
    /// operation and its arguments.
    pub fn list_environments(&self) -> fluent_builders::ListEnvironments<C, M, R> {
        fluent_builders::ListEnvironments::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListTagsForResource` operation.
    ///
    /// See [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) for more information about the
    /// operation and its arguments.
    pub fn list_tags_for_resource(&self) -> fluent_builders::ListTagsForResource<C, M, R> {
        fluent_builders::ListTagsForResource::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `TagResource` operation.
    ///
    /// See [`TagResource`](crate::client::fluent_builders::TagResource) for more information about the
    /// operation and its arguments.
    pub fn tag_resource(&self) -> fluent_builders::TagResource<C, M, R> {
        fluent_builders::TagResource::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `UntagResource` operation.
    ///
    /// See [`UntagResource`](crate::client::fluent_builders::UntagResource) for more information about the
    /// operation and its arguments.
    pub fn untag_resource(&self) -> fluent_builders::UntagResource<C, M, R> {
        fluent_builders::UntagResource::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `UpdateEnvironment` operation.
    ///
    /// See [`UpdateEnvironment`](crate::client::fluent_builders::UpdateEnvironment) for more information about the
    /// operation and its arguments.
    pub fn update_environment(&self) -> fluent_builders::UpdateEnvironment<C, M, R> {
        fluent_builders::UpdateEnvironment::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `CreateEnvironment`.
    ///
    /// <p>Create a new FinSpace environment.</p>
    #[derive(std::fmt::Debug)]
    pub struct CreateEnvironment<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_environment_input::Builder,
    }
    impl<C, M, R> CreateEnvironment<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `CreateEnvironment`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::CreateEnvironmentOutput,
            aws_smithy_http::result::SdkError<crate::error::CreateEnvironmentError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateEnvironmentInputOperationOutputAlias,
                crate::output::CreateEnvironmentOutput,
                crate::error::CreateEnvironmentError,
                crate::input::CreateEnvironmentInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the FinSpace environment to be created.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        /// <p>The name of the FinSpace environment to be created.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>The description of the FinSpace environment to be created.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        /// <p>The description of the FinSpace environment to be created.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>The KMS key id to encrypt your data in the FinSpace environment.</p>
        pub fn kms_key_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.kms_key_id(inp);
            self
        }
        /// <p>The KMS key id to encrypt your data in the FinSpace environment.</p>
        pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_kms_key_id(input);
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Add tags to your FinSpace environment.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        /// <p>Add tags to your FinSpace environment.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
        /// <p>Authentication mode for the environment.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>FEDERATED</code> - Users access FinSpace through Single Sign On (SSO) via your Identity provider.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>LOCAL</code> - Users access FinSpace via email and password managed within the FinSpace environment.</p>
        /// </li>
        /// </ul>
        pub fn federation_mode(mut self, inp: crate::model::FederationMode) -> Self {
            self.inner = self.inner.federation_mode(inp);
            self
        }
        /// <p>Authentication mode for the environment.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>FEDERATED</code> - Users access FinSpace through Single Sign On (SSO) via your Identity provider.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>LOCAL</code> - Users access FinSpace via email and password managed within the FinSpace environment.</p>
        /// </li>
        /// </ul>
        pub fn set_federation_mode(
            mut self,
            input: std::option::Option<crate::model::FederationMode>,
        ) -> Self {
            self.inner = self.inner.set_federation_mode(input);
            self
        }
        /// <p>Configuration information when authentication mode is FEDERATED.</p>
        pub fn federation_parameters(mut self, inp: crate::model::FederationParameters) -> Self {
            self.inner = self.inner.federation_parameters(inp);
            self
        }
        /// <p>Configuration information when authentication mode is FEDERATED.</p>
        pub fn set_federation_parameters(
            mut self,
            input: std::option::Option<crate::model::FederationParameters>,
        ) -> Self {
            self.inner = self.inner.set_federation_parameters(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DeleteEnvironment`.
    ///
    /// <p>Delete an FinSpace environment.</p>
    #[derive(std::fmt::Debug)]
    pub struct DeleteEnvironment<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_environment_input::Builder,
    }
    impl<C, M, R> DeleteEnvironment<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteEnvironment`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::DeleteEnvironmentOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteEnvironmentError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteEnvironmentInputOperationOutputAlias,
                crate::output::DeleteEnvironmentOutput,
                crate::error::DeleteEnvironmentError,
                crate::input::DeleteEnvironmentInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The identifier for the FinSpace environment.</p>
        pub fn environment_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_id(inp);
            self
        }
        /// <p>The identifier for the FinSpace environment.</p>
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetEnvironment`.
    ///
    /// <p>Returns the FinSpace environment object.</p>
    #[derive(std::fmt::Debug)]
    pub struct GetEnvironment<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_environment_input::Builder,
    }
    impl<C, M, R> GetEnvironment<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetEnvironment`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::GetEnvironmentOutput,
            aws_smithy_http::result::SdkError<crate::error::GetEnvironmentError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetEnvironmentInputOperationOutputAlias,
                crate::output::GetEnvironmentOutput,
                crate::error::GetEnvironmentError,
                crate::input::GetEnvironmentInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The identifier of the FinSpace environment.</p>
        pub fn environment_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_id(inp);
            self
        }
        /// <p>The identifier of the FinSpace environment.</p>
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListEnvironments`.
    ///
    /// <p>A list of all of your FinSpace environments.</p>
    #[derive(std::fmt::Debug)]
    pub struct ListEnvironments<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_environments_input::Builder,
    }
    impl<C, M, R> ListEnvironments<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListEnvironments`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::ListEnvironmentsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListEnvironmentsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListEnvironmentsInputOperationOutputAlias,
                crate::output::ListEnvironmentsOutput,
                crate::error::ListEnvironmentsError,
                crate::input::ListEnvironmentsInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>A token generated by FinSpace that specifies where to continue pagination if a previous
        /// request was truncated. To get the next set of pages, pass in the nextToken value from the
        /// response object of the previous page call.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>A token generated by FinSpace that specifies where to continue pagination if a previous
        /// request was truncated. To get the next set of pages, pass in the nextToken value from the
        /// response object of the previous page call.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return in this request.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        /// <p>The maximum number of results to return in this request.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListTagsForResource`.
    ///
    /// <p>A list of all tags for a resource.</p>
    #[derive(std::fmt::Debug)]
    pub struct ListTagsForResource<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_tags_for_resource_input::Builder,
    }
    impl<C, M, R> ListTagsForResource<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListTagsForResource`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::ListTagsForResourceOutput,
            aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListTagsForResourceInputOperationOutputAlias,
                crate::output::ListTagsForResourceOutput,
                crate::error::ListTagsForResourceError,
                crate::input::ListTagsForResourceInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name of the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        /// <p>The Amazon Resource Name of the resource.</p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
    }
    /// Fluent builder constructing a request to `TagResource`.
    ///
    /// <p>Adds metadata tags to a FinSpace resource.</p>
    #[derive(std::fmt::Debug)]
    pub struct TagResource<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::tag_resource_input::Builder,
    }
    impl<C, M, R> TagResource<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `TagResource`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::TagResourceOutput,
            aws_smithy_http::result::SdkError<crate::error::TagResourceError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::TagResourceInputOperationOutputAlias,
                crate::output::TagResourceOutput,
                crate::error::TagResourceError,
                crate::input::TagResourceInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) for the resource.</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        /// <p>The Amazon Resource Name (ARN) for the resource.</p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>One or more tags to be assigned to the resource.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.tags(k, v);
            self
        }
        /// <p>One or more tags to be assigned to the resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.inner = self.inner.set_tags(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UntagResource`.
    ///
    /// <p>Removes metadata tags from a FinSpace resource.</p>
    #[derive(std::fmt::Debug)]
    pub struct UntagResource<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::untag_resource_input::Builder,
    }
    impl<C, M, R> UntagResource<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UntagResource`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::UntagResourceOutput,
            aws_smithy_http::result::SdkError<crate::error::UntagResourceError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UntagResourceInputOperationOutputAlias,
                crate::output::UntagResourceOutput,
                crate::error::UntagResourceError,
                crate::input::UntagResourceInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>A FinSpace resource from which you want to remove a tag or tags. The value for this
        /// parameter is an Amazon Resource Name (ARN).</p>
        pub fn resource_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.resource_arn(inp);
            self
        }
        /// <p>A FinSpace resource from which you want to remove a tag or tags. The value for this
        /// parameter is an Amazon Resource Name (ARN).</p>
        pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_resource_arn(input);
            self
        }
        /// Appends an item to `tagKeys`.
        ///
        /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
        ///
        /// <p>The tag keys (names) of one or more tags to be removed.</p>
        pub fn tag_keys(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tag_keys(inp);
            self
        }
        /// <p>The tag keys (names) of one or more tags to be removed.</p>
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_tag_keys(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateEnvironment`.
    ///
    /// <p>Update your FinSpace environment.</p>
    #[derive(std::fmt::Debug)]
    pub struct UpdateEnvironment<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_environment_input::Builder,
    }
    impl<C, M, R> UpdateEnvironment<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UpdateEnvironment`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::UpdateEnvironmentOutput,
            aws_smithy_http::result::SdkError<crate::error::UpdateEnvironmentError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateEnvironmentInputOperationOutputAlias,
                crate::output::UpdateEnvironmentOutput,
                crate::error::UpdateEnvironmentError,
                crate::input::UpdateEnvironmentInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The identifier of the FinSpace environment.</p>
        pub fn environment_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_id(inp);
            self
        }
        /// <p>The identifier of the FinSpace environment.</p>
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_id(input);
            self
        }
        /// <p>The name of the environment.</p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        /// <p>The name of the environment.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>The description of the environment.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.description(inp);
            self
        }
        /// <p>The description of the environment.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_description(input);
            self
        }
        /// <p>Authentication mode for the environment.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>FEDERATED</code> - Users access FinSpace through Single Sign On (SSO) via your Identity provider.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>LOCAL</code> - Users access FinSpace via email and password managed within the FinSpace environment.</p>
        /// </li>
        /// </ul>
        pub fn federation_mode(mut self, inp: crate::model::FederationMode) -> Self {
            self.inner = self.inner.federation_mode(inp);
            self
        }
        /// <p>Authentication mode for the environment.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>FEDERATED</code> - Users access FinSpace through Single Sign On (SSO) via your Identity provider.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>LOCAL</code> - Users access FinSpace via email and password managed within the FinSpace environment.</p>
        /// </li>
        /// </ul>
        pub fn set_federation_mode(
            mut self,
            input: std::option::Option<crate::model::FederationMode>,
        ) -> Self {
            self.inner = self.inner.set_federation_mode(input);
            self
        }
        /// <p>Configuration information when authentication mode is FEDERATED.</p>
        pub fn federation_parameters(mut self, inp: crate::model::FederationParameters) -> Self {
            self.inner = self.inner.federation_parameters(inp);
            self
        }
        /// <p>Configuration information when authentication mode is FEDERATED.</p>
        pub fn set_federation_parameters(
            mut self,
            input: std::option::Option<crate::model::FederationParameters>,
        ) -> Self {
            self.inner = self.inner.set_federation_parameters(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::new(conn).with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let client = aws_hyper::Client::https().with_retry_config(retry_config.into());
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
