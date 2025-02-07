// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of alert actions taken in response to an alert going into <code>InAlert</code> status.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MonitoringAlertActions {
    /// <p>An alert action taken to light up an icon on the Model Dashboard when an alert goes into <code>InAlert</code> status.</p>
    #[doc(hidden)]
    pub model_dashboard_indicator:
        ::std::option::Option<crate::types::ModelDashboardIndicatorAction>,
}
impl MonitoringAlertActions {
    /// <p>An alert action taken to light up an icon on the Model Dashboard when an alert goes into <code>InAlert</code> status.</p>
    pub fn model_dashboard_indicator(
        &self,
    ) -> ::std::option::Option<&crate::types::ModelDashboardIndicatorAction> {
        self.model_dashboard_indicator.as_ref()
    }
}
impl MonitoringAlertActions {
    /// Creates a new builder-style object to manufacture [`MonitoringAlertActions`](crate::types::MonitoringAlertActions).
    pub fn builder() -> crate::types::builders::MonitoringAlertActionsBuilder {
        crate::types::builders::MonitoringAlertActionsBuilder::default()
    }
}

/// A builder for [`MonitoringAlertActions`](crate::types::MonitoringAlertActions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MonitoringAlertActionsBuilder {
    pub(crate) model_dashboard_indicator:
        ::std::option::Option<crate::types::ModelDashboardIndicatorAction>,
}
impl MonitoringAlertActionsBuilder {
    /// <p>An alert action taken to light up an icon on the Model Dashboard when an alert goes into <code>InAlert</code> status.</p>
    pub fn model_dashboard_indicator(
        mut self,
        input: crate::types::ModelDashboardIndicatorAction,
    ) -> Self {
        self.model_dashboard_indicator = ::std::option::Option::Some(input);
        self
    }
    /// <p>An alert action taken to light up an icon on the Model Dashboard when an alert goes into <code>InAlert</code> status.</p>
    pub fn set_model_dashboard_indicator(
        mut self,
        input: ::std::option::Option<crate::types::ModelDashboardIndicatorAction>,
    ) -> Self {
        self.model_dashboard_indicator = input;
        self
    }
    /// Consumes the builder and constructs a [`MonitoringAlertActions`](crate::types::MonitoringAlertActions).
    pub fn build(self) -> crate::types::MonitoringAlertActions {
        crate::types::MonitoringAlertActions {
            model_dashboard_indicator: self.model_dashboard_indicator,
        }
    }
}
