// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a virtual service returned by a describe operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualServiceData {
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    #[doc(hidden)]
    pub mesh_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the virtual service.</p>
    #[doc(hidden)]
    pub virtual_service_name: ::std::option::Option<::std::string::String>,
    /// <p>The specifications of the virtual service.</p>
    #[doc(hidden)]
    pub spec: ::std::option::Option<crate::types::VirtualServiceSpec>,
    /// <p>An object that represents metadata for a resource.</p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<crate::types::ResourceMetadata>,
    /// <p>The current status of the virtual service.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::VirtualServiceStatus>,
}
impl VirtualServiceData {
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    pub fn mesh_name(&self) -> ::std::option::Option<&str> {
        self.mesh_name.as_deref()
    }
    /// <p>The name of the virtual service.</p>
    pub fn virtual_service_name(&self) -> ::std::option::Option<&str> {
        self.virtual_service_name.as_deref()
    }
    /// <p>The specifications of the virtual service.</p>
    pub fn spec(&self) -> ::std::option::Option<&crate::types::VirtualServiceSpec> {
        self.spec.as_ref()
    }
    /// <p>An object that represents metadata for a resource.</p>
    pub fn metadata(&self) -> ::std::option::Option<&crate::types::ResourceMetadata> {
        self.metadata.as_ref()
    }
    /// <p>The current status of the virtual service.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::VirtualServiceStatus> {
        self.status.as_ref()
    }
}
impl VirtualServiceData {
    /// Creates a new builder-style object to manufacture [`VirtualServiceData`](crate::types::VirtualServiceData).
    pub fn builder() -> crate::types::builders::VirtualServiceDataBuilder {
        crate::types::builders::VirtualServiceDataBuilder::default()
    }
}

/// A builder for [`VirtualServiceData`](crate::types::VirtualServiceData).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualServiceDataBuilder {
    pub(crate) mesh_name: ::std::option::Option<::std::string::String>,
    pub(crate) virtual_service_name: ::std::option::Option<::std::string::String>,
    pub(crate) spec: ::std::option::Option<crate::types::VirtualServiceSpec>,
    pub(crate) metadata: ::std::option::Option<crate::types::ResourceMetadata>,
    pub(crate) status: ::std::option::Option<crate::types::VirtualServiceStatus>,
}
impl VirtualServiceDataBuilder {
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    pub fn mesh_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mesh_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service mesh that the virtual service resides in.</p>
    pub fn set_mesh_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mesh_name = input;
        self
    }
    /// <p>The name of the virtual service.</p>
    pub fn virtual_service_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.virtual_service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the virtual service.</p>
    pub fn set_virtual_service_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.virtual_service_name = input;
        self
    }
    /// <p>The specifications of the virtual service.</p>
    pub fn spec(mut self, input: crate::types::VirtualServiceSpec) -> Self {
        self.spec = ::std::option::Option::Some(input);
        self
    }
    /// <p>The specifications of the virtual service.</p>
    pub fn set_spec(
        mut self,
        input: ::std::option::Option<crate::types::VirtualServiceSpec>,
    ) -> Self {
        self.spec = input;
        self
    }
    /// <p>An object that represents metadata for a resource.</p>
    pub fn metadata(mut self, input: crate::types::ResourceMetadata) -> Self {
        self.metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object that represents metadata for a resource.</p>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<crate::types::ResourceMetadata>,
    ) -> Self {
        self.metadata = input;
        self
    }
    /// <p>The current status of the virtual service.</p>
    pub fn status(mut self, input: crate::types::VirtualServiceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the virtual service.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::VirtualServiceStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualServiceData`](crate::types::VirtualServiceData).
    pub fn build(self) -> crate::types::VirtualServiceData {
        crate::types::VirtualServiceData {
            mesh_name: self.mesh_name,
            virtual_service_name: self.virtual_service_name,
            spec: self.spec,
            metadata: self.metadata,
            status: self.status,
        }
    }
}
