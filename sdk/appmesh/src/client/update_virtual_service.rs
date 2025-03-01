// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateVirtualService`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`virtual_service_name(impl ::std::convert::Into<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::virtual_service_name) / [`set_virtual_service_name(Option<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::set_virtual_service_name): <p>The name of the virtual service to update.</p>
    ///   - [`mesh_name(impl ::std::convert::Into<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::mesh_name) / [`set_mesh_name(Option<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::set_mesh_name): <p>The name of the service mesh that the virtual service resides in.</p>
    ///   - [`spec(VirtualServiceSpec)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::spec) / [`set_spec(Option<VirtualServiceSpec>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::set_spec): <p>The new virtual service specification to apply. This overwrites the existing data.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    ///   - [`mesh_owner(impl ::std::convert::Into<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::mesh_owner) / [`set_mesh_owner(Option<String>)`](crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::set_mesh_owner): <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    /// - On success, responds with [`UpdateVirtualServiceOutput`](crate::operation::update_virtual_service::UpdateVirtualServiceOutput) with field(s):
    ///   - [`virtual_service(Option<VirtualServiceData>)`](crate::operation::update_virtual_service::UpdateVirtualServiceOutput::virtual_service): <p>A full description of the virtual service that was updated.</p>
    /// - On failure, responds with [`SdkError<UpdateVirtualServiceError>`](crate::operation::update_virtual_service::UpdateVirtualServiceError)
    pub fn update_virtual_service(
        &self,
    ) -> crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder {
        crate::operation::update_virtual_service::builders::UpdateVirtualServiceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
