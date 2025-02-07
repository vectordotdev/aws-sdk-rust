// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteOrganizationConformancePack`](crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_conformance_pack_name(impl ::std::convert::Into<String>)`](crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackFluentBuilder::organization_conformance_pack_name) / [`set_organization_conformance_pack_name(Option<String>)`](crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackFluentBuilder::set_organization_conformance_pack_name): <p>The name of organization conformance pack that you want to delete.</p>
    /// - On success, responds with [`DeleteOrganizationConformancePackOutput`](crate::operation::delete_organization_conformance_pack::DeleteOrganizationConformancePackOutput)
    /// - On failure, responds with [`SdkError<DeleteOrganizationConformancePackError>`](crate::operation::delete_organization_conformance_pack::DeleteOrganizationConformancePackError)
    pub fn delete_organization_conformance_pack(&self) -> crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackFluentBuilder{
        crate::operation::delete_organization_conformance_pack::builders::DeleteOrganizationConformancePackFluentBuilder::new(self.handle.clone())
    }
}
