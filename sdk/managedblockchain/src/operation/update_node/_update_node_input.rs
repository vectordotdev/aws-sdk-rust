// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateNodeInput {
    /// <p>The unique identifier of the network that the node is on.</p>
    #[doc(hidden)]
    pub network_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the member that owns the node.</p>
    /// <p>Applies only to Hyperledger Fabric.</p>
    #[doc(hidden)]
    pub member_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the node.</p>
    #[doc(hidden)]
    pub node_id: ::std::option::Option<::std::string::String>,
    /// <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
    #[doc(hidden)]
    pub log_publishing_configuration:
        ::std::option::Option<crate::types::NodeLogPublishingConfiguration>,
}
impl UpdateNodeInput {
    /// <p>The unique identifier of the network that the node is on.</p>
    pub fn network_id(&self) -> ::std::option::Option<&str> {
        self.network_id.as_deref()
    }
    /// <p>The unique identifier of the member that owns the node.</p>
    /// <p>Applies only to Hyperledger Fabric.</p>
    pub fn member_id(&self) -> ::std::option::Option<&str> {
        self.member_id.as_deref()
    }
    /// <p>The unique identifier of the node.</p>
    pub fn node_id(&self) -> ::std::option::Option<&str> {
        self.node_id.as_deref()
    }
    /// <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
    pub fn log_publishing_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::NodeLogPublishingConfiguration> {
        self.log_publishing_configuration.as_ref()
    }
}
impl UpdateNodeInput {
    /// Creates a new builder-style object to manufacture [`UpdateNodeInput`](crate::operation::update_node::UpdateNodeInput).
    pub fn builder() -> crate::operation::update_node::builders::UpdateNodeInputBuilder {
        crate::operation::update_node::builders::UpdateNodeInputBuilder::default()
    }
}

/// A builder for [`UpdateNodeInput`](crate::operation::update_node::UpdateNodeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateNodeInputBuilder {
    pub(crate) network_id: ::std::option::Option<::std::string::String>,
    pub(crate) member_id: ::std::option::Option<::std::string::String>,
    pub(crate) node_id: ::std::option::Option<::std::string::String>,
    pub(crate) log_publishing_configuration:
        ::std::option::Option<crate::types::NodeLogPublishingConfiguration>,
}
impl UpdateNodeInputBuilder {
    /// <p>The unique identifier of the network that the node is on.</p>
    pub fn network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the network that the node is on.</p>
    pub fn set_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_id = input;
        self
    }
    /// <p>The unique identifier of the member that owns the node.</p>
    /// <p>Applies only to Hyperledger Fabric.</p>
    pub fn member_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the member that owns the node.</p>
    /// <p>Applies only to Hyperledger Fabric.</p>
    pub fn set_member_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_id = input;
        self
    }
    /// <p>The unique identifier of the node.</p>
    pub fn node_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the node.</p>
    pub fn set_node_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_id = input;
        self
    }
    /// <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
    pub fn log_publishing_configuration(
        mut self,
        input: crate::types::NodeLogPublishingConfiguration,
    ) -> Self {
        self.log_publishing_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration properties for publishing to Amazon CloudWatch Logs.</p>
    pub fn set_log_publishing_configuration(
        mut self,
        input: ::std::option::Option<crate::types::NodeLogPublishingConfiguration>,
    ) -> Self {
        self.log_publishing_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateNodeInput`](crate::operation::update_node::UpdateNodeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_node::UpdateNodeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_node::UpdateNodeInput {
            network_id: self.network_id,
            member_id: self.member_id,
            node_id: self.node_id,
            log_publishing_configuration: self.log_publishing_configuration,
        })
    }
}
