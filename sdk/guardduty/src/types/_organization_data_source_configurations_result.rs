// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that contains information on which data sources are automatically enabled for new members within the organization.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OrganizationDataSourceConfigurationsResult {
    /// <p>Describes whether S3 data event logs are enabled as a data source.</p>
    #[doc(hidden)]
    pub s3_logs: ::std::option::Option<crate::types::OrganizationS3LogsConfigurationResult>,
    /// <p>Describes the configuration of Kubernetes data sources.</p>
    #[doc(hidden)]
    pub kubernetes: ::std::option::Option<crate::types::OrganizationKubernetesConfigurationResult>,
    /// <p>Describes the configuration of Malware Protection data source for an organization.</p>
    #[doc(hidden)]
    pub malware_protection:
        ::std::option::Option<crate::types::OrganizationMalwareProtectionConfigurationResult>,
}
impl OrganizationDataSourceConfigurationsResult {
    /// <p>Describes whether S3 data event logs are enabled as a data source.</p>
    pub fn s3_logs(
        &self,
    ) -> ::std::option::Option<&crate::types::OrganizationS3LogsConfigurationResult> {
        self.s3_logs.as_ref()
    }
    /// <p>Describes the configuration of Kubernetes data sources.</p>
    pub fn kubernetes(
        &self,
    ) -> ::std::option::Option<&crate::types::OrganizationKubernetesConfigurationResult> {
        self.kubernetes.as_ref()
    }
    /// <p>Describes the configuration of Malware Protection data source for an organization.</p>
    pub fn malware_protection(
        &self,
    ) -> ::std::option::Option<&crate::types::OrganizationMalwareProtectionConfigurationResult>
    {
        self.malware_protection.as_ref()
    }
}
impl OrganizationDataSourceConfigurationsResult {
    /// Creates a new builder-style object to manufacture [`OrganizationDataSourceConfigurationsResult`](crate::types::OrganizationDataSourceConfigurationsResult).
    pub fn builder() -> crate::types::builders::OrganizationDataSourceConfigurationsResultBuilder {
        crate::types::builders::OrganizationDataSourceConfigurationsResultBuilder::default()
    }
}

/// A builder for [`OrganizationDataSourceConfigurationsResult`](crate::types::OrganizationDataSourceConfigurationsResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OrganizationDataSourceConfigurationsResultBuilder {
    pub(crate) s3_logs: ::std::option::Option<crate::types::OrganizationS3LogsConfigurationResult>,
    pub(crate) kubernetes:
        ::std::option::Option<crate::types::OrganizationKubernetesConfigurationResult>,
    pub(crate) malware_protection:
        ::std::option::Option<crate::types::OrganizationMalwareProtectionConfigurationResult>,
}
impl OrganizationDataSourceConfigurationsResultBuilder {
    /// <p>Describes whether S3 data event logs are enabled as a data source.</p>
    pub fn s3_logs(mut self, input: crate::types::OrganizationS3LogsConfigurationResult) -> Self {
        self.s3_logs = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes whether S3 data event logs are enabled as a data source.</p>
    pub fn set_s3_logs(
        mut self,
        input: ::std::option::Option<crate::types::OrganizationS3LogsConfigurationResult>,
    ) -> Self {
        self.s3_logs = input;
        self
    }
    /// <p>Describes the configuration of Kubernetes data sources.</p>
    pub fn kubernetes(
        mut self,
        input: crate::types::OrganizationKubernetesConfigurationResult,
    ) -> Self {
        self.kubernetes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the configuration of Kubernetes data sources.</p>
    pub fn set_kubernetes(
        mut self,
        input: ::std::option::Option<crate::types::OrganizationKubernetesConfigurationResult>,
    ) -> Self {
        self.kubernetes = input;
        self
    }
    /// <p>Describes the configuration of Malware Protection data source for an organization.</p>
    pub fn malware_protection(
        mut self,
        input: crate::types::OrganizationMalwareProtectionConfigurationResult,
    ) -> Self {
        self.malware_protection = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the configuration of Malware Protection data source for an organization.</p>
    pub fn set_malware_protection(
        mut self,
        input: ::std::option::Option<
            crate::types::OrganizationMalwareProtectionConfigurationResult,
        >,
    ) -> Self {
        self.malware_protection = input;
        self
    }
    /// Consumes the builder and constructs a [`OrganizationDataSourceConfigurationsResult`](crate::types::OrganizationDataSourceConfigurationsResult).
    pub fn build(self) -> crate::types::OrganizationDataSourceConfigurationsResult {
        crate::types::OrganizationDataSourceConfigurationsResult {
            s3_logs: self.s3_logs,
            kubernetes: self.kubernetes,
            malware_protection: self.malware_protection,
        }
    }
}
