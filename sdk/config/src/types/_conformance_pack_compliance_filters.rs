// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Filters the conformance pack by compliance types and Config rule names.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConformancePackComplianceFilters {
    /// <p>Filters the results by Config rule names.</p>
    #[doc(hidden)]
    pub config_rule_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Filters the results by compliance.</p>
    /// <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    #[doc(hidden)]
    pub compliance_type: ::std::option::Option<crate::types::ConformancePackComplianceType>,
}
impl ConformancePackComplianceFilters {
    /// <p>Filters the results by Config rule names.</p>
    pub fn config_rule_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.config_rule_names.as_deref()
    }
    /// <p>Filters the results by compliance.</p>
    /// <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    pub fn compliance_type(
        &self,
    ) -> ::std::option::Option<&crate::types::ConformancePackComplianceType> {
        self.compliance_type.as_ref()
    }
}
impl ConformancePackComplianceFilters {
    /// Creates a new builder-style object to manufacture [`ConformancePackComplianceFilters`](crate::types::ConformancePackComplianceFilters).
    pub fn builder() -> crate::types::builders::ConformancePackComplianceFiltersBuilder {
        crate::types::builders::ConformancePackComplianceFiltersBuilder::default()
    }
}

/// A builder for [`ConformancePackComplianceFilters`](crate::types::ConformancePackComplianceFilters).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConformancePackComplianceFiltersBuilder {
    pub(crate) config_rule_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) compliance_type: ::std::option::Option<crate::types::ConformancePackComplianceType>,
}
impl ConformancePackComplianceFiltersBuilder {
    /// Appends an item to `config_rule_names`.
    ///
    /// To override the contents of this collection use [`set_config_rule_names`](Self::set_config_rule_names).
    ///
    /// <p>Filters the results by Config rule names.</p>
    pub fn config_rule_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.config_rule_names.unwrap_or_default();
        v.push(input.into());
        self.config_rule_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>Filters the results by Config rule names.</p>
    pub fn set_config_rule_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.config_rule_names = input;
        self
    }
    /// <p>Filters the results by compliance.</p>
    /// <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    pub fn compliance_type(mut self, input: crate::types::ConformancePackComplianceType) -> Self {
        self.compliance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Filters the results by compliance.</p>
    /// <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    pub fn set_compliance_type(
        mut self,
        input: ::std::option::Option<crate::types::ConformancePackComplianceType>,
    ) -> Self {
        self.compliance_type = input;
        self
    }
    /// Consumes the builder and constructs a [`ConformancePackComplianceFilters`](crate::types::ConformancePackComplianceFilters).
    pub fn build(self) -> crate::types::ConformancePackComplianceFilters {
        crate::types::ConformancePackComplianceFilters {
            config_rule_names: self.config_rule_names,
            compliance_type: self.compliance_type,
        }
    }
}
