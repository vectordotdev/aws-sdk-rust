// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateResource`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl ::std::convert::Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_organization_id): <p>The identifier associated with the organization for which the resource is updated.</p>
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_resource_id): <p>The identifier of the resource to be updated.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_name): <p>The name of the resource to be updated.</p>
    ///   - [`booking_options(BookingOptions)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::booking_options) / [`set_booking_options(Option<BookingOptions>)`](crate::operation::update_resource::builders::UpdateResourceFluentBuilder::set_booking_options): <p>The resource's booking options to be updated.</p>
    /// - On success, responds with [`UpdateResourceOutput`](crate::operation::update_resource::UpdateResourceOutput)
    /// - On failure, responds with [`SdkError<UpdateResourceError>`](crate::operation::update_resource::UpdateResourceError)
    pub fn update_resource(
        &self,
    ) -> crate::operation::update_resource::builders::UpdateResourceFluentBuilder {
        crate::operation::update_resource::builders::UpdateResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
