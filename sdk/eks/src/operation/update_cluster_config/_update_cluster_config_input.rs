// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateClusterConfigInput {
    /// <p>The name of the Amazon EKS cluster to update.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
    #[doc(hidden)]
    pub resources_vpc_config: ::std::option::Option<crate::types::VpcConfigRequest>,
    /// <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS cluster control plane logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note>
    /// <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>
    /// </note>
    #[doc(hidden)]
    pub logging: ::std::option::Option<crate::types::Logging>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[doc(hidden)]
    pub client_request_token: ::std::option::Option<::std::string::String>,
}
impl UpdateClusterConfigInput {
    /// <p>The name of the Amazon EKS cluster to update.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
    pub fn resources_vpc_config(&self) -> ::std::option::Option<&crate::types::VpcConfigRequest> {
        self.resources_vpc_config.as_ref()
    }
    /// <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS cluster control plane logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note>
    /// <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>
    /// </note>
    pub fn logging(&self) -> ::std::option::Option<&crate::types::Logging> {
        self.logging.as_ref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
}
impl UpdateClusterConfigInput {
    /// Creates a new builder-style object to manufacture [`UpdateClusterConfigInput`](crate::operation::update_cluster_config::UpdateClusterConfigInput).
    pub fn builder(
    ) -> crate::operation::update_cluster_config::builders::UpdateClusterConfigInputBuilder {
        crate::operation::update_cluster_config::builders::UpdateClusterConfigInputBuilder::default(
        )
    }
}

/// A builder for [`UpdateClusterConfigInput`](crate::operation::update_cluster_config::UpdateClusterConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateClusterConfigInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) resources_vpc_config: ::std::option::Option<crate::types::VpcConfigRequest>,
    pub(crate) logging: ::std::option::Option<crate::types::Logging>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
}
impl UpdateClusterConfigInputBuilder {
    /// <p>The name of the Amazon EKS cluster to update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon EKS cluster to update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
    pub fn resources_vpc_config(mut self, input: crate::types::VpcConfigRequest) -> Self {
        self.resources_vpc_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
    pub fn set_resources_vpc_config(
        mut self,
        input: ::std::option::Option<crate::types::VpcConfigRequest>,
    ) -> Self {
        self.resources_vpc_config = input;
        self
    }
    /// <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS cluster control plane logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note>
    /// <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>
    /// </note>
    pub fn logging(mut self, input: crate::types::Logging) -> Self {
        self.logging = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS cluster control plane logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note>
    /// <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>
    /// </note>
    pub fn set_logging(mut self, input: ::std::option::Option<crate::types::Logging>) -> Self {
        self.logging = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.client_request_token = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateClusterConfigInput`](crate::operation::update_cluster_config::UpdateClusterConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_cluster_config::UpdateClusterConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_cluster_config::UpdateClusterConfigInput {
                name: self.name,
                resources_vpc_config: self.resources_vpc_config,
                logging: self.logging,
                client_request_token: self.client_request_token,
            },
        )
    }
}
