// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The service integration information about the resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceIntegrations {
    /// <p>The information about the integration of Resource Groups.</p>
    #[doc(hidden)]
    pub resource_group: ::std::option::Option<crate::types::ResourceGroup>,
}
impl ResourceIntegrations {
    /// <p>The information about the integration of Resource Groups.</p>
    pub fn resource_group(&self) -> ::std::option::Option<&crate::types::ResourceGroup> {
        self.resource_group.as_ref()
    }
}
impl ResourceIntegrations {
    /// Creates a new builder-style object to manufacture [`ResourceIntegrations`](crate::types::ResourceIntegrations).
    pub fn builder() -> crate::types::builders::ResourceIntegrationsBuilder {
        crate::types::builders::ResourceIntegrationsBuilder::default()
    }
}

/// A builder for [`ResourceIntegrations`](crate::types::ResourceIntegrations).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceIntegrationsBuilder {
    pub(crate) resource_group: ::std::option::Option<crate::types::ResourceGroup>,
}
impl ResourceIntegrationsBuilder {
    /// <p>The information about the integration of Resource Groups.</p>
    pub fn resource_group(mut self, input: crate::types::ResourceGroup) -> Self {
        self.resource_group = ::std::option::Option::Some(input);
        self
    }
    /// <p>The information about the integration of Resource Groups.</p>
    pub fn set_resource_group(
        mut self,
        input: ::std::option::Option<crate::types::ResourceGroup>,
    ) -> Self {
        self.resource_group = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourceIntegrations`](crate::types::ResourceIntegrations).
    pub fn build(self) -> crate::types::ResourceIntegrations {
        crate::types::ResourceIntegrations {
            resource_group: self.resource_group,
        }
    }
}
