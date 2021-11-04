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

/// An ergonomic service client for `AWSOrigamiServiceGatewayService`.
///
/// This client allows ergonomic access to a `AWSOrigamiServiceGatewayService`-shaped service.
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
    /// Constructs a fluent builder for the `DeleteReportDefinition` operation.
    ///
    /// See [`DeleteReportDefinition`](crate::client::fluent_builders::DeleteReportDefinition) for more information about the
    /// operation and its arguments.
    pub fn delete_report_definition(&self) -> fluent_builders::DeleteReportDefinition<C, M, R> {
        fluent_builders::DeleteReportDefinition::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DescribeReportDefinitions` operation.
    ///
    /// See [`DescribeReportDefinitions`](crate::client::fluent_builders::DescribeReportDefinitions) for more information about the
    /// operation and its arguments.
    pub fn describe_report_definitions(
        &self,
    ) -> fluent_builders::DescribeReportDefinitions<C, M, R> {
        fluent_builders::DescribeReportDefinitions::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ModifyReportDefinition` operation.
    ///
    /// See [`ModifyReportDefinition`](crate::client::fluent_builders::ModifyReportDefinition) for more information about the
    /// operation and its arguments.
    pub fn modify_report_definition(&self) -> fluent_builders::ModifyReportDefinition<C, M, R> {
        fluent_builders::ModifyReportDefinition::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `PutReportDefinition` operation.
    ///
    /// See [`PutReportDefinition`](crate::client::fluent_builders::PutReportDefinition) for more information about the
    /// operation and its arguments.
    pub fn put_report_definition(&self) -> fluent_builders::PutReportDefinition<C, M, R> {
        fluent_builders::PutReportDefinition::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `DeleteReportDefinition`.
    ///
    /// <p>Deletes the specified report.</p>
    #[derive(std::fmt::Debug)]
    pub struct DeleteReportDefinition<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_report_definition_input::Builder,
    }
    impl<C, M, R> DeleteReportDefinition<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteReportDefinition`.
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
            crate::output::DeleteReportDefinitionOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteReportDefinitionInputOperationOutputAlias,
                crate::output::DeleteReportDefinitionOutput,
                crate::error::DeleteReportDefinitionError,
                crate::input::DeleteReportDefinitionInputOperationRetryAlias,
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
        /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
        pub fn report_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_name(inp);
            self
        }
        /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeReportDefinitions`.
    ///
    /// <p>Lists the AWS Cost and Usage reports available to this account.</p>
    #[derive(std::fmt::Debug)]
    pub struct DescribeReportDefinitions<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_report_definitions_input::Builder,
    }
    impl<C, M, R> DescribeReportDefinitions<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DescribeReportDefinitions`.
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
            crate::output::DescribeReportDefinitionsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeReportDefinitionsInputOperationOutputAlias,
                crate::output::DescribeReportDefinitionsOutput,
                crate::error::DescribeReportDefinitionsError,
                crate::input::DescribeReportDefinitionsInputOperationRetryAlias,
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
        /// <p>The maximum number of results that AWS returns for the operation.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        /// <p>The maximum number of results that AWS returns for the operation.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>A generic string.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>A generic string.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ModifyReportDefinition`.
    ///
    /// <p>Allows you to programatically update your report preferences.</p>
    #[derive(std::fmt::Debug)]
    pub struct ModifyReportDefinition<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::modify_report_definition_input::Builder,
    }
    impl<C, M, R> ModifyReportDefinition<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ModifyReportDefinition`.
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
            crate::output::ModifyReportDefinitionOutput,
            aws_smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ModifyReportDefinitionInputOperationOutputAlias,
                crate::output::ModifyReportDefinitionOutput,
                crate::error::ModifyReportDefinitionError,
                crate::input::ModifyReportDefinitionInputOperationRetryAlias,
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
        /// <p>The name of the report that you want to create. The name must be unique,
        /// is case sensitive, and can't include spaces. </p>
        pub fn report_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_name(inp);
            self
        }
        /// <p>The name of the report that you want to create. The name must be unique,
        /// is case sensitive, and can't include spaces. </p>
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_name(input);
            self
        }
        /// <p>The definition of AWS Cost and Usage Report. You can specify the report name,
        /// time unit, report format, compression format, S3 bucket, additional artifacts, and schema
        /// elements in the definition.
        /// </p>
        pub fn report_definition(mut self, inp: crate::model::ReportDefinition) -> Self {
            self.inner = self.inner.report_definition(inp);
            self
        }
        /// <p>The definition of AWS Cost and Usage Report. You can specify the report name,
        /// time unit, report format, compression format, S3 bucket, additional artifacts, and schema
        /// elements in the definition.
        /// </p>
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.inner = self.inner.set_report_definition(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutReportDefinition`.
    ///
    /// <p>Creates a new report using the description that you provide.</p>
    #[derive(std::fmt::Debug)]
    pub struct PutReportDefinition<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_report_definition_input::Builder,
    }
    impl<C, M, R> PutReportDefinition<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `PutReportDefinition`.
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
            crate::output::PutReportDefinitionOutput,
            aws_smithy_http::result::SdkError<crate::error::PutReportDefinitionError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutReportDefinitionInputOperationOutputAlias,
                crate::output::PutReportDefinitionOutput,
                crate::error::PutReportDefinitionError,
                crate::input::PutReportDefinitionInputOperationRetryAlias,
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
        /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed
        /// metadata and data file information. </p>
        pub fn report_definition(mut self, inp: crate::model::ReportDefinition) -> Self {
            self.inner = self.inner.report_definition(inp);
            self
        }
        /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed
        /// metadata and data file information. </p>
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.inner = self.inner.set_report_definition(input);
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
