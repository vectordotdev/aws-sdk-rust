// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the effective recommendation preferences for a resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EffectiveRecommendationPreferences {
    /// <p>Describes the CPU vendor and architecture for an instance or Auto Scaling group recommendations.</p>
    /// <p>For example, when you specify <code>AWS_ARM64</code> with:</p>
    /// <ul>
    /// <li> <p>A <code>GetEC2InstanceRecommendations</code> or <code>GetAutoScalingGroupRecommendations</code> request, Compute Optimizer returns recommendations that consist of Graviton2 instance types only.</p> </li>
    /// <li> <p>A <code>GetEC2RecommendationProjectedMetrics</code> request, Compute Optimizer returns projected utilization metrics for Graviton2 instance type recommendations only.</p> </li>
    /// <li> <p>A <code>ExportEC2InstanceRecommendations</code> or <code>ExportAutoScalingGroupRecommendations</code> request, Compute Optimizer exports recommendations that consist of Graviton2 instance types only.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub cpu_vendor_architectures:
        ::std::option::Option<::std::vec::Vec<crate::types::CpuVendorArchitecture>>,
    /// <p>Describes the activation status of the enhanced infrastructure metrics preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh, and a status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Enhanced infrastructure metrics</a> in the <i>Compute Optimizer User Guide</i>.</p>
    #[doc(hidden)]
    pub enhanced_infrastructure_metrics:
        ::std::option::Option<crate::types::EnhancedInfrastructureMetrics>,
    /// <p>Describes the activation status of the inferred workload types preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh. A status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    #[doc(hidden)]
    pub inferred_workload_types:
        ::std::option::Option<crate::types::InferredWorkloadTypesPreference>,
    /// <p> An object that describes the external metrics recommendation preference. </p>
    /// <p> If the preference is applied in the latest recommendation refresh, an object with a valid <code>source</code> value appears in the response. If the preference isn't applied to the recommendations already, then this object doesn't appear in the response. </p>
    #[doc(hidden)]
    pub external_metrics_preference: ::std::option::Option<crate::types::ExternalMetricsPreference>,
}
impl EffectiveRecommendationPreferences {
    /// <p>Describes the CPU vendor and architecture for an instance or Auto Scaling group recommendations.</p>
    /// <p>For example, when you specify <code>AWS_ARM64</code> with:</p>
    /// <ul>
    /// <li> <p>A <code>GetEC2InstanceRecommendations</code> or <code>GetAutoScalingGroupRecommendations</code> request, Compute Optimizer returns recommendations that consist of Graviton2 instance types only.</p> </li>
    /// <li> <p>A <code>GetEC2RecommendationProjectedMetrics</code> request, Compute Optimizer returns projected utilization metrics for Graviton2 instance type recommendations only.</p> </li>
    /// <li> <p>A <code>ExportEC2InstanceRecommendations</code> or <code>ExportAutoScalingGroupRecommendations</code> request, Compute Optimizer exports recommendations that consist of Graviton2 instance types only.</p> </li>
    /// </ul>
    pub fn cpu_vendor_architectures(
        &self,
    ) -> ::std::option::Option<&[crate::types::CpuVendorArchitecture]> {
        self.cpu_vendor_architectures.as_deref()
    }
    /// <p>Describes the activation status of the enhanced infrastructure metrics preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh, and a status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Enhanced infrastructure metrics</a> in the <i>Compute Optimizer User Guide</i>.</p>
    pub fn enhanced_infrastructure_metrics(
        &self,
    ) -> ::std::option::Option<&crate::types::EnhancedInfrastructureMetrics> {
        self.enhanced_infrastructure_metrics.as_ref()
    }
    /// <p>Describes the activation status of the inferred workload types preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh. A status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    pub fn inferred_workload_types(
        &self,
    ) -> ::std::option::Option<&crate::types::InferredWorkloadTypesPreference> {
        self.inferred_workload_types.as_ref()
    }
    /// <p> An object that describes the external metrics recommendation preference. </p>
    /// <p> If the preference is applied in the latest recommendation refresh, an object with a valid <code>source</code> value appears in the response. If the preference isn't applied to the recommendations already, then this object doesn't appear in the response. </p>
    pub fn external_metrics_preference(
        &self,
    ) -> ::std::option::Option<&crate::types::ExternalMetricsPreference> {
        self.external_metrics_preference.as_ref()
    }
}
impl EffectiveRecommendationPreferences {
    /// Creates a new builder-style object to manufacture [`EffectiveRecommendationPreferences`](crate::types::EffectiveRecommendationPreferences).
    pub fn builder() -> crate::types::builders::EffectiveRecommendationPreferencesBuilder {
        crate::types::builders::EffectiveRecommendationPreferencesBuilder::default()
    }
}

