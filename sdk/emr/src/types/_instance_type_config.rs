// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An instance type configuration for each instance type in an instance fleet, which determines the Amazon EC2 instances Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities. When you use an allocation strategy, you can include a maximum of 30 instance type configurations for a fleet. For more information about how to use an allocation strategy, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-fleet.html">Configure Instance Fleets</a>. Without an allocation strategy, you may specify a maximum of five instance type configurations for a fleet.</p> <note>
/// <p>The instance fleet configuration is available only in Amazon EMR releases 4.8.0 and later, excluding 5.0.x versions.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceTypeConfig {
    /// <p>An Amazon EC2 instance type, such as <code>m3.xlarge</code>. </p>
    #[doc(hidden)]
    pub instance_type: ::std::option::Option<::std::string::String>,
    /// <p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <code>InstanceFleetConfig</code>. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified. </p>
    #[doc(hidden)]
    pub weighted_capacity: ::std::option::Option<i32>,
    /// <p>The bid price for each Amazon EC2 Spot Instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%. </p>
    #[doc(hidden)]
    pub bid_price: ::std::option::Option<::std::string::String>,
    /// <p>The bid price, as a percentage of On-Demand price, for each Amazon EC2 Spot Instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%). If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    #[doc(hidden)]
    pub bid_price_as_percentage_of_on_demand_price: ::std::option::Option<f64>,
    /// <p>The configuration of Amazon Elastic Block Store (Amazon EBS) attached to each instance as defined by <code>InstanceType</code>. </p>
    #[doc(hidden)]
    pub ebs_configuration: ::std::option::Option<crate::types::EbsConfiguration>,
    /// <p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster.</p>
    #[doc(hidden)]
    pub configurations: ::std::option::Option<::std::vec::Vec<crate::types::Configuration>>,
    /// <p>The custom AMI ID to use for the instance type.</p>
    #[doc(hidden)]
    pub custom_ami_id: ::std::option::Option<::std::string::String>,
}
impl InstanceTypeConfig {
    /// <p>An Amazon EC2 instance type, such as <code>m3.xlarge</code>. </p>
    pub fn instance_type(&self) -> ::std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <code>InstanceFleetConfig</code>. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified. </p>
    pub fn weighted_capacity(&self) -> ::std::option::Option<i32> {
        self.weighted_capacity
    }
    /// <p>The bid price for each Amazon EC2 Spot Instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%. </p>
    pub fn bid_price(&self) -> ::std::option::Option<&str> {
        self.bid_price.as_deref()
    }
    /// <p>The bid price, as a percentage of On-Demand price, for each Amazon EC2 Spot Instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%). If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    pub fn bid_price_as_percentage_of_on_demand_price(&self) -> ::std::option::Option<f64> {
        self.bid_price_as_percentage_of_on_demand_price
    }
    /// <p>The configuration of Amazon Elastic Block Store (Amazon EBS) attached to each instance as defined by <code>InstanceType</code>. </p>
    pub fn ebs_configuration(&self) -> ::std::option::Option<&crate::types::EbsConfiguration> {
        self.ebs_configuration.as_ref()
    }
    /// <p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster.</p>
    pub fn configurations(&self) -> ::std::option::Option<&[crate::types::Configuration]> {
        self.configurations.as_deref()
    }
    /// <p>The custom AMI ID to use for the instance type.</p>
    pub fn custom_ami_id(&self) -> ::std::option::Option<&str> {
        self.custom_ami_id.as_deref()
    }
}
impl InstanceTypeConfig {
    /// Creates a new builder-style object to manufacture [`InstanceTypeConfig`](crate::types::InstanceTypeConfig).
    pub fn builder() -> crate::types::builders::InstanceTypeConfigBuilder {
        crate::types::builders::InstanceTypeConfigBuilder::default()
    }
}

