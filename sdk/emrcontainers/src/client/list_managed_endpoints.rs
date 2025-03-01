// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListManagedEndpoints`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`virtual_cluster_id(impl ::std::convert::Into<String>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::virtual_cluster_id) / [`set_virtual_cluster_id(Option<String>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_virtual_cluster_id): <p>The ID of the virtual cluster.</p>
    ///   - [`created_before(DateTime)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::created_before) / [`set_created_before(Option<DateTime>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_created_before): <p>The date and time before which the endpoints are created.</p>
    ///   - [`created_after(DateTime)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::created_after) / [`set_created_after(Option<DateTime>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_created_after): <p> The date and time after which the endpoints are created.</p>
    ///   - [`types(Vec<String>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::types) / [`set_types(Option<Vec<String>>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_types): <p>The types of the managed endpoints.</p>
    ///   - [`states(Vec<EndpointState>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::states) / [`set_states(Option<Vec<EndpointState>>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_states): <p>The states of the managed endpoints.</p>
    ///   - [`max_results(i32)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_max_results): <p>The maximum number of managed endpoints that can be listed.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::set_next_token): <p> The token for the next set of managed endpoints to return. </p>
    /// - On success, responds with [`ListManagedEndpointsOutput`](crate::operation::list_managed_endpoints::ListManagedEndpointsOutput) with field(s):
    ///   - [`endpoints(Option<Vec<Endpoint>>)`](crate::operation::list_managed_endpoints::ListManagedEndpointsOutput::endpoints): <p>The managed endpoints to be listed.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_managed_endpoints::ListManagedEndpointsOutput::next_token): <p> The token for the next set of endpoints to return. </p>
    /// - On failure, responds with [`SdkError<ListManagedEndpointsError>`](crate::operation::list_managed_endpoints::ListManagedEndpointsError)
    pub fn list_managed_endpoints(
        &self,
    ) -> crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder {
        crate::operation::list_managed_endpoints::builders::ListManagedEndpointsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