/// A builder for [`EffectiveRecommendationPreferences`](crate::types::EffectiveRecommendationPreferences).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EffectiveRecommendationPreferencesBuilder {
    pub(crate) cpu_vendor_architectures:
        ::std::option::Option<::std::vec::Vec<crate::types::CpuVendorArchitecture>>,
    pub(crate) enhanced_infrastructure_metrics:
        ::std::option::Option<crate::types::EnhancedInfrastructureMetrics>,
    pub(crate) inferred_workload_types:
        ::std::option::Option<crate::types::InferredWorkloadTypesPreference>,
    pub(crate) external_metrics_preference:
        ::std::option::Option<crate::types::ExternalMetricsPreference>,
}
impl EffectiveRecommendationPreferencesBuilder {
    /// Appends an item to `cpu_vendor_architectures`.
    ///
    /// To override the contents of this collection use [`set_cpu_vendor_architectures`](Self::set_cpu_vendor_architectures).
    ///
    /// <p>Describes the CPU vendor and architecture for an instance or Auto Scaling group recommendations.</p>
    /// <p>For example, when you specify <code>AWS_ARM64</code> with:</p>
    /// <ul>
    /// <li> <p>A <code>GetEC2InstanceRecommendations</code> or <code>GetAutoScalingGroupRecommendations</code> request, Compute Optimizer returns recommendations that consist of Graviton2 instance types only.</p> </li>
    /// <li> <p>A <code>GetEC2RecommendationProjectedMetrics</code> request, Compute Optimizer returns projected utilization metrics for Graviton2 instance type recommendations only.</p> </li>
    /// <li> <p>A <code>ExportEC2InstanceRecommendations</code> or <code>ExportAutoScalingGroupRecommendations</code> request, Compute Optimizer exports recommendations that consist of Graviton2 instance types only.</p> </li>
    /// </ul>
    pub fn cpu_vendor_architectures(mut self, input: crate::types::CpuVendorArchitecture) -> Self {
        let mut v = self.cpu_vendor_architectures.unwrap_or_default();
        v.push(input);
        self.cpu_vendor_architectures = ::std::option::Option::Some(v);
        self
    }
    /// <p>Describes the CPU vendor and architecture for an instance or Auto Scaling group recommendations.</p>
    /// <p>For example, when you specify <code>AWS_ARM64</code> with:</p>
    /// <ul>
    /// <li> <p>A <code>GetEC2InstanceRecommendations</code> or <code>GetAutoScalingGroupRecommendations</code> request, Compute Optimizer returns recommendations that consist of Graviton2 instance types only.</p> </li>
    /// <li> <p>A <code>GetEC2RecommendationProjectedMetrics</code> request, Compute Optimizer returns projected utilization metrics for Graviton2 instance type recommendations only.</p> </li>
    /// <li> <p>A <code>ExportEC2InstanceRecommendations</code> or <code>ExportAutoScalingGroupRecommendations</code> request, Compute Optimizer exports recommendations that consist of Graviton2 instance types only.</p> </li>
    /// </ul>
    pub fn set_cpu_vendor_architectures(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CpuVendorArchitecture>>,
    ) -> Self {
        self.cpu_vendor_architectures = input;
        self
    }
    /// <p>Describes the activation status of the enhanced infrastructure metrics preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh, and a status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Enhanced infrastructure metrics</a> in the <i>Compute Optimizer User Guide</i>.</p>
    pub fn enhanced_infrastructure_metrics(
        mut self,
        input: crate::types::EnhancedInfrastructureMetrics,
    ) -> Self {
        self.enhanced_infrastructure_metrics = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the activation status of the enhanced infrastructure metrics preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh, and a status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Enhanced infrastructure metrics</a> in the <i>Compute Optimizer User Guide</i>.</p>
    pub fn set_enhanced_infrastructure_metrics(
        mut self,
        input: ::std::option::Option<crate::types::EnhancedInfrastructureMetrics>,
    ) -> Self {
        self.enhanced_infrastructure_metrics = input;
        self
    }
    /// <p>Describes the activation status of the inferred workload types preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh. A status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    pub fn inferred_workload_types(
        mut self,
        input: crate::types::InferredWorkloadTypesPreference,
    ) -> Self {
        self.inferred_workload_types = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the activation status of the inferred workload types preference.</p>
    /// <p>A status of <code>Active</code> confirms that the preference is applied in the latest recommendation refresh. A status of <code>Inactive</code> confirms that it's not yet applied to recommendations.</p>
    pub fn set_inferred_workload_types(
        mut self,
        input: ::std::option::Option<crate::types::InferredWorkloadTypesPreference>,
    ) -> Self {
        self.inferred_workload_types = input;
        self
    }
    /// <p> An object that describes the external metrics recommendation preference. </p>
    /// <p> If the preference is applied in the latest recommendation refresh, an object with a valid <code>source</code> value appears in the response. If the preference isn't applied to the recommendations already, then this object doesn't appear in the response. </p>
    pub fn external_metrics_preference(
        mut self,
        input: crate::types::ExternalMetricsPreference,
    ) -> Self {
        self.external_metrics_preference = ::std::option::Option::Some(input);
        self
    }
    /// <p> An object that describes the external metrics recommendation preference. </p>
    /// <p> If the preference is applied in the latest recommendation refresh, an object with a valid <code>source</code> value appears in the response. If the preference isn't applied to the recommendations already, then this object doesn't appear in the response. </p>
    pub fn set_external_metrics_preference(
        mut self,
        input: ::std::option::Option<crate::types::ExternalMetricsPreference>,
    ) -> Self {
        self.external_metrics_preference = input;
        self
    }
    /// Consumes the builder and constructs a [`EffectiveRecommendationPreferences`](crate::types::EffectiveRecommendationPreferences).
    pub fn build(self) -> crate::types::EffectiveRecommendationPreferences {
        crate::types::EffectiveRecommendationPreferences {
            cpu_vendor_architectures: self.cpu_vendor_architectures,
            enhanced_infrastructure_metrics: self.enhanced_infrastructure_metrics,
            inferred_workload_types: self.inferred_workload_types,
            external_metrics_preference: self.external_metrics_preference,
        }
    }
}
