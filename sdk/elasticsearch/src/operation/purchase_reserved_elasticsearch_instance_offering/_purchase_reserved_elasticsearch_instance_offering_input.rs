// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for parameters to <code>PurchaseReservedElasticsearchInstanceOffering</code></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PurchaseReservedElasticsearchInstanceOfferingInput {
    /// <p>The ID of the reserved Elasticsearch instance offering to purchase.</p>
    #[doc(hidden)]
    pub reserved_elasticsearch_instance_offering_id: ::std::option::Option<::std::string::String>,
    /// <p>A customer-specified identifier to track this reservation.</p>
    #[doc(hidden)]
    pub reservation_name: ::std::option::Option<::std::string::String>,
    /// <p>The number of Elasticsearch instances to reserve.</p>
    #[doc(hidden)]
    pub instance_count: ::std::option::Option<i32>,
}
impl PurchaseReservedElasticsearchInstanceOfferingInput {
    /// <p>The ID of the reserved Elasticsearch instance offering to purchase.</p>
    pub fn reserved_elasticsearch_instance_offering_id(&self) -> ::std::option::Option<&str> {
        self.reserved_elasticsearch_instance_offering_id.as_deref()
    }
    /// <p>A customer-specified identifier to track this reservation.</p>
    pub fn reservation_name(&self) -> ::std::option::Option<&str> {
        self.reservation_name.as_deref()
    }
    /// <p>The number of Elasticsearch instances to reserve.</p>
    pub fn instance_count(&self) -> ::std::option::Option<i32> {
        self.instance_count
    }
}
impl PurchaseReservedElasticsearchInstanceOfferingInput {
    /// Creates a new builder-style object to manufacture [`PurchaseReservedElasticsearchInstanceOfferingInput`](crate::operation::purchase_reserved_elasticsearch_instance_offering::PurchaseReservedElasticsearchInstanceOfferingInput).
    pub fn builder() -> crate::operation::purchase_reserved_elasticsearch_instance_offering::builders::PurchaseReservedElasticsearchInstanceOfferingInputBuilder{
        crate::operation::purchase_reserved_elasticsearch_instance_offering::builders::PurchaseReservedElasticsearchInstanceOfferingInputBuilder::default()
    }
}

/// A builder for [`PurchaseReservedElasticsearchInstanceOfferingInput`](crate::operation::purchase_reserved_elasticsearch_instance_offering::PurchaseReservedElasticsearchInstanceOfferingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PurchaseReservedElasticsearchInstanceOfferingInputBuilder {
    pub(crate) reserved_elasticsearch_instance_offering_id:
        ::std::option::Option<::std::string::String>,
    pub(crate) reservation_name: ::std::option::Option<::std::string::String>,
    pub(crate) instance_count: ::std::option::Option<i32>,
}
impl PurchaseReservedElasticsearchInstanceOfferingInputBuilder {
    /// <p>The ID of the reserved Elasticsearch instance offering to purchase.</p>
    pub fn reserved_elasticsearch_instance_offering_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reserved_elasticsearch_instance_offering_id =
            ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the reserved Elasticsearch instance offering to purchase.</p>
    pub fn set_reserved_elasticsearch_instance_offering_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reserved_elasticsearch_instance_offering_id = input;
        self
    }
    /// <p>A customer-specified identifier to track this reservation.</p>
    pub fn reservation_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reservation_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A customer-specified identifier to track this reservation.</p>
    pub fn set_reservation_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reservation_name = input;
        self
    }
    /// <p>The number of Elasticsearch instances to reserve.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of Elasticsearch instances to reserve.</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// Consumes the builder and constructs a [`PurchaseReservedElasticsearchInstanceOfferingInput`](crate::operation::purchase_reserved_elasticsearch_instance_offering::PurchaseReservedElasticsearchInstanceOfferingInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::purchase_reserved_elasticsearch_instance_offering::PurchaseReservedElasticsearchInstanceOfferingInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::purchase_reserved_elasticsearch_instance_offering::PurchaseReservedElasticsearchInstanceOfferingInput {
                reserved_elasticsearch_instance_offering_id: self.reserved_elasticsearch_instance_offering_id
                ,
                reservation_name: self.reservation_name
                ,
                instance_count: self.instance_count
                ,
            }
        )
    }
}
