// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The pricing plan.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PricingPlan {
    /// <p>The billable entity count.</p>
    #[doc(hidden)]
    pub billable_entity_count: ::std::option::Option<i64>,
    /// <p>The pricing plan's bundle information.</p>
    #[doc(hidden)]
    pub bundle_information: ::std::option::Option<crate::types::BundleInformation>,
    /// <p>The effective date and time of the pricing plan.</p>
    #[doc(hidden)]
    pub effective_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The pricing mode.</p>
    #[doc(hidden)]
    pub pricing_mode: ::std::option::Option<crate::types::PricingMode>,
    /// <p>The set date and time for updating a pricing plan.</p>
    #[doc(hidden)]
    pub update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The update reason for changing a pricing plan.</p>
    #[doc(hidden)]
    pub update_reason: ::std::option::Option<crate::types::UpdateReason>,
}
impl PricingPlan {
    /// <p>The billable entity count.</p>
    pub fn billable_entity_count(&self) -> ::std::option::Option<i64> {
        self.billable_entity_count
    }
    /// <p>The pricing plan's bundle information.</p>
    pub fn bundle_information(&self) -> ::std::option::Option<&crate::types::BundleInformation> {
        self.bundle_information.as_ref()
    }
    /// <p>The effective date and time of the pricing plan.</p>
    pub fn effective_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.effective_date_time.as_ref()
    }
    /// <p>The pricing mode.</p>
    pub fn pricing_mode(&self) -> ::std::option::Option<&crate::types::PricingMode> {
        self.pricing_mode.as_ref()
    }
    /// <p>The set date and time for updating a pricing plan.</p>
    pub fn update_date_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_date_time.as_ref()
    }
    /// <p>The update reason for changing a pricing plan.</p>
    pub fn update_reason(&self) -> ::std::option::Option<&crate::types::UpdateReason> {
        self.update_reason.as_ref()
    }
}
impl PricingPlan {
    /// Creates a new builder-style object to manufacture [`PricingPlan`](crate::types::PricingPlan).
    pub fn builder() -> crate::types::builders::PricingPlanBuilder {
        crate::types::builders::PricingPlanBuilder::default()
    }
}

/// A builder for [`PricingPlan`](crate::types::PricingPlan).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PricingPlanBuilder {
    pub(crate) billable_entity_count: ::std::option::Option<i64>,
    pub(crate) bundle_information: ::std::option::Option<crate::types::BundleInformation>,
    pub(crate) effective_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) pricing_mode: ::std::option::Option<crate::types::PricingMode>,
    pub(crate) update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_reason: ::std::option::Option<crate::types::UpdateReason>,
}
impl PricingPlanBuilder {
    /// <p>The billable entity count.</p>
    pub fn billable_entity_count(mut self, input: i64) -> Self {
        self.billable_entity_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The billable entity count.</p>
    pub fn set_billable_entity_count(mut self, input: ::std::option::Option<i64>) -> Self {
        self.billable_entity_count = input;
        self
    }
    /// <p>The pricing plan's bundle information.</p>
    pub fn bundle_information(mut self, input: crate::types::BundleInformation) -> Self {
        self.bundle_information = ::std::option::Option::Some(input);
        self
    }
    /// <p>The pricing plan's bundle information.</p>
    pub fn set_bundle_information(
        mut self,
        input: ::std::option::Option<crate::types::BundleInformation>,
    ) -> Self {
        self.bundle_information = input;
        self
    }
    /// <p>The effective date and time of the pricing plan.</p>
    pub fn effective_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.effective_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The effective date and time of the pricing plan.</p>
    pub fn set_effective_date_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.effective_date_time = input;
        self
    }
    /// <p>The pricing mode.</p>
    pub fn pricing_mode(mut self, input: crate::types::PricingMode) -> Self {
        self.pricing_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The pricing mode.</p>
    pub fn set_pricing_mode(
        mut self,
        input: ::std::option::Option<crate::types::PricingMode>,
    ) -> Self {
        self.pricing_mode = input;
        self
    }
    /// <p>The set date and time for updating a pricing plan.</p>
    pub fn update_date_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_date_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The set date and time for updating a pricing plan.</p>
    pub fn set_update_date_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_date_time = input;
        self
    }
    /// <p>The update reason for changing a pricing plan.</p>
    pub fn update_reason(mut self, input: crate::types::UpdateReason) -> Self {
        self.update_reason = ::std::option::Option::Some(input);
        self
    }
    /// <p>The update reason for changing a pricing plan.</p>
    pub fn set_update_reason(
        mut self,
        input: ::std::option::Option<crate::types::UpdateReason>,
    ) -> Self {
        self.update_reason = input;
        self
    }
    /// Consumes the builder and constructs a [`PricingPlan`](crate::types::PricingPlan).
    pub fn build(self) -> crate::types::PricingPlan {
        crate::types::PricingPlan {
            billable_entity_count: self.billable_entity_count,
            bundle_information: self.bundle_information,
            effective_date_time: self.effective_date_time,
            pricing_mode: self.pricing_mode,
            update_date_time: self.update_date_time,
            update_reason: self.update_reason,
        }
    }
}
