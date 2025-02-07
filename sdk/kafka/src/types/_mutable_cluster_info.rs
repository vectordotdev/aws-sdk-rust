// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about cluster attributes that can be updated via update APIs.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MutableClusterInfo {
    /// <p>Specifies the size of the EBS volume and the ID of the associated broker.</p>
    #[doc(hidden)]
    pub broker_ebs_volume_info:
        ::std::option::Option<::std::vec::Vec<crate::types::BrokerEbsVolumeInfo>>,
    /// <p>Information about the changes in the configuration of the brokers.</p>
    #[doc(hidden)]
    pub configuration_info: ::std::option::Option<crate::types::ConfigurationInfo>,
    /// <p>The number of broker nodes in the cluster.</p>
    #[doc(hidden)]
    pub number_of_broker_nodes: ::std::option::Option<i32>,
    /// <p>Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.</p>
    #[doc(hidden)]
    pub enhanced_monitoring: ::std::option::Option<crate::types::EnhancedMonitoring>,
    /// <p>The settings for open monitoring.</p>
    #[doc(hidden)]
    pub open_monitoring: ::std::option::Option<crate::types::OpenMonitoring>,
    /// <p>The Apache Kafka version.</p>
    #[doc(hidden)]
    pub kafka_version: ::std::option::Option<::std::string::String>,
    /// <p>You can configure your MSK cluster to send broker logs to different destination types. This is a container for the configuration details related to broker logs.</p>
    #[doc(hidden)]
    pub logging_info: ::std::option::Option<crate::types::LoggingInfo>,
    /// <p>Information about the Amazon MSK broker type.</p>
    #[doc(hidden)]
    pub instance_type: ::std::option::Option<::std::string::String>,
    /// <p>Includes all client authentication information.</p>
    #[doc(hidden)]
    pub client_authentication: ::std::option::Option<crate::types::ClientAuthentication>,
    /// <p>Includes all encryption-related information.</p>
    #[doc(hidden)]
    pub encryption_info: ::std::option::Option<crate::types::EncryptionInfo>,
    /// <p>Information about the broker access configuration.</p>
    #[doc(hidden)]
    pub connectivity_info: ::std::option::Option<crate::types::ConnectivityInfo>,
    /// <p>This controls storage mode for supported storage tiers.</p>
    #[doc(hidden)]
    pub storage_mode: ::std::option::Option<crate::types::StorageMode>,
}
impl MutableClusterInfo {
    /// <p>Specifies the size of the EBS volume and the ID of the associated broker.</p>
    pub fn broker_ebs_volume_info(
        &self,
    ) -> ::std::option::Option<&[crate::types::BrokerEbsVolumeInfo]> {
        self.broker_ebs_volume_info.as_deref()
    }
    /// <p>Information about the changes in the configuration of the brokers.</p>
    pub fn configuration_info(&self) -> ::std::option::Option<&crate::types::ConfigurationInfo> {
        self.configuration_info.as_ref()
    }
    /// <p>The number of broker nodes in the cluster.</p>
    pub fn number_of_broker_nodes(&self) -> ::std::option::Option<i32> {
        self.number_of_broker_nodes
    }
    /// <p>Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.</p>
    pub fn enhanced_monitoring(&self) -> ::std::option::Option<&crate::types::EnhancedMonitoring> {
        self.enhanced_monitoring.as_ref()
    }
    /// <p>The settings for open monitoring.</p>
    pub fn open_monitoring(&self) -> ::std::option::Option<&crate::types::OpenMonitoring> {
        self.open_monitoring.as_ref()
    }
    /// <p>The Apache Kafka version.</p>
    pub fn kafka_version(&self) -> ::std::option::Option<&str> {
        self.kafka_version.as_deref()
    }
    /// <p>You can configure your MSK cluster to send broker logs to different destination types. This is a container for the configuration details related to broker logs.</p>
    pub fn logging_info(&self) -> ::std::option::Option<&crate::types::LoggingInfo> {
        self.logging_info.as_ref()
    }
    /// <p>Information about the Amazon MSK broker type.</p>
    pub fn instance_type(&self) -> ::std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>Includes all client authentication information.</p>
    pub fn client_authentication(
        &self,
    ) -> ::std::option::Option<&crate::types::ClientAuthentication> {
        self.client_authentication.as_ref()
    }
    /// <p>Includes all encryption-related information.</p>
    pub fn encryption_info(&self) -> ::std::option::Option<&crate::types::EncryptionInfo> {
        self.encryption_info.as_ref()
    }
    /// <p>Information about the broker access configuration.</p>
    pub fn connectivity_info(&self) -> ::std::option::Option<&crate::types::ConnectivityInfo> {
        self.connectivity_info.as_ref()
    }
    /// <p>This controls storage mode for supported storage tiers.</p>
    pub fn storage_mode(&self) -> ::std::option::Option<&crate::types::StorageMode> {
        self.storage_mode.as_ref()
    }
}
impl MutableClusterInfo {
    /// Creates a new builder-style object to manufacture [`MutableClusterInfo`](crate::types::MutableClusterInfo).
    pub fn builder() -> crate::types::builders::MutableClusterInfoBuilder {
        crate::types::builders::MutableClusterInfoBuilder::default()
    }
}

