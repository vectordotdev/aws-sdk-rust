// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_datalake::_create_datalake_output::CreateDatalakeOutputBuilder;

pub use crate::operation::create_datalake::_create_datalake_input::CreateDatalakeInputBuilder;

/// Fluent builder constructing a request to `CreateDatalake`.
///
/// <p>Initializes an Amazon Security Lake instance with the provided (or default) configuration. You can enable Security Lake in Amazon Web Services Regions with customized settings before enabling log collection in Regions. You can either use the <code>enableAll</code> parameter to specify all Regions or specify the Regions where you want to enable Security Lake. To specify particular Regions, use the <code>Regions</code> parameter and then configure these Regions using the <code>configurations</code> parameter. If you have already enabled Security Lake in a Region when you call this command, the command will update the Region if you provide new configuration parameters. If you have not already enabled Security Lake in the Region when you call this API, it will set up the data lake in the Region with the specified configurations.</p>
/// <p>When you enable Security Lake, it starts ingesting security data after the <code>CreateAwsLogSource</code> call. This includes ingesting security data from sources, storing data, and making data accessible to subscribers. Security Lake also enables all the existing settings and resources that it stores or maintains for your Amazon Web Services account in the current Region, including security log and event data. For more information, see the <a href="https://docs.aws.amazon.com/security-lake/latest/userguide/what-is-security-lake.html">Amazon Security Lake User Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDatalakeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_datalake::builders::CreateDatalakeInputBuilder,
}
impl CreateDatalakeFluentBuilder {
    /// Creates a new `CreateDatalake`.
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
            crate::operation::create_datalake::CreateDatalake,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_datalake::CreateDatalakeError>,
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
        crate::operation::create_datalake::CreateDatalakeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_datalake::CreateDatalakeError>,
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
        crate::operation::create_datalake::CreateDatalakeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_datalake::CreateDatalakeError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_datalake::CreateDatalake,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_datalake::CreateDatalakeError>,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `regions`.
    ///
    /// To override the contents of this collection use [`set_regions`](Self::set_regions).
    ///
    /// <p>Enable Security Lake in the specified Regions. To enable Security Lake in specific Amazon Web Services Regions, such as us-east-1 or ap-northeast-3, provide the Region codes. For a list of Region codes, see <a href="https://docs.aws.amazon.com/general/latest/gr/securitylake.html">Amazon Security Lake endpoints</a> in the Amazon Web Services General Reference.</p>
    pub fn regions(mut self, input: crate::types::Region) -> Self {
        self.inner = self.inner.regions(input);
        self
    }
    /// <p>Enable Security Lake in the specified Regions. To enable Security Lake in specific Amazon Web Services Regions, such as us-east-1 or ap-northeast-3, provide the Region codes. For a list of Region codes, see <a href="https://docs.aws.amazon.com/general/latest/gr/securitylake.html">Amazon Security Lake endpoints</a> in the Amazon Web Services General Reference.</p>
    pub fn set_regions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Region>>,
    ) -> Self {
        self.inner = self.inner.set_regions(input);
        self
    }
    /// Adds a key-value pair to `configurations`.
    ///
    /// To override the contents of this collection use [`set_configurations`](Self::set_configurations).
    ///
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn configurations(
        mut self,
        k: crate::types::Region,
        v: crate::types::LakeConfigurationRequest,
    ) -> Self {
        self.inner = self.inner.configurations(k, v);
        self
    }
    /// <p>Specify the Region or Regions that will contribute data to the rollup region.</p>
    pub fn set_configurations(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                crate::types::Region,
                crate::types::LakeConfigurationRequest,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.set_configurations(input);
        self
    }
    /// <p>Enable Security Lake in all Regions.</p>
    pub fn enable_all(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_all(input);
        self
    }
    /// <p>Enable Security Lake in all Regions.</p>
    pub fn set_enable_all(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_all(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) used to create and update the Glue table. This table contains partitions generated by the ingestion and normalization of Amazon Web Services log sources and custom sources.</p>
    pub fn meta_store_manager_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.meta_store_manager_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) used to create and update the Glue table. This table contains partitions generated by the ingestion and normalization of Amazon Web Services log sources and custom sources.</p>
    pub fn set_meta_store_manager_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_meta_store_manager_role_arn(input);
        self
    }
}
