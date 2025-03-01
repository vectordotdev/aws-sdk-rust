// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a data source.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataSource {
    /// <p>The data source Amazon Resource Name (ARN).</p>
    #[doc(hidden)]
    pub data_source_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the data source.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the data source.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The type of the data source.</p>
    /// <ul>
    /// <li> <p> <b>AWS_LAMBDA</b>: The data source is an Lambda function.</p> </li>
    /// <li> <p> <b>AMAZON_DYNAMODB</b>: The data source is an Amazon DynamoDB table.</p> </li>
    /// <li> <p> <b>AMAZON_ELASTICSEARCH</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_OPENSEARCH_SERVICE</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_EVENTBRIDGE</b>: The data source is an Amazon EventBridge configuration.</p> </li>
    /// <li> <p> <b>NONE</b>: There is no data source. Use this type when you want to invoke a GraphQL operation without connecting to a data source, such as when you're performing data transformation with resolvers or invoking a subscription from a mutation.</p> </li>
    /// <li> <p> <b>HTTP</b>: The data source is an HTTP endpoint.</p> </li>
    /// <li> <p> <b>RELATIONAL_DATABASE</b>: The data source is a relational database.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::DataSourceType>,
    /// <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN) for the data source. The system assumes this role when accessing the data source.</p>
    #[doc(hidden)]
    pub service_role_arn: ::std::option::Option<::std::string::String>,
    /// <p>DynamoDB settings.</p>
    #[doc(hidden)]
    pub dynamodb_config: ::std::option::Option<crate::types::DynamodbDataSourceConfig>,
    /// <p>Lambda settings.</p>
    #[doc(hidden)]
    pub lambda_config: ::std::option::Option<crate::types::LambdaDataSourceConfig>,
    /// <p>Amazon OpenSearch Service settings.</p>
    #[doc(hidden)]
    pub elasticsearch_config: ::std::option::Option<crate::types::ElasticsearchDataSourceConfig>,
    /// <p>Amazon OpenSearch Service settings.</p>
    #[doc(hidden)]
    pub open_search_service_config:
        ::std::option::Option<crate::types::OpenSearchServiceDataSourceConfig>,
    /// <p>HTTP endpoint settings.</p>
    #[doc(hidden)]
    pub http_config: ::std::option::Option<crate::types::HttpDataSourceConfig>,
    /// <p>Relational database settings.</p>
    #[doc(hidden)]
    pub relational_database_config:
        ::std::option::Option<crate::types::RelationalDatabaseDataSourceConfig>,
    /// <p>Amazon EventBridge settings.</p>
    #[doc(hidden)]
    pub event_bridge_config: ::std::option::Option<crate::types::EventBridgeDataSourceConfig>,
}
impl DataSource {
    /// <p>The data source Amazon Resource Name (ARN).</p>
    pub fn data_source_arn(&self) -> ::std::option::Option<&str> {
        self.data_source_arn.as_deref()
    }
    /// <p>The name of the data source.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the data source.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The type of the data source.</p>
    /// <ul>
    /// <li> <p> <b>AWS_LAMBDA</b>: The data source is an Lambda function.</p> </li>
    /// <li> <p> <b>AMAZON_DYNAMODB</b>: The data source is an Amazon DynamoDB table.</p> </li>
    /// <li> <p> <b>AMAZON_ELASTICSEARCH</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_OPENSEARCH_SERVICE</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_EVENTBRIDGE</b>: The data source is an Amazon EventBridge configuration.</p> </li>
    /// <li> <p> <b>NONE</b>: There is no data source. Use this type when you want to invoke a GraphQL operation without connecting to a data source, such as when you're performing data transformation with resolvers or invoking a subscription from a mutation.</p> </li>
    /// <li> <p> <b>HTTP</b>: The data source is an HTTP endpoint.</p> </li>
    /// <li> <p> <b>RELATIONAL_DATABASE</b>: The data source is a relational database.</p> </li>
    /// </ul>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::DataSourceType> {
        self.r#type.as_ref()
    }
    /// <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN) for the data source. The system assumes this role when accessing the data source.</p>
    pub fn service_role_arn(&self) -> ::std::option::Option<&str> {
        self.service_role_arn.as_deref()
    }
    /// <p>DynamoDB settings.</p>
    pub fn dynamodb_config(
        &self,
    ) -> ::std::option::Option<&crate::types::DynamodbDataSourceConfig> {
        self.dynamodb_config.as_ref()
    }
    /// <p>Lambda settings.</p>
    pub fn lambda_config(&self) -> ::std::option::Option<&crate::types::LambdaDataSourceConfig> {
        self.lambda_config.as_ref()
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn elasticsearch_config(
        &self,
    ) -> ::std::option::Option<&crate::types::ElasticsearchDataSourceConfig> {
        self.elasticsearch_config.as_ref()
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn open_search_service_config(
        &self,
    ) -> ::std::option::Option<&crate::types::OpenSearchServiceDataSourceConfig> {
        self.open_search_service_config.as_ref()
    }
    /// <p>HTTP endpoint settings.</p>
    pub fn http_config(&self) -> ::std::option::Option<&crate::types::HttpDataSourceConfig> {
        self.http_config.as_ref()
    }
    /// <p>Relational database settings.</p>
    pub fn relational_database_config(
        &self,
    ) -> ::std::option::Option<&crate::types::RelationalDatabaseDataSourceConfig> {
        self.relational_database_config.as_ref()
    }
    /// <p>Amazon EventBridge settings.</p>
    pub fn event_bridge_config(
        &self,
    ) -> ::std::option::Option<&crate::types::EventBridgeDataSourceConfig> {
        self.event_bridge_config.as_ref()
    }
}
impl DataSource {
    /// Creates a new builder-style object to manufacture [`DataSource`](crate::types::DataSource).
    pub fn builder() -> crate::types::builders::DataSourceBuilder {
        crate::types::builders::DataSourceBuilder::default()
    }
}

/// A builder for [`DataSource`](crate::types::DataSource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DataSourceBuilder {
    pub(crate) data_source_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::DataSourceType>,
    pub(crate) service_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) dynamodb_config: ::std::option::Option<crate::types::DynamodbDataSourceConfig>,
    pub(crate) lambda_config: ::std::option::Option<crate::types::LambdaDataSourceConfig>,
    pub(crate) elasticsearch_config:
        ::std::option::Option<crate::types::ElasticsearchDataSourceConfig>,
    pub(crate) open_search_service_config:
        ::std::option::Option<crate::types::OpenSearchServiceDataSourceConfig>,
    pub(crate) http_config: ::std::option::Option<crate::types::HttpDataSourceConfig>,
    pub(crate) relational_database_config:
        ::std::option::Option<crate::types::RelationalDatabaseDataSourceConfig>,
    pub(crate) event_bridge_config:
        ::std::option::Option<crate::types::EventBridgeDataSourceConfig>,
}
impl DataSourceBuilder {
    /// <p>The data source Amazon Resource Name (ARN).</p>
    pub fn data_source_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.data_source_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The data source Amazon Resource Name (ARN).</p>
    pub fn set_data_source_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.data_source_arn = input;
        self
    }
    /// <p>The name of the data source.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the data source.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the data source.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the data source.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The type of the data source.</p>
    /// <ul>
    /// <li> <p> <b>AWS_LAMBDA</b>: The data source is an Lambda function.</p> </li>
    /// <li> <p> <b>AMAZON_DYNAMODB</b>: The data source is an Amazon DynamoDB table.</p> </li>
    /// <li> <p> <b>AMAZON_ELASTICSEARCH</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_OPENSEARCH_SERVICE</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_EVENTBRIDGE</b>: The data source is an Amazon EventBridge configuration.</p> </li>
    /// <li> <p> <b>NONE</b>: There is no data source. Use this type when you want to invoke a GraphQL operation without connecting to a data source, such as when you're performing data transformation with resolvers or invoking a subscription from a mutation.</p> </li>
    /// <li> <p> <b>HTTP</b>: The data source is an HTTP endpoint.</p> </li>
    /// <li> <p> <b>RELATIONAL_DATABASE</b>: The data source is a relational database.</p> </li>
    /// </ul>
    pub fn r#type(mut self, input: crate::types::DataSourceType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the data source.</p>
    /// <ul>
    /// <li> <p> <b>AWS_LAMBDA</b>: The data source is an Lambda function.</p> </li>
    /// <li> <p> <b>AMAZON_DYNAMODB</b>: The data source is an Amazon DynamoDB table.</p> </li>
    /// <li> <p> <b>AMAZON_ELASTICSEARCH</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_OPENSEARCH_SERVICE</b>: The data source is an Amazon OpenSearch Service domain.</p> </li>
    /// <li> <p> <b>AMAZON_EVENTBRIDGE</b>: The data source is an Amazon EventBridge configuration.</p> </li>
    /// <li> <p> <b>NONE</b>: There is no data source. Use this type when you want to invoke a GraphQL operation without connecting to a data source, such as when you're performing data transformation with resolvers or invoking a subscription from a mutation.</p> </li>
    /// <li> <p> <b>HTTP</b>: The data source is an HTTP endpoint.</p> </li>
    /// <li> <p> <b>RELATIONAL_DATABASE</b>: The data source is a relational database.</p> </li>
    /// </ul>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::DataSourceType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN) for the data source. The system assumes this role when accessing the data source.</p>
    pub fn service_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.service_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN) for the data source. The system assumes this role when accessing the data source.</p>
    pub fn set_service_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.service_role_arn = input;
        self
    }
    /// <p>DynamoDB settings.</p>
    pub fn dynamodb_config(mut self, input: crate::types::DynamodbDataSourceConfig) -> Self {
        self.dynamodb_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>DynamoDB settings.</p>
    pub fn set_dynamodb_config(
        mut self,
        input: ::std::option::Option<crate::types::DynamodbDataSourceConfig>,
    ) -> Self {
        self.dynamodb_config = input;
        self
    }
    /// <p>Lambda settings.</p>
    pub fn lambda_config(mut self, input: crate::types::LambdaDataSourceConfig) -> Self {
        self.lambda_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Lambda settings.</p>
    pub fn set_lambda_config(
        mut self,
        input: ::std::option::Option<crate::types::LambdaDataSourceConfig>,
    ) -> Self {
        self.lambda_config = input;
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn elasticsearch_config(
        mut self,
        input: crate::types::ElasticsearchDataSourceConfig,
    ) -> Self {
        self.elasticsearch_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn set_elasticsearch_config(
        mut self,
        input: ::std::option::Option<crate::types::ElasticsearchDataSourceConfig>,
    ) -> Self {
        self.elasticsearch_config = input;
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn open_search_service_config(
        mut self,
        input: crate::types::OpenSearchServiceDataSourceConfig,
    ) -> Self {
        self.open_search_service_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amazon OpenSearch Service settings.</p>
    pub fn set_open_search_service_config(
        mut self,
        input: ::std::option::Option<crate::types::OpenSearchServiceDataSourceConfig>,
    ) -> Self {
        self.open_search_service_config = input;
        self
    }
    /// <p>HTTP endpoint settings.</p>
    pub fn http_config(mut self, input: crate::types::HttpDataSourceConfig) -> Self {
        self.http_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>HTTP endpoint settings.</p>
    pub fn set_http_config(
        mut self,
        input: ::std::option::Option<crate::types::HttpDataSourceConfig>,
    ) -> Self {
        self.http_config = input;
        self
    }
    /// <p>Relational database settings.</p>
    pub fn relational_database_config(
        mut self,
        input: crate::types::RelationalDatabaseDataSourceConfig,
    ) -> Self {
        self.relational_database_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Relational database settings.</p>
    pub fn set_relational_database_config(
        mut self,
        input: ::std::option::Option<crate::types::RelationalDatabaseDataSourceConfig>,
    ) -> Self {
        self.relational_database_config = input;
        self
    }
    /// <p>Amazon EventBridge settings.</p>
    pub fn event_bridge_config(mut self, input: crate::types::EventBridgeDataSourceConfig) -> Self {
        self.event_bridge_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Amazon EventBridge settings.</p>
    pub fn set_event_bridge_config(
        mut self,
        input: ::std::option::Option<crate::types::EventBridgeDataSourceConfig>,
    ) -> Self {
        self.event_bridge_config = input;
        self
    }
    /// Consumes the builder and constructs a [`DataSource`](crate::types::DataSource).
    pub fn build(self) -> crate::types::DataSource {
        crate::types::DataSource {
            data_source_arn: self.data_source_arn,
            name: self.name,
            description: self.description,
            r#type: self.r#type,
            service_role_arn: self.service_role_arn,
            dynamodb_config: self.dynamodb_config,
            lambda_config: self.lambda_config,
            elasticsearch_config: self.elasticsearch_config,
            open_search_service_config: self.open_search_service_config,
            http_config: self.http_config,
            relational_database_config: self.relational_database_config,
            event_bridge_config: self.event_bridge_config,
        }
    }
}