/// A builder for [`InstanceTypeConfig`](crate::types::InstanceTypeConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InstanceTypeConfigBuilder {
    pub(crate) instance_type: ::std::option::Option<::std::string::String>,
    pub(crate) weighted_capacity: ::std::option::Option<i32>,
    pub(crate) bid_price: ::std::option::Option<::std::string::String>,
    pub(crate) bid_price_as_percentage_of_on_demand_price: ::std::option::Option<f64>,
    pub(crate) ebs_configuration: ::std::option::Option<crate::types::EbsConfiguration>,
    pub(crate) configurations: ::std::option::Option<::std::vec::Vec<crate::types::Configuration>>,
    pub(crate) custom_ami_id: ::std::option::Option<::std::string::String>,
}
impl InstanceTypeConfigBuilder {
    /// <p>An Amazon EC2 instance type, such as <code>m3.xlarge</code>. </p>
    pub fn instance_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.instance_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon EC2 instance type, such as <code>m3.xlarge</code>. </p>
    pub fn set_instance_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <code>InstanceFleetConfig</code>. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified. </p>
    pub fn weighted_capacity(mut self, input: i32) -> Self {
        self.weighted_capacity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <code>InstanceFleetConfig</code>. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified. </p>
    pub fn set_weighted_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.weighted_capacity = input;
        self
    }
    /// <p>The bid price for each Amazon EC2 Spot Instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%. </p>
    pub fn bid_price(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bid_price = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The bid price for each Amazon EC2 Spot Instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%. </p>
    pub fn set_bid_price(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bid_price = input;
        self
    }
    /// <p>The bid price, as a percentage of On-Demand price, for each Amazon EC2 Spot Instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%). If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    pub fn bid_price_as_percentage_of_on_demand_price(mut self, input: f64) -> Self {
        self.bid_price_as_percentage_of_on_demand_price = ::std::option::Option::Some(input);
        self
    }
    /// <p>The bid price, as a percentage of On-Demand price, for each Amazon EC2 Spot Instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%). If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    pub fn set_bid_price_as_percentage_of_on_demand_price(
        mut self,
        input: ::std::option::Option<f64>,
    ) -> Self {
        self.bid_price_as_percentage_of_on_demand_price = input;
        self
    }
    /// <p>The configuration of Amazon Elastic Block Store (Amazon EBS) attached to each instance as defined by <code>InstanceType</code>. </p>
    pub fn ebs_configuration(mut self, input: crate::types::EbsConfiguration) -> Self {
        self.ebs_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration of Amazon Elastic Block Store (Amazon EBS) attached to each instance as defined by <code>InstanceType</code>. </p>
    pub fn set_ebs_configuration(
        mut self,
        input: ::std::option::Option<crate::types::EbsConfiguration>,
    ) -> Self {
        self.ebs_configuration = input;
        self
    }
    /// Appends an item to `configurations`.
    ///
    /// To override the contents of this collection use [`set_configurations`](Self::set_configurations).
    ///
    /// <p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster.</p>
    pub fn configurations(mut self, input: crate::types::Configuration) -> Self {
        let mut v = self.configurations.unwrap_or_default();
        v.push(input);
        self.configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster.</p>
    pub fn set_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Configuration>>,
    ) -> Self {
        self.configurations = input;
        self
    }
    /// <p>The custom AMI ID to use for the instance type.</p>
    pub fn custom_ami_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.custom_ami_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom AMI ID to use for the instance type.</p>
    pub fn set_custom_ami_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.custom_ami_id = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceTypeConfig`](crate::types::InstanceTypeConfig).
    pub fn build(self) -> crate::types::InstanceTypeConfig {
        crate::types::InstanceTypeConfig {
            instance_type: self.instance_type,
            weighted_capacity: self.weighted_capacity,
            bid_price: self.bid_price,
            bid_price_as_percentage_of_on_demand_price: self
                .bid_price_as_percentage_of_on_demand_price,
            ebs_configuration: self.ebs_configuration,
            configurations: self.configurations,
            custom_ami_id: self.custom_ami_id,
        }
    }
}
