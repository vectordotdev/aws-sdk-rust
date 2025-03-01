// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteClusterOutput {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    #[doc(hidden)]
    pub cluster_arn: ::std::option::Option<::std::string::String>,
    /// <p>The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::ClusterState>,
    _request_id: Option<String>,
}
impl DeleteClusterOutput {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn cluster_arn(&self) -> ::std::option::Option<&str> {
        self.cluster_arn.as_deref()
    }
    /// <p>The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::ClusterState> {
        self.state.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteClusterOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteClusterOutput {
    /// Creates a new builder-style object to manufacture [`DeleteClusterOutput`](crate::operation::delete_cluster::DeleteClusterOutput).
    pub fn builder() -> crate::operation::delete_cluster::builders::DeleteClusterOutputBuilder {
        crate::operation::delete_cluster::builders::DeleteClusterOutputBuilder::default()
    }
}

/// A builder for [`DeleteClusterOutput`](crate::operation::delete_cluster::DeleteClusterOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteClusterOutputBuilder {
    pub(crate) cluster_arn: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::ClusterState>,
    _request_id: Option<String>,
}
impl DeleteClusterOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn cluster_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn set_cluster_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_arn = input;
        self
    }
    /// <p>The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.</p>
    pub fn state(mut self, input: crate::types::ClusterState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the cluster. The possible states are ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ClusterState>) -> Self {
        self.state = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteClusterOutput`](crate::operation::delete_cluster::DeleteClusterOutput).
    pub fn build(self) -> crate::operation::delete_cluster::DeleteClusterOutput {
        crate::operation::delete_cluster::DeleteClusterOutput {
            cluster_arn: self.cluster_arn,
            state: self.state,
            _request_id: self._request_id,
        }
    }
}