/// A builder for [`MutableClusterInfo`](crate::types::MutableClusterInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MutableClusterInfoBuilder {
    pub(crate) broker_ebs_volume_info:
        ::std::option::Option<::std::vec::Vec<crate::types::BrokerEbsVolumeInfo>>,
    pub(crate) configuration_info: ::std::option::Option<crate::types::ConfigurationInfo>,
    pub(crate) number_of_broker_nodes: ::std::option::Option<i32>,
    pub(crate) enhanced_monitoring: ::std::option::Option<crate::types::EnhancedMonitoring>,
    pub(crate) open_monitoring: ::std::option::Option<crate::types::OpenMonitoring>,
    pub(crate) kafka_version: ::std::option::Option<::std::string::String>,
    pub(crate) logging_info: ::std::option::Option<crate::types::LoggingInfo>,
    pub(crate) instance_type: ::std::option::Option<::std::string::String>,
    pub(crate) client_authentication: ::std::option::Option<crate::types::ClientAuthentication>,
    pub(crate) encryption_info: ::std::option::Option<crate::types::EncryptionInfo>,
    pub(crate) connectivity_info: ::std::option::Option<crate::types::ConnectivityInfo>,
    pub(crate) storage_mode: ::std::option::Option<crate::types::StorageMode>,
}
impl MutableClusterInfoBuilder {
    /// Appends an item to `broker_ebs_volume_info`.
    ///
    /// To override the contents of this collection use [`set_broker_ebs_volume_info`](Self::set_broker_ebs_volume_info).
    ///
    /// <p>Specifies the size of the EBS volume and the ID of the associated broker.</p>
    pub fn broker_ebs_volume_info(mut self, input: crate::types::BrokerEbsVolumeInfo) -> Self {
        let mut v = self.broker_ebs_volume_info.unwrap_or_default();
        v.push(input);
        self.broker_ebs_volume_info = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the size of the EBS volume and the ID of the associated broker.</p>
    pub fn set_broker_ebs_volume_info(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BrokerEbsVolumeInfo>>,
    ) -> Self {
        self.broker_ebs_volume_info = input;
        self
    }
    /// <p>Information about the changes in the configuration of the brokers.</p>
    pub fn configuration_info(mut self, input: crate::types::ConfigurationInfo) -> Self {
        self.configuration_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the changes in the configuration of the brokers.</p>
    pub fn set_configuration_info(
        mut self,
        input: ::std::option::Option<crate::types::ConfigurationInfo>,
    ) -> Self {
        self.configuration_info = input;
        self
    }
    /// <p>The number of broker nodes in the cluster.</p>
    pub fn number_of_broker_nodes(mut self, input: i32) -> Self {
        self.number_of_broker_nodes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of broker nodes in the cluster.</p>
    pub fn set_number_of_broker_nodes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.number_of_broker_nodes = input;
        self
    }
    /// <p>Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.</p>
    pub fn enhanced_monitoring(mut self, input: crate::types::EnhancedMonitoring) -> Self {
        self.enhanced_monitoring = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies which Apache Kafka metrics Amazon MSK gathers and sends to Amazon CloudWatch for this cluster.</p>
    pub fn set_enhanced_monitoring(
        mut self,
        input: ::std::option::Option<crate::types::EnhancedMonitoring>,
    ) -> Self {
        self.enhanced_monitoring = input;
        self
    }
    /// <p>The settings for open monitoring.</p>
    pub fn open_monitoring(mut self, input: crate::types::OpenMonitoring) -> Self {
        self.open_monitoring = ::std::option::Option::Some(input);
        self
    }
    /// <p>The settings for open monitoring.</p>
    pub fn set_open_monitoring(
        mut self,
        input: ::std::option::Option<crate::types::OpenMonitoring>,
    ) -> Self {
        self.open_monitoring = input;
        self
    }
    /// <p>The Apache Kafka version.</p>
    pub fn kafka_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.kafka_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Apache Kafka version.</p>
    pub fn set_kafka_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.kafka_version = input;
        self
    }
    /// <p>You can configure your MSK cluster to send broker logs to different destination types. This is a container for the configuration details related to broker logs.</p>
    pub fn logging_info(mut self, input: crate::types::LoggingInfo) -> Self {
        self.logging_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>You can configure your MSK cluster to send broker logs to different destination types. This is a container for the configuration details related to broker logs.</p>
    pub fn set_logging_info(
        mut self,
        input: ::std::option::Option<crate::types::LoggingInfo>,
    ) -> Self {
        self.logging_info = input;
        self
    }
    /// <p>Information about the Amazon MSK broker type.</p>
    pub fn instance_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.instance_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Information about the Amazon MSK broker type.</p>
    pub fn set_instance_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>Includes all client authentication information.</p>
    pub fn client_authentication(mut self, input: crate::types::ClientAuthentication) -> Self {
        self.client_authentication = ::std::option::Option::Some(input);
        self
    }
    /// <p>Includes all client authentication information.</p>
    pub fn set_client_authentication(
        mut self,
        input: ::std::option::Option<crate::types::ClientAuthentication>,
    ) -> Self {
        self.client_authentication = input;
        self
    }
    /// <p>Includes all encryption-related information.</p>
    pub fn encryption_info(mut self, input: crate::types::EncryptionInfo) -> Self {
        self.encryption_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Includes all encryption-related information.</p>
    pub fn set_encryption_info(
        mut self,
        input: ::std::option::Option<crate::types::EncryptionInfo>,
    ) -> Self {
        self.encryption_info = input;
        self
    }
    /// <p>Information about the broker access configuration.</p>
    pub fn connectivity_info(mut self, input: crate::types::ConnectivityInfo) -> Self {
        self.connectivity_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the broker access configuration.</p>
    pub fn set_connectivity_info(
        mut self,
        input: ::std::option::Option<crate::types::ConnectivityInfo>,
    ) -> Self {
        self.connectivity_info = input;
        self
    }
    /// <p>This controls storage mode for supported storage tiers.</p>
    pub fn storage_mode(mut self, input: crate::types::StorageMode) -> Self {
        self.storage_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>This controls storage mode for supported storage tiers.</p>
    pub fn set_storage_mode(
        mut self,
        input: ::std::option::Option<crate::types::StorageMode>,
    ) -> Self {
        self.storage_mode = input;
        self
    }
    /// Consumes the builder and constructs a [`MutableClusterInfo`](crate::types::MutableClusterInfo).
    pub fn build(self) -> crate::types::MutableClusterInfo {
        crate::types::MutableClusterInfo {
            broker_ebs_volume_info: self.broker_ebs_volume_info,
            configuration_info: self.configuration_info,
            number_of_broker_nodes: self.number_of_broker_nodes,
            enhanced_monitoring: self.enhanced_monitoring,
            open_monitoring: self.open_monitoring,
            kafka_version: self.kafka_version,
            logging_info: self.logging_info,
            instance_type: self.instance_type,
            client_authentication: self.client_authentication,
            encryption_info: self.encryption_info,
            connectivity_info: self.connectivity_info,
            storage_mode: self.storage_mode,
        }
    }
}
